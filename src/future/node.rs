use std::sync::Arc;

pub use crate::node_api::NodeApi;
pub use crate::{MutexCreate, MutexLockErr, MutexLockOrLog, Result, SMutex};
use futures::{StreamExt, executor::ThreadPool, task::SpawnExt};

pub struct Node {
    r2r_node: SMutex<r2r::Node>,
    pool: ThreadPool,
}

impl NodeApi for Node {
    type Publisher<M: r2r::WrappedTypesupport> = crate::future::Publisher<M>;
    type Client<S: r2r::WrappedServiceTypeSupport> = crate::future::Client<S>;

    //-------------------------------------------------- Create --------------------------------------------------

    fn create(name: &str, namespace: &str) -> Result<Self> {
        let ctx = r2r::Context::create()?;
        let r2r_node = SMutex::create(r2r::Node::create(ctx, name, namespace)?);
        let pool = ThreadPool::new()?;
        //
        let node = Self { r2r_node, pool };
        Ok(node)
    }

    //-------------------------------------------------- R2R --------------------------------------------------

    fn r2r(&self) -> SMutex<r2r::Node> {
        self.r2r_node.clone()
    }

    //-------------------------------------------------- Now --------------------------------------------------

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

    //-------------------------------------------------- Logger --------------------------------------------------

    fn logger(&self) -> String {
        let node = self.r2r_node.lock_or_log("r2r_node");
        node.logger().to_string()
    }

    //-------------------------------------------------- Parameter --------------------------------------------------

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

    //-------------------------------------------------- Timer --------------------------------------------------

    fn create_wall_timer_0<F, R>(&self, period: std::time::Duration, callback: F) -> Result<()>
    where
        F: Send + 'static,
        F: Fn() -> R,
        R: Future<Output = ()>,
        R: Send,
    {
        let logger = self.logger();
        let mut timer = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_wall_timer(period)?
        };

        self.pool.spawn(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => {
                        callback().await;
                    }
                    Err(e) => {
                        r2r::log_error!(&logger, "timer execution error: {}", e)
                    }
                }
            }
        })?;

        Ok(())
    }

    fn create_wall_timer_1<T, F, R>(
        &self,
        period: std::time::Duration,
        callback: F,
        data: T,
    ) -> Result<()>
    where
        T: Clone + Send + 'static,
        F: Send + 'static,
        F: Fn(T) -> R,
        R: Future<Output = ()>,
        R: Send,
    {
        let logger = self.logger();
        let mut timer = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_wall_timer(period)?
        };

        self.pool.spawn(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => {
                        callback(data.clone()).await;
                    }
                    Err(e) => {
                        r2r::log_error!(&logger, "timer execution error: {}", e)
                    }
                }
            }
        })?;

        Ok(())
    }

    fn create_wall_timer_2<T1, T2, F, R>(
        &self,
        period: std::time::Duration,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> Result<()>
    where
        T1: Clone + Send + 'static,
        T2: Clone + Send + 'static,
        F: Send + 'static,
        F: Fn(T1, T2) -> R,
        R: Future<Output = ()>,
        R: Send,
    {
        let logger = self.logger();
        let mut timer = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            node.create_wall_timer(period)?
        };

        self.pool.spawn(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => {
                        callback(data_1.clone(), data_2.clone()).await;
                    }
                    Err(e) => {
                        r2r::log_error!(&logger, "timer execution error: {}", e)
                    }
                }
            }
        })?;

        Ok(())
    }

    //-------------------------------------------------- Publisher --------------------------------------------------

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

    //-------------------------------------------------- Subscriber --------------------------------------------------

    fn create_subscription_0<M, F, R>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
    ) -> Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        F: Send + Sync + 'static,
        F: Fn(M) -> R,
        R: Future<Output = ()>,
        R: Send,
    {
        let subscription = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            let subscription = node.subscribe::<M>(topic, qos_profile)?;
            subscription
        };

        self.pool
            .spawn(async move { subscription.for_each(|msg| callback(msg)).await })?;
        Ok(())
    }

    fn create_subscription_1<M, T, F, R>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static,
        F: Fn(T, M) -> R,
        R: Future<Output = ()>,
        R: Send,
    {
        let subscription = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            let subscription = node.subscribe::<M>(topic, qos_profile)?;
            subscription
        };

        self.pool.spawn(async move {
            subscription
                .for_each(|msg| callback(data.clone(), msg))
                .await
        })?;
        Ok(())
    }

    fn create_subscription_2<M, T1, T2, F, R>(
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
        F: Send + Sync + 'static,
        F: Fn(T1, T2, M) -> R,
        R: Future<Output = ()>,
        R: Send,
    {
        let subscription = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            let subscription = node.subscribe::<M>(topic, qos_profile)?;
            subscription
        };

        self.pool.spawn(async move {
            subscription
                .for_each(|msg| callback(data_1.clone(), data_2.clone(), msg))
                .await
        })?;
        Ok(())
    }

    //-------------------------------------------------- Service --------------------------------------------------

    fn create_service_0<S, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
    ) -> Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        F: Send + 'static,
        F: Fn(S::Request) -> R,
        R: Future<Output = S::Response>,
        R: Send,
    {
        let mut service = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            let service = node.create_service::<S>(service_name, qos_profile)?;
            service
        };

        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        //
        self.pool.spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response = callback(request.message.clone()).await;
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
        //
        Ok(())
    }

    fn create_service_1<S, T, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        F: Send + 'static,
        F: Fn(T, S::Request) -> R,
        R: Future<Output = S::Response>,
        R: Send,
        T: Clone + Send + 'static,
    {
        let mut service = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            let service = node.create_service::<S>(service_name, qos_profile)?;
            service
        };

        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        //
        self.pool.spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response = callback(data.clone(), request.message.clone()).await;
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
        //
        Ok(())
    }

    fn create_service_2<S, T1, T2, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        F: Send + 'static,
        F: Fn(T1, T2, S::Request) -> R,
        R: Future<Output = S::Response>,
        R: Send,
        T1: Clone + Send + 'static,
        T2: Clone + Send + 'static,
    {
        let mut service = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            let service = node.create_service::<S>(service_name, qos_profile)?;
            service
        };

        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        //
        self.pool.spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response =
                            callback(data_1.clone(), data_2.clone(), request.message.clone()).await;
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
        //
        Ok(())
    }

    //-------------------------------------------------- Client --------------------------------------------------

    fn create_client<S>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
    ) -> Result<Self::Client<S>>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
    {
        let r2r_client = {
            let mut node = self.r2r_node.lock_err("r2r_node")?;
            let r2r_client = node.create_client::<S>(service_name, qos_profile)?;
            r2r_client
        };

        let client = Self::Client::Defined {
            r2r_client: Arc::new(r2r_client),
        };
        Ok(client)
    }

    //-------------------------------------------------- Spin --------------------------------------------------

    fn spin(&mut self) {
        loop {
            {
                let mut node = self.r2r_node.lock_or_log("r2r_node");
                node.spin_once(std::time::Duration::from_millis(10_000));
            }
        }
    }
}
