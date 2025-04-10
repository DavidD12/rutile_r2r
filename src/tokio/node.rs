use super::*;
use futures::StreamExt;
use tokio::task;

pub struct Node {
    r2r_node: SMutex<r2r::Node>,
}

impl Node {
    pub fn create(name: &str, namespace: &str) -> Result<Self> {
        let ctx = r2r::Context::create()?;
        let r2r_node = SMutex::create(r2r::Node::create(ctx, name, namespace)?);
        //
        let node = Self { r2r_node };
        Ok(node)
    }

    //-------------------------------------------------- R2R --------------------------------------------------

    pub fn r2r(&self) -> SMutex<r2r::Node> {
        self.r2r_node.clone()
    }

    //-------------------------------------------------- Now --------------------------------------------------

    pub fn now(&self) -> crate::Result<std::time::Duration> {
        let clock = self.r2r_node.lock().unwrap().get_ros_clock();
        let t = clock.lock().unwrap().get_now()?;
        Ok(t)
    }

    //-------------------------------------------------- Logger --------------------------------------------------

    pub fn logger(&self) -> String {
        self.r2r_node.lock().unwrap().logger().to_string()
    }

    //-------------------------------------------------- Parameter --------------------------------------------------

    pub fn get_parameter<P>(&self, name: &str) -> r2r::Result<P>
    where
        r2r::ParameterValue: TryInto<P, Error = r2r::WrongParameterType>,
    {
        self.r2r_node.lock().unwrap().get_parameter(name)
    }

    pub fn get_parameter_with_default<P>(&self, name: &str, default: P) -> r2r::Result<P>
    where
        r2r::ParameterValue: TryInto<Option<P>, Error = r2r::WrongParameterType>,
    {
        let opt = self
            .r2r_node
            .lock()
            .unwrap()
            .get_parameter::<Option<P>>(name)?;

        Ok(opt.unwrap_or(default))
    }

    //-------------------------------------------------- Timer --------------------------------------------------

    pub fn create_wall_timer<F, R>(&self, period: std::time::Duration, callback: F) -> Result<()>
    where
        F: Send + 'static,
        F: Fn() -> R,
        R: Future<Output = ()>,
        R: Send,
    {
        let logger = self.logger();
        let mut timer = self.r2r_node.lock().unwrap().create_wall_timer(period)?;

        task::spawn(async move {
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
        });

        Ok(())
    }

    pub fn create_wall_timer_1<T, F, R>(
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
        let mut timer = self.r2r_node.lock().unwrap().create_wall_timer(period)?;

        task::spawn(async move {
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
        });

        Ok(())
    }

