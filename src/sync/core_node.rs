use std::rc::Rc;
pub use std::sync::{Arc, Mutex};

pub use super::*;
use futures::executor::{LocalPool, LocalSpawner};
use futures::task::LocalSpawnExt;
use futures::{StreamExt, future};

pub struct CoreNode {
    pub(crate) r2r_node: r2r::Node,
    pub(crate) pool: LocalPool,
    pub(crate) spawner: Rc<LocalSpawner>,
}

impl CoreNode {
    pub fn create(name: &str, namespace: &str) -> Result<Self> {
        let ctx = r2r::Context::create()?;
        let r2r_node = r2r::Node::create(ctx, name, namespace)?;
        //
        let pool = LocalPool::new();
        let spawner = pool.spawner();
        let spawner = Rc::new(spawner);
        //
        let node = Self {
            r2r_node,
            pool,
            spawner,
        };
        Ok(node)
    }

    pub fn r2r_node(&self) -> &r2r::Node {
        &self.r2r_node
    }

    pub fn get_parameter<P>(self_mutex: Arc<Mutex<Self>>, name: &str) -> r2r::Result<P>
    where
        r2r::ParameterValue: TryInto<P, Error = r2r::WrongParameterType>,
    {
        let this = self_mutex.lock().unwrap();
        this.r2r_node.get_parameter(name)
    }

    pub fn get_parameter_with_default<P>(
        self_mutex: Arc<Mutex<Self>>,
        name: &str,
        default: P,
    ) -> r2r::Result<P>
    where
        r2r::ParameterValue: TryInto<Option<P>, Error = r2r::WrongParameterType>,
    {
        let this = self_mutex.lock().unwrap();
        let opt: Option<P> = this.r2r_node.get_parameter::<Option<P>>(name)?;

        Ok(opt.unwrap_or(default))
    }

    pub fn create_wall_timer<T>(
        self_mutex: Arc<Mutex<Self>>,
        data_mutex: Arc<Mutex<T>>,
        period: std::time::Duration,
        callback: fn(Arc<Mutex<Self>>, Arc<Mutex<T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: 'static,
    {
        let self_mutex = self_mutex.clone();
        let data_mutex = data_mutex.clone();
        //
        let mut timer = {
            let mut this = self_mutex.lock().unwrap();
            this.r2r_node.create_wall_timer(period)?
        };
        let spawner = {
            let this = self_mutex.lock().unwrap();
            this.spawner.clone()
        };
        //
        spawner.spawn_local(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => {
                        if let Err(e) = callback(self_mutex.clone(), data_mutex.clone()) {
                            let this = self_mutex.lock().unwrap();
                            r2r::log_error!(
                                this.r2r_node.logger(),
                                "wall_timer callback error: {}",
                                e
                            );
                        }
                    }

                    Err(e) => {
                        let this = self_mutex.lock().unwrap();
                        r2r::log_error!(this.r2r_node.logger(), "timer execution error: {}", e)
                    }
                }
            }
        })?;

        Ok(())
    }

    pub fn create_publisher<M>(
        self_mutex: Arc<Mutex<Self>>,
        topic: &str,
        qos_profile: r2r::QosProfile,
    ) -> Result<Publisher<M>>
    where
        M: r2r::WrappedTypesupport,
    {
        let mut this = self_mutex.lock().unwrap();
        let r2r_publisher = this.r2r_node.create_publisher(topic, qos_profile)?;
        Ok(Publisher::Defined { r2r_publisher })
    }

    pub fn create_subscription<T, M>(
        self_mutex: Arc<Mutex<Self>>,
        data_mutex: Arc<Mutex<T>>,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<Self>>, Arc<Mutex<T>>, &M) -> Result<()>,
    ) -> Result<()>
    where
        T: 'static,
        M: 'static + r2r::WrappedTypesupport,
    {
        let self_mutex = self_mutex.clone();
        let data_mutex = data_mutex.clone();
        //
        let spawner = {
            let this = self_mutex.lock().unwrap();
            this.spawner.clone()
        };
        //
        let subscription = {
            let mut this = self_mutex.lock().unwrap();
            this.r2r_node.subscribe::<M>(topic, qos_profile)?
        };
        //
        let topic = topic.to_string();
        spawner.spawn_local(async move {
            subscription
                .for_each(|msg| {
                    if let Err(e) = callback(self_mutex.clone(), data_mutex.clone(), &msg) {
                        let this = self_mutex.lock().unwrap();
                        r2r::log_error!(
                            this.r2r_node.logger(),
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
        self_mutex: Arc<Mutex<Self>>,
        data_mutex: Arc<Mutex<T>>,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<Self>>, Arc<Mutex<T>>, &S::Request) -> Result<S::Response>,
    ) -> Result<()>
    where
        T: 'static,
        S: 'static + r2r::WrappedServiceTypeSupport,
    {
        let self_mutex = self_mutex.clone();
        let data_mutex = data_mutex.clone();
        //
        let spawner = {
            let this = self_mutex.lock().unwrap();
            this.spawner.clone()
        };
        //
        let mut service = {
            let mut this = self_mutex.lock().unwrap();
            this.r2r_node
                .create_service::<S>(service_name, qos_profile)?
        };
        //
        let service_name = service_name.to_string();
        spawner.spawn_local(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        match callback(self_mutex.clone(), data_mutex.clone(), &request.message) {
                            Ok(response) => {
                                if let Err(e) = request.respond(response) {
                                    let this = self_mutex.lock().unwrap();
                                    r2r::log_error!(
                                        this.r2r_node.logger(),
                                        "service response error (service_name='{}'): {}",
                                        service_name,
                                        e
                                    );
                                }
                            }
                            Err(e) => {
                                let this = self_mutex.lock().unwrap();
                                r2r::log_error!(
                                    this.r2r_node.logger(),
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

    pub fn create_client<T, S>(
        self_mutex: Arc<Mutex<Self>>,
        data_mutex: Arc<Mutex<T>>,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<Self>>, Arc<Mutex<T>>, &S::Response) -> Result<()>,
    ) -> Result<Client<T, S>>
    where
        T: 'static,
        S: 'static + r2r::WrappedServiceTypeSupport,
    {
        let r2r_client = {
            let mut this = self_mutex.lock().unwrap();
            Arc::new(
                this.r2r_node
                    .create_client::<S>(service_name, qos_profile)?,
            )
        };
        let client = Client::Defined {
            name: service_name.to_string(),
            node_mutex: self_mutex.clone(),
            r2r_client,
            callback: Arc::new(callback),
            data_mutex: data_mutex.clone(),
        };
        Ok(client)
    }

    pub fn spin(self_mutex: Arc<Mutex<Self>>) {
        loop {
            {
                let mut this = self_mutex.lock().unwrap();
                this.r2r_node
                    .spin_once(std::time::Duration::from_millis(10_000));
            }
            // self.pool.run_until_stalled();
            todo!()
        }
    }
}
