use super::{Client, NodeInterface, Publisher};
pub use crate::Result;
pub use std::sync::{Arc, Mutex};

use futures::{StreamExt, executor::LocalSpawner, future, task::LocalSpawnExt};

pub struct Core {
    pub(crate) r2r_node: r2r::Node,
    pub(crate) spawner: LocalSpawner,
}

impl Core {
    pub fn now(self_mutex: Arc<Mutex<Self>>) -> Result<std::time::Duration> {
        let this = self_mutex.lock().unwrap();
        let mutex = this.r2r_node.get_ros_clock();
        let mut clock = mutex.lock().unwrap();
        let t = clock.get_now()?;
        Ok(t)
    }

    pub fn logger(self_mutex: Arc<Mutex<Self>>) -> String {
        let this = self_mutex.lock().unwrap();
        this.r2r_node.logger().to_string()
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
        let publisher = Publisher::Defined { r2r_publisher };
        Ok(publisher)
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
            core_mutex: self_mutex.clone(),
            r2r_client,
            callback: Arc::new(callback),
            data_mutex: data_mutex.clone(),
        };
        Ok(client)
    }
}

impl NodeInterface for Arc<Mutex<Core>> {
    fn now(&self) -> Result<std::time::Duration> {
        Core::now(self.clone())
    }

    fn logger(&self) -> String {
        Core::logger(self.clone())
    }

    fn get_parameter<P>(&self, name: &str) -> r2r::Result<P>
    where
        r2r::ParameterValue: TryInto<P, Error = r2r::WrongParameterType>,
    {
        Core::get_parameter(self.clone(), name)
    }

    fn create_wall_timer<T>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        period: std::time::Duration,
        callback: fn(Arc<Mutex<Core>>, Arc<Mutex<T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: 'static,
    {
        Core::create_wall_timer(self.clone(), data_mutex, period, callback)
    }

    fn create_publisher<M>(&self, topic: &str, qos_profile: r2r::QosProfile) -> Result<Publisher<M>>
    where
        M: r2r::WrappedTypesupport,
    {
        Core::create_publisher(self.clone(), topic, qos_profile)
    }

    fn create_subscription<T, M>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<Core>>, Arc<Mutex<T>>, &M) -> Result<()>,
    ) -> Result<()>
    where
        T: 'static,
        M: 'static + r2r::WrappedTypesupport,
    {
        Core::create_subscription(self.clone(), data_mutex, topic, qos_profile, callback)
    }

    fn create_service<T, S>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<Core>>, Arc<Mutex<T>>, &S::Request) -> Result<S::Response>,
    ) -> Result<()>
    where
        T: 'static,
        S: 'static + r2r::WrappedServiceTypeSupport,
    {
        Core::create_service::<T, S>(
            self.clone(),
            data_mutex,
            service_name,
            qos_profile,
            callback,
        )
    }

    fn create_client<T, S>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<Core>>, Arc<Mutex<T>>, &S::Response) -> Result<()>,
    ) -> Result<Client<T, S>>
    where
        T: 'static,
        S: 'static + r2r::WrappedServiceTypeSupport,
    {
        Core::create_client(
            self.clone(),
            data_mutex,
            service_name,
            qos_profile,
            callback,
        )
    }
}
