use std::sync::Arc;

pub use crate::api::NodeMulti;
pub use crate::{MutexCreate, MutexLockErr, MutexLockOrLog, Result, SMutex};
use futures::StreamExt;
use futures::executor::ThreadPool;
use futures::task::SpawnExt;

pub struct Node {
    r2r_node: SMutex<r2r::Node>,
    pool: ThreadPool,
}

impl NodeMulti for Node {
    type Publisher<M: r2r::WrappedTypesupport> = crate::multi::Publisher<M>;
    type Client<S: r2r::WrappedServiceTypeSupport> = crate::multi::Client<S>;

    fn create(name: &str, namespace: &str) -> Result<Self> {
        let ctx = r2r::Context::create()?;
        let r2r_node = SMutex::create(r2r::Node::create(ctx, name, namespace)?);

        let pool = ThreadPool::new()?;

        let node = Self {
            r2r_node,
            pool,
        };
        Ok(node)
    }

    fn r2r(&self) -> SMutex<r2r::Node> {
        self.r2r_node.clone()
    }

    fn now(&self) -> std::time::Duration {
        let node = self.r2r_node.lock_or_log("r2r_node");
        let ros_clock = node.get_ros_clock();
        let mut clock = ros_clock.lock_or_log("clock");
        let now = clock.get_now().unwrap_or_else(|e| {
            eprintln!("[WARN] get_now() error: {e}");
            std::time::Duration::from_secs(0)
        });
        now
    }

    fn logger(&self) -> String {
        let node = self.r2r_node.lock_or_log("r2r_node");
        node.logger().to_string()
    }

    fn get_parameter<P>(&self, name: &str) -> crate::Result<P>
    where
        r2r::ParameterValue: TryInto<P, Error = r2r::WrongParameterType>,
    {
        let node = self.r2r_node.lock_err("r2r_node")?;
        match node.get_parameter(name) {
            Ok(value) => Ok(value),
            Err(e) => Err(e.into()),
        }
    }

    fn get_parameter_with_default<P>(&self, name: &str, default: P) -> crate::Result<P>
    where
        r2r::ParameterValue: TryInto<Option<P>, Error = r2r::WrongParameterType>,
    {
        let node = self.r2r_node.lock_err("r2r_node")?;
        let opt = node.get_parameter::<Option<P>>(name)?;
        Ok(opt.unwrap_or(default))
    }

    fn create_wall_timer_0<F>(&self, period: std::time::Duration, callback: F) -> Result<()>
    where
        F: Send + Sync + 'static + Fn(),
    {
        let logger = self.logger();
        let mut timer = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_wall_timer(period)?
        };

