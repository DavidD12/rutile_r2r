pub use std::sync::{Arc, Mutex};

pub use super::*;
use futures::executor::{LocalPool, LocalSpawner};
use futures::task::LocalSpawnExt;
use futures::{StreamExt, future};

pub struct CoreNode {
    pub(crate) r2r_node_mutex: Arc<Mutex<r2r::Node>>,
    pub(crate) pool: LocalPool,
    pub(crate) spawner: Arc<LocalSpawner>,
}

impl CoreNode {
    pub fn create(name: &str, namespace: &str) -> Result<Self> {
        let ctx = r2r::Context::create()?;
        let node = r2r::Node::create(ctx, name, namespace)?;
        let node = Arc::new(Mutex::new(node));
        //
        let pool = LocalPool::new();
        let spawner = pool.spawner();
        let spawner = Arc::new(spawner);
        //
        let node = Self {
            r2r_node_mutex: node,
            pool,
            spawner,
        };
        Ok(node)
    }

    pub fn get_parameter<P>(&self, name: &str) -> r2r::Result<P>
    where
        r2r::ParameterValue: TryInto<P, Error = r2r::WrongParameterType>,
    {
        let node = self.r2r_node_mutex.lock().unwrap();
        node.get_parameter(name)
    }

    pub fn create_wall_timer<T>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        period: std::time::Duration,
        callback: fn(Arc<Mutex<r2r::Node>>, Arc<Mutex<T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: 'static,
    {
        let r2r_node_mutex = self.r2r_node_mutex.clone();
        let data_mutex = data_mutex.clone();
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

    pub fn create_subscription<T, M>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<r2r::Node>>, Arc<Mutex<T>>, &M) -> Result<()>,
    ) -> Result<()>
    where
        T: 'static,
        M: 'static + r2r::WrappedTypesupport,
    {
        let r2r_node_mutex = self.r2r_node_mutex.clone();
        let data_mutex = data_mutex.clone();
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

    pub fn create_service<T, S>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<r2r::Node>>, Arc<Mutex<T>>, &S::Request) -> Result<S::Response>,
    ) -> Result<()>
    where
        T: 'static,
        S: 'static + r2r::WrappedServiceTypeSupport,
    {
        let r2r_node_mutex = self.r2r_node_mutex.clone();
        let data_mutex = data_mutex.clone();
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

    pub fn create_client<T, S>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<r2r::Node>>, Arc<Mutex<T>>, &S::Response) -> Result<()>,
    ) -> Result<Client<T, S>>
    where
        T: 'static,
        S: 'static + r2r::WrappedServiceTypeSupport,
    {
        let mut rnode = self.r2r_node_mutex.lock().unwrap();

        let r2r_client = Arc::new(rnode.create_client::<S>(service_name, qos_profile)?);
        let client = Client::Defined {
            name: service_name.to_string(),
            r2r_node_mutex: self.r2r_node_mutex.clone(),
            r2r_client,
            callback: Arc::new(callback),
            data_mutex: data_mutex.clone(),
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
