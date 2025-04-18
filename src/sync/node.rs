use std::sync::{Arc, Mutex};

use super::*;
use crate::Result;
use futures::executor::{LocalPool, LocalSpawner};
use futures::task::LocalSpawnExt;
use futures::{StreamExt, future};

pub struct Node<T>
where
    T: Data,
{
    pub data_mutex: Arc<Mutex<T>>,
    pub(crate) r2r_node_mutex: Arc<Mutex<r2r::Node>>,
    pub(crate) pool: LocalPool,
    pub(crate) spawner: Arc<LocalSpawner>,
}

impl<T> Node<T>
where
    T: Data,
{
    pub fn create(name: &str, namespace: &str) -> Result<Self> {
        let data = T::default();
        let data_mutex = Arc::new(Mutex::new(data));
        //
        let ctx = r2r::Context::create()?;
        let node = r2r::Node::create(ctx, name, namespace)?;
        let node = Arc::new(Mutex::new(node));
        //
        let pool = LocalPool::new();
        let spawner = pool.spawner();
        let spawner = Arc::new(spawner);
        //
        let node = Self {
            data_mutex: data_mutex.clone(),
            r2r_node_mutex: node,
            pool,
            spawner,
        };
        //
        let mut data = data_mutex.lock().unwrap();
        data.initialize(&node)?;

        Ok(node)
    }

    pub fn get_parameter<P>(&self, name: &str) -> r2r::Result<P>
    where
        r2r::ParameterValue: TryInto<P, Error = r2r::WrongParameterType>,
    {
        let node = self.r2r_node_mutex.lock().unwrap();
        node.get_parameter::<P>(name)
    }

    pub fn get_parameter_with_default<P>(&self, name: &str, default: P) -> r2r::Result<P>
    where
        r2r::ParameterValue: TryInto<Option<P>, Error = r2r::WrongParameterType>,
    {
        let node = self.r2r_node_mutex.lock().unwrap();
        let opt: Option<P> = node.get_parameter::<Option<P>>(name)?;

        Ok(opt.unwrap_or(default))
    }

    pub fn create_wall_timer(
        &self,
        period: std::time::Duration,
        callback: fn(Arc<Mutex<r2r::Node>>, Arc<Mutex<T>>) -> Result<()>,
    ) -> Result<()> {
        let r2r_node_mutex = self.r2r_node_mutex.clone();
        let data_mutex = self.data_mutex.clone();
        //
        let mut timer = {
            let mut node = r2r_node_mutex.lock().unwrap();
            node.create_wall_timer(period)?
        };
        //
        self.spawner.spawn_local(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => {
                        if let Err(e) = callback(r2r_node_mutex.clone(), data_mutex.clone()) {
                            let node = r2r_node_mutex.lock().unwrap();
                            r2r::log_error!(node.logger(), "wall_timer callback error: {}", e);
                        }
                    }

                    Err(e) => {
                        let node = r2r_node_mutex.lock().unwrap();
                        r2r::log_error!(node.logger(), "timer execution error: {}", e)
                    }
                }
            }
        })?;

        Ok(())
    }

    pub fn create_publisher<M>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
    ) -> Result<Publisher<M>>
    where
        M: r2r::WrappedTypesupport,
    {
        let mut r2r_node = self.r2r_node_mutex.lock().unwrap();
        let r2r_publisher = r2r_node.create_publisher(topic, qos_profile)?;
        Ok(Publisher::Defined { r2r_publisher })
    }

    pub fn create_subscription<M>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<r2r::Node>>, Arc<Mutex<T>>, &M) -> Result<()>,
    ) -> Result<()>
    where
        M: 'static + r2r::WrappedTypesupport,
    {
        let r2r_node_mutex = self.r2r_node_mutex.clone();
        let data_mutex = self.data_mutex.clone();
        //
        let subscription = {
            let mut node = r2r_node_mutex.lock().unwrap();
            node.subscribe::<M>(topic, qos_profile)?
        };
        //
        let topic = topic.to_string();
        self.spawner.spawn_local(async move {
            subscription
                .for_each(|msg| {
                    if let Err(e) = callback(r2r_node_mutex.clone(), data_mutex.clone(), &msg) {
                        let node = r2r_node_mutex.lock().unwrap();
                        r2r::log_error!(
                            node.logger(),
                            "subscription callback error (topic='{}'): {}",
                            topic,
                            e
                        );
                    };
                    future::ready(())
                })
                .await
        })?;

        Ok(())
    }

    pub fn create_service<S>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<r2r::Node>>, Arc<Mutex<T>>, &S::Request) -> Result<S::Response>,
    ) -> Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
    {
        let r2r_node_mutex = self.r2r_node_mutex.clone();
        let data_mutex = self.data_mutex.clone();
        //
        let mut service = {
            let mut node = r2r_node_mutex.lock().unwrap();
            node.create_service::<S>(service_name, qos_profile)?
        };
        //
        let service_name = service_name.to_string();
        self.spawner.spawn_local(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        match callback(r2r_node_mutex.clone(), data_mutex.clone(), &request.message)
                        {
                            Ok(response) => {
                                if let Err(e) = request.respond(response) {
                                    let node = r2r_node_mutex.lock().unwrap();
                                    r2r::log_error!(
                                        node.logger(),
                                        "service response error (service_name='{}'): {}",
                                        service_name,
                                        e
                                    );
                                }
                            }
                            Err(e) => {
                                let node = r2r_node_mutex.lock().unwrap();
                                r2r::log_error!(
                                    node.logger(),
                                    "service callback error (service_name='{}'): {}",
                                    service_name,
                                    e
                                );
                            }
                        }
                    }
                    None => break,
                }
            }
        })?;
        //
        Ok(())
    }

    // pub fn create_client_sync<S>(
    //     &self,
    //     service_name: &str,
    //     qos_profile: r2r::QosProfile,
    // ) -> Result<ClientSync<S>>
    // where
    //     S: 'static + r2r::WrappedServiceTypeSupport,
    // {
    //     let mut rnode = self.r2r_node_mutex.lock().unwrap();

    //     let r2r_client = Arc::new(rnode.create_client::<S>(service_name, qos_profile)?);
    //     let client = ClientSync::Defined {
    //         name: service_name.to_string(),
    //         r2r_client,
    //         spawner: self.spawner.clone(),
    //     };
    //     Ok(client)
    // }

    pub fn create_client<S>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<r2r::Node>>, Arc<Mutex<T>>, &S::Response) -> Result<()>,
    ) -> Result<ClientAsync<T, S>>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
    {
        let mut rnode = self.r2r_node_mutex.lock().unwrap();

        let r2r_client = Arc::new(rnode.create_client::<S>(service_name, qos_profile)?);
        let client = ClientAsync::Defined {
            name: service_name.to_string(),
            r2r_node_mutex: self.r2r_node_mutex.clone(),
            r2r_client,
            callback: Arc::new(callback),
            data_mutex: self.data_mutex.clone(),
            spawner: self.spawner.clone(),
        };
        Ok(client)
    }

    pub fn spin(&mut self) {
        loop {
            {
                let mut node = self.r2r_node_mutex.lock().unwrap();
                node.spin_once(std::time::Duration::from_millis(10_000));
            }
            self.pool.run_until_stalled();
        }
    }
}
