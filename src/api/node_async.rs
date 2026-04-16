use std::future::Future;

pub trait NodeAsync: Sized {
    type Publisher<M: r2r::WrappedTypesupport>;
    type Client<S: r2r::WrappedServiceTypeSupport>;

    //-------------------------------------------------- Create --------------------------------------------------

    fn create(name: &str, namespace: &str) -> crate::Result<Self>;

    //-------------------------------------------------- R2R --------------------------------------------------

    fn r2r(&self) -> crate::SMutex<r2r::Node>;

    //-------------------------------------------------- Now --------------------------------------------------

    fn now(&self) -> std::time::Duration;

    //-------------------------------------------------- Logger --------------------------------------------------

    fn logger(&self) -> String;

    //-------------------------------------------------- Parameter --------------------------------------------------

    fn get_parameter<P>(&self, name: &str) -> crate::Result<P>
    where
        r2r::ParameterValue: TryInto<P, Error = r2r::WrongParameterType>;

    fn get_parameter_with_default<P>(&self, name: &str, default: P) -> crate::Result<P>
    where
        r2r::ParameterValue: TryInto<Option<P>, Error = r2r::WrongParameterType>;

    //-------------------------------------------------- Timer --------------------------------------------------

    fn create_wall_timer<T, F, R>(
        &self,
        period: std::time::Duration,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        T: Clone + Send + 'static,
        F: Send + 'static,
        F: Fn(T) -> R,
        R: Future<Output = ()>,
        R: Send,
    {
        self.create_wall_timer_1::<T, F, R>(period, callback, data)
    }

    fn create_wall_timer_0<F, R>(
        &self,
        period: std::time::Duration,
        callback: F,
    ) -> crate::Result<()>
    where
        F: Send + 'static,
        F: Fn() -> R,
        R: Future<Output = ()>,
        R: Send;

    fn create_wall_timer_1<T, F, R>(
        &self,
        period: std::time::Duration,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        T: Clone + Send + 'static,
        F: Send + 'static,
        F: Fn(T) -> R,
        R: Future<Output = ()>,
        R: Send;

    fn create_wall_timer_2<T1, T2, F, R>(
        &self,
        period: std::time::Duration,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> crate::Result<()>
    where
        T1: Clone + Send + 'static,
        T2: Clone + Send + 'static,
        F: Send + 'static,
        F: Fn(T1, T2) -> R,
        R: Future<Output = ()>,
        R: Send;

    //-------------------------------------------------- Publisher --------------------------------------------------

    fn create_publisher<M>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
    ) -> crate::Result<Self::Publisher<M>>
    where
        M: r2r::WrappedTypesupport;

    //-------------------------------------------------- Subscriber --------------------------------------------------

    fn create_subscription<M, T, F, R>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static,
        F: Fn(T, M) -> R,
        R: Future<Output = ()>,
        R: Send,
    {
        self.create_subscription_1::<M, T, F, R>(topic, qos_profile, callback, data)
    }

    fn create_subscription_0<M, F, R>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
    ) -> crate::Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        F: Send + Sync + 'static,
        F: Fn(M) -> R,
        R: Future<Output = ()>,
        R: Send;

    fn create_subscription_1<M, T, F, R>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static,
        F: Fn(T, M) -> R,
        R: Future<Output = ()>,
        R: Send;

    fn create_subscription_2<M, T1, T2, F, R>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> crate::Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T1: Clone + Send + Sync + 'static,
        T2: Clone + Send + Sync + 'static,
        F: Send + Sync + 'static,
        F: Fn(T1, T2, M) -> R,
        R: Future<Output = ()>,
        R: Send;

    //-------------------------------------------------- Service --------------------------------------------------

    fn create_service<S, T, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T: Clone + Send + 'static,
        F: Send + 'static,
        F: Fn(T, S::Request) -> R,
        R: Future<Output = S::Response>,
        R: Send,
    {
        self.create_service_1::<S, T, F, R>(service_name, qos_profile, callback, data)
    }

    fn create_service_0<S, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        F: Send + 'static,
        F: Fn(S::Request) -> R,
        R: Future<Output = S::Response>,
        R: Send;

    fn create_service_1<S, T, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T: Clone + Send + 'static,
        F: Send + 'static,
        F: Fn(T, S::Request) -> R,
        R: Future<Output = S::Response>,
        R: Send;

    fn create_service_2<S, T1, T2, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T1: Clone + Send + 'static,
        T2: Clone + Send + 'static,
        F: Send + 'static,
        F: Fn(T1, T2, S::Request) -> R,
        R: Future<Output = S::Response>,
        R: Send;

    //-------------------------------------------------- Client --------------------------------------------------

    fn create_client<S>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
    ) -> crate::Result<Self::Client<S>>
    where
        S: 'static + r2r::WrappedServiceTypeSupport;

    fn spin(&mut self, duration: std::time::Duration);
}