        self.pool.spawn(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => callback(),
                    Err(e) => {
                        r2r::log_error!(&logger, "timer execution error: {}", e)
                    }
                }
            }
        })?;

        Ok(())
    }

    fn create_wall_timer_1<T, F>(
        &self,
        period: std::time::Duration,
        callback: F,
        data: T,
    ) -> Result<()>
    where
        T: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T),
    {
        let logger = self.logger();
        let mut timer = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_wall_timer(period)?
        };

        self.pool.spawn(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => callback(data.clone()),
                    Err(e) => {
                        r2r::log_error!(&logger, "timer execution error: {}", e)
                    }
                }
            }
        })?;

        Ok(())
    }

    fn create_wall_timer_2<T1, T2, F>(
        &self,
        period: std::time::Duration,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> Result<()>
    where
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2),
    {
        let logger = self.logger();
        let mut timer = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_wall_timer(period)?
        };

        self.pool.spawn(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => callback(data_1.clone(), data_2.clone()),
                    Err(e) => {
                        r2r::log_error!(&logger, "timer execution error: {}", e)
                    }
                }
            }
        })?;

        Ok(())
    }

    fn create_wall_timer_3<T1, T2, T3, F>(
        &self,
        period: std::time::Duration,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
    ) -> Result<()>
    where
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        T3: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2, T3),
    {
        let logger = self.logger();
        let mut timer = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_wall_timer(period)?
        };

        self.pool.spawn(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => callback(data_1.clone(), data_2.clone(), data_3.clone()),
                    Err(e) => {
                        r2r::log_error!(&logger, "timer execution error: {}", e)
                    }
                }
            }
        })?;

        Ok(())
    }

    fn create_wall_timer_4<T1, T2, T3, T4, F>(
        &self,
        period: std::time::Duration,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
        data_4: T4,
    ) -> Result<()>
    where
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        T3: Clone + Send + Sync + 'static,
        T4: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2, T3, T4),
    {
        let logger = self.logger();
        let mut timer = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_wall_timer(period)?
        };

        self.pool.spawn(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => callback(
                        data_1.clone(),
                        data_2.clone(),
                        data_3.clone(),
                        data_4.clone(),
                    ),
                    Err(e) => {
                        r2r::log_error!(&logger, "timer execution error: {}", e)
                    }
                }
            }
        })?;

        Ok(())
    }

    fn create_wall_timer_5<T1, T2, T3, T4, T5, F>(
        &self,
        period: std::time::Duration,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
        data_4: T4,
        data_5: T5,
    ) -> Result<()>
    where
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        T3: Clone + Send + Sync + 'static,
        T4: Clone + Send + Sync + 'static,
        T5: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2, T3, T4, T5),
    {
        let logger = self.logger();
        let mut timer = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_wall_timer(period)?
        };

        self.pool.spawn(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => callback(
                        data_1.clone(),
                        data_2.clone(),
                        data_3.clone(),
                        data_4.clone(),
                        data_5.clone(),
                    ),
                    Err(e) => {
                        r2r::log_error!(&logger, "timer execution error: {}", e)
                    }
                }
            }
        })?;

        Ok(())
    }

    fn create_publisher<M>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
    ) -> Result<Self::Publisher<M>>
    where
        M: r2r::WrappedTypesupport,
    {
        let logger = self.logger();

        let r2r_publisher = {
            let mut r2r_node = self.r2r_node.lock_err("r2r_node")?;
            let r2_publisher = r2r_node.create_publisher(topic, qos_profile)?;
            SMutex::create(r2_publisher)
        };

        Ok(Self::Publisher::Defined {
            logger,
            r2r_publisher,
        })
    }

    fn create_subscription_0<M, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
    ) -> Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        F: Send + Sync + 'static + Fn(M),
    {
        let subscription = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.subscribe::<M>(topic, qos_profile)?
        };

        let callback = Arc::new(callback);
        self.pool.spawn(async move {
            subscription
                .for_each(move |msg| {
                    let callback = callback.clone();
                    async move {
                        (*callback)(msg);
                    }
                })
                .await;
        })?;
        Ok(())
    }

    fn create_subscription_1<M, T, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T, M),
    {
        let subscription = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.subscribe::<M>(topic, qos_profile)?
        };

        let callback = Arc::new(callback);
        self.pool.spawn(async move {
            subscription
                .for_each(move |msg| {
                    let callback = callback.clone();
                    let data = data.clone();
                    async move {
                        (*callback)(data, msg);
                    }
                })
                .await;
        })?;
        Ok(())
    }

    fn create_subscription_2<M, T1, T2, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2, M),
    {
        let subscription = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.subscribe::<M>(topic, qos_profile)?
        };

        let callback = Arc::new(callback);
        self.pool.spawn(async move {
            subscription
                .for_each(move |msg| {
                    let callback = callback.clone();
                    let data_1 = data_1.clone();
                    let data_2 = data_2.clone();
                    async move {
                        (*callback)(data_1, data_2, msg);
                    }
                })
                .await;
        })?;
        Ok(())
    }

    fn create_subscription_3<M, T1, T2, T3, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
    ) -> Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        T3: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2, T3, M),
    {
        let subscription = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.subscribe::<M>(topic, qos_profile)?
        };

        let callback = Arc::new(callback);
        self.pool.spawn(async move {
            subscription
                .for_each(move |msg| {
                    let callback = callback.clone();
                    let data_1 = data_1.clone();
                    let data_2 = data_2.clone();
                    let data_3 = data_3.clone();
                    async move {
                        (*callback)(data_1, data_2, data_3, msg);
                    }
                })
                .await;
        })?;
        Ok(())
    }

    fn create_subscription_4<M, T1, T2, T3, T4, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
        data_4: T4,
    ) -> Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        T3: Clone + Send + Sync + 'static,
        T4: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2, T3, T4, M),
    {
        let subscription = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.subscribe::<M>(topic, qos_profile)?
        };

        let callback = Arc::new(callback);
        self.pool.spawn(async move {
            subscription
                .for_each(move |msg| {
                    let callback = callback.clone();
                    let data_1 = data_1.clone();
                    let data_2 = data_2.clone();
                    let data_3 = data_3.clone();
                    let data_4 = data_4.clone();
                    async move {
                        (*callback)(data_1, data_2, data_3, data_4, msg);
                    }
                })
                .await;
        })?;
        Ok(())
    }

    fn create_subscription_5<M, T1, T2, T3, T4, T5, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
        data_4: T4,
        data_5: T5,
    ) -> Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        T3: Clone + Send + Sync + 'static,
        T4: Clone + Send + Sync + 'static,
        T5: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2, T3, T4, T5, M),
    {
        let subscription = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.subscribe::<M>(topic, qos_profile)?
        };

        let callback = Arc::new(callback);
        self.pool.spawn(async move {
            subscription
                .for_each(move |msg| {
                    let callback = callback.clone();
                    let data_1 = data_1.clone();
                    let data_2 = data_2.clone();
                    let data_3 = data_3.clone();
                    let data_4 = data_4.clone();
                    let data_5 = data_5.clone();
                    async move {
                        (*callback)(data_1, data_2, data_3, data_4, data_5, msg);
                    }
                })
                .await;
        })?;
        Ok(())
    }

    fn create_service_0<S, F>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
    ) -> Result<()>
    where
        S: 'static + Send + Sync + r2r::WrappedServiceTypeSupport,
        F: Send + Sync + 'static + Fn(S::Request) -> S::Response,
    {
        let mut service = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_service::<S>(service_name, qos_profile)?
        };

        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        self.pool.spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response = callback(request.message.clone());
                        if let Err(e) = request.respond(response) {
                            r2r::log_error!(
                                r2r_node_mutex
                                    .lock_or_log("r2r_node.create_service")
                                    .logger(),
                                "service response error (service_name='{}'): {}",
                                service_name,
                                e
                            );
                        }
                    }
                    None => break,
                }
            }
        })?;
        Ok(())
    }

    fn create_service_1<S, T, F>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> Result<()>
    where
        S: 'static + Send + Sync + r2r::WrappedServiceTypeSupport,
        T: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T, S::Request) -> S::Response,
    {
        let mut service = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_service::<S>(service_name, qos_profile)?
        };

        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        self.pool.spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response = callback(data.clone(), request.message.clone());
                        if let Err(e) = request.respond(response) {
                            r2r::log_error!(
                                r2r_node_mutex
                                    .lock_or_log("r2r_node.create_service")
                                    .logger(),
                                "service response error (service_name='{}'): {}",
                                service_name,
                                e
                            );
                        }
                    }
                    None => break,
                }
            }
        })?;
        Ok(())
    }

    fn create_service_2<S, T1, T2, F>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> Result<()>
    where
        S: 'static + Send + Sync + r2r::WrappedServiceTypeSupport,
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2, S::Request) -> S::Response,
    {
        let mut service = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_service::<S>(service_name, qos_profile)?
        };

        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        self.pool.spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response =
                            callback(data_1.clone(), data_2.clone(), request.message.clone());
                        if let Err(e) = request.respond(response) {
                            r2r::log_error!(
                                r2r_node_mutex
                                    .lock_or_log("r2r_node.create_service")
                                    .logger(),
                                "service response error (service_name='{}'): {}",
                                service_name,
                                e
                            );
                        }
                    }
                    None => break,
                }
            }
        })?;
        Ok(())
    }

    fn create_service_3<S, T1, T2, T3, F>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
    ) -> Result<()>
    where
        S: 'static + Send + Sync + r2r::WrappedServiceTypeSupport,
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        T3: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2, T3, S::Request) -> S::Response,
    {
        let mut service = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_service::<S>(service_name, qos_profile)?
        };

        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        self.pool.spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response = callback(
                            data_1.clone(),
                            data_2.clone(),
                            data_3.clone(),
                            request.message.clone(),
                        );
                        if let Err(e) = request.respond(response) {
                            r2r::log_error!(
                                r2r_node_mutex
                                    .lock_or_log("r2r_node.create_service")
                                    .logger(),
                                "service response error (service_name='{}'): {}",
                                service_name,
                                e
                            );
                        }
                    }
                    None => break,
                }
            }
        })?;
        Ok(())
    }

    fn create_service_4<S, T1, T2, T3, T4, F>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
        data_4: T4,
    ) -> Result<()>
    where
        S: 'static + Send + Sync + r2r::WrappedServiceTypeSupport,
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        T3: Clone + Send + Sync + 'static,
        T4: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2, T3, T4, S::Request) -> S::Response,
    {
        let mut service = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_service::<S>(service_name, qos_profile)?
        };

        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        self.pool.spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response = callback(
                            data_1.clone(),
                            data_2.clone(),
                            data_3.clone(),
                            data_4.clone(),
                            request.message.clone(),
                        );
                        if let Err(e) = request.respond(response) {
                            r2r::log_error!(
                                r2r_node_mutex
                                    .lock_or_log("r2r_node.create_service")
                                    .logger(),
                                "service response error (service_name='{}'): {}",
                                service_name,
                                e
                            );
                        }
                    }
                    None => break,
                }
            }
        })?;
        Ok(())
    }

    fn create_service_5<S, T1, T2, T3, T4, T5, F>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
        data_4: T4,
        data_5: T5,
    ) -> Result<()>
    where
        S: 'static + Send + Sync + r2r::WrappedServiceTypeSupport,
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        T3: Clone + Send + Sync + 'static,
        T4: Clone + Send + Sync + 'static,
        T5: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static + Fn(T1, T2, T3, T4, T5, S::Request) -> S::Response,
    {
        let mut service = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_service::<S>(service_name, qos_profile)?
        };

        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        self.pool.spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response = callback(
                            data_1.clone(),
                            data_2.clone(),
                            data_3.clone(),
                            data_4.clone(),
                            data_5.clone(),
                            request.message.clone(),
                        );
                        if let Err(e) = request.respond(response) {
                            r2r::log_error!(
                                r2r_node_mutex
                                    .lock_or_log("r2r_node.create_service")
                                    .logger(),
                                "service response error (service_name='{}'): {}",
                                service_name,
                                e
                            );
                        }
                    }
                    None => break,
                }
            }
        })?;
        Ok(())
    }

    fn create_client<S>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
    ) -> Result<Self::Client<S>>
    where
        S: 'static + Send + Sync + r2r::WrappedServiceTypeSupport,
        S::Request: Send + Sync + 'static,
        S::Response: Send + 'static,
    {
        let r2r_client = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_client::<S>(service_name, qos_profile)?
        };

        Ok(Self::Client::Defined {
            r2r_client: Arc::new(r2r_client),
            r2r_node: self.r2r_node.clone(),
            pool: self.pool.clone(),
        })
    }

    fn spin(&mut self, timeout: std::time::Duration) {
        loop {
            {
                let mut node = self.r2r_node.lock_or_log("r2r_node");
                node.spin_once(timeout);
            }
        }
    }
}