    pub fn create_wall_timer_2<T1, T2, F, R>(
        &self,
        period: std::time::Duration,
        callback: F,
        data1: T1,
        data2: T2,
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
        let mut timer = self.r2r_node.lock().unwrap().create_wall_timer(period)?;

        task::spawn(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => {
                        callback(data1.clone(), data2.clone()).await;
                    }
                    Err(e) => {
                        r2r::log_error!(&logger, "timer execution error: {}", e)
                    }
                }
            }
        });

        Ok(())
    }

    pub fn create_wall_timer_3<T1, T2, T3, F, R>(
        &self,
        period: std::time::Duration,
        callback: F,
        data1: T1,
        data2: T2,
        data3: T3,
    ) -> Result<()>
    where
        T1: Clone + Send + 'static,
        T2: Clone + Send + 'static,
        T3: Clone + Send + 'static,
        F: Send + 'static,
        F: Fn(T1, T2, T3) -> R,
        R: Future<Output = ()>,
        R: Send,
    {
        let logger = self.logger();
        let mut timer = self.r2r_node.lock().unwrap().create_wall_timer(period)?;

        task::spawn(async move {
            loop {
                match timer.tick().await {
                    Ok(_) => {
                        callback(data1.clone(), data2.clone(), data3.clone()).await;
                    }
                    Err(e) => {
                        r2r::log_error!(&logger, "timer execution error: {}", e)
                    }
                }
            }
        });

        Ok(())
    }

    //-------------------------------------------------- Publisher --------------------------------------------------

    pub fn create_publisher<M>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
    ) -> Result<Publisher<M>>
    where
        M: r2r::WrappedTypesupport,
    {
        let logger = self.logger();
        let mut r2r_node = self.r2r_node.lock().unwrap();
        let r2r_publisher = SMutex::create(r2r_node.create_publisher(topic, qos_profile)?);
        Ok(Publisher::Defined {
            logger,
            r2r_publisher,
        })
    }

    //-------------------------------------------------- Subscriber --------------------------------------------------

    pub fn create_subscription<M, F, R>(
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
        let subscription = self
            .r2r_node
            .lock()
            .unwrap()
            .subscribe::<M>(topic, qos_profile)?;
        task::spawn(async move { subscription.for_each(|msg| callback(msg)).await });
        Ok(())
    }

    pub fn create_subscription_1<M, T, F, R>(
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
        let subscription = self
            .r2r_node
            .lock()
            .unwrap()
            .subscribe::<M>(topic, qos_profile)?;
        task::spawn(async move {
            subscription
                .for_each(|msg| callback(data.clone(), msg))
                .await
        });
        Ok(())
    }

    pub fn create_subscription_2<M, T1, T2, F, R>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data1: T1,
        data2: T2,
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
        let subscription = self
            .r2r_node
            .lock()
            .unwrap()
            .subscribe::<M>(topic, qos_profile)?;
        task::spawn(async move {
            subscription
                .for_each(|msg| callback(data1.clone(), data2.clone(), msg))
                .await
        });
        Ok(())
    }

    pub fn create_subscription_3<M, T1, T2, T3, F, R>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data1: T1,
        data2: T2,
        data3: T3,
    ) -> Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        T3: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static,
        F: Fn(T1, T2, T3, M) -> R,
        R: Future<Output = ()>,
        R: Send,
    {
        let subscription = self
            .r2r_node
            .lock()
            .unwrap()
            .subscribe::<M>(topic, qos_profile)?;
        task::spawn(async move {
            subscription
                .for_each(|msg| callback(data1.clone(), data2.clone(), data3.clone(), msg))
                .await
        });
        Ok(())
    }

    //-------------------------------------------------- Service --------------------------------------------------

    pub fn create_service<S, F, R>(
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
        let mut service = self
            .r2r_node
            .lock()
            .unwrap()
            .create_service::<S>(service_name, qos_profile)?;
        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        //
        task::spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response = callback(request.message.clone()).await;
                        if let Err(e) = request.respond(response) {
                            r2r::log_error!(
                                r2r_node_mutex.lock().unwrap().logger(),
                                "service response error (service_name='{}'): {}",
                                service_name,
                                e
                            );
                        }
                    }
                    None => break,
                }
            }
        });
        //
        Ok(())
    }

    pub fn create_service_1<S, T, F, R>(
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
        let mut service = self
            .r2r_node
            .lock()
            .unwrap()
            .create_service::<S>(service_name, qos_profile)?;
        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        //
        task::spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response = callback(data.clone(), request.message.clone()).await;
                        if let Err(e) = request.respond(response) {
                            r2r::log_error!(
                                r2r_node_mutex.lock().unwrap().logger(),
                                "service response error (service_name='{}'): {}",
                                service_name,
                                e
                            );
                        }
                    }
                    None => break,
                }
            }
        });
        //
        Ok(())
    }

    pub fn create_service_2<S, T1, T2, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data1: T1,
        data2: T2,
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
        let mut service = self
            .r2r_node
            .lock()
            .unwrap()
            .create_service::<S>(service_name, qos_profile)?;
        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        //
        task::spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response =
                            callback(data1.clone(), data2.clone(), request.message.clone()).await;
                        if let Err(e) = request.respond(response) {
                            r2r::log_error!(
                                r2r_node_mutex.lock().unwrap().logger(),
                                "service response error (service_name='{}'): {}",
                                service_name,
                                e
                            );
                        }
                    }
                    None => break,
                }
            }
        });
        //
        Ok(())
    }

    pub fn create_service_3<S, T1, T2, T3, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data1: T1,
        data2: T2,
        data3: T3,
    ) -> Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        F: Send + 'static,
        F: Fn(T1, T2, T3, S::Request) -> R,
        R: Future<Output = S::Response>,
        R: Send,
        T1: Clone + Send + 'static,
        T2: Clone + Send + 'static,
        T3: Clone + Send + 'static,
    {
        let mut service = self
            .r2r_node
            .lock()
            .unwrap()
            .create_service::<S>(service_name, qos_profile)?;
        let r2r_node_mutex = self.r2r_node.clone();
        let service_name = service_name.to_string();
        //
        task::spawn(async move {
            loop {
                match service.next().await {
                    Some(request) => {
                        let response = callback(
                            data1.clone(),
                            data2.clone(),
                            data3.clone(),
                            request.message.clone(),
                        )
                        .await;
                        if let Err(e) = request.respond(response) {
                            r2r::log_error!(
                                r2r_node_mutex.lock().unwrap().logger(),
                                "service response error (service_name='{}'): {}",
                                service_name,
                                e
                            );
                        }
                    }
                    None => break,
                }
            }
        });
        //
        Ok(())
    }

    //-------------------------------------------------- Client --------------------------------------------------

    pub fn create_client<S>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
    ) -> Result<Client<S>>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
    {
        let r2r_client = self
            .r2r_node
            .lock()
            .unwrap()
            .create_client::<S>(service_name, qos_profile)?;
        let client = Client::Defined {
            r2r_client: Arc::new(r2r_client),
        };
        Ok(client)
    }

    //-------------------------------------------------- Spin --------------------------------------------------

    pub fn spin(&mut self) {
        let mutex = self.r2r_node.clone();
        let handle = std::thread::spawn(move || {
            loop {
                mutex
                    .lock()
                    .unwrap()
                    .spin_once(std::time::Duration::from_millis(100));
            }
        });
        handle.join().unwrap();
    }
}
