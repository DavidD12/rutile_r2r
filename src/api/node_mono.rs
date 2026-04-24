pub trait NodeMono: Sized {
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

    fn create_wall_timer<T, F>(
        &self,
        period: std::time::Duration,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        T: Clone + 'static,
        F: 'static + Fn(T),
    {
        self.create_wall_timer_1::<T, F>(period, callback, data)
    }

    fn create_wall_timer_0<F>(&self, period: std::time::Duration, callback: F) -> crate::Result<()>
    where
        F: 'static + Fn();

    fn create_wall_timer_1<T, F>(
        &self,
        period: std::time::Duration,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        T: Clone + 'static,
        F: 'static + Fn(T);

    fn create_wall_timer_2<T1, T2, F>(
        &self,
        period: std::time::Duration,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> crate::Result<()>
    where
        T1: Clone + 'static,
        T2: Clone + 'static,
        F: 'static + Fn(T1, T2);

    fn create_wall_timer_3<T1, T2, T3, F>(
        &self,
        period: std::time::Duration,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
    ) -> crate::Result<()>
    where
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        F: 'static + Fn(T1, T2, T3);

    fn create_wall_timer_4<T1, T2, T3, T4, F>(
        &self,
        period: std::time::Duration,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
        data_4: T4,
    ) -> crate::Result<()>
    where
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
        F: 'static + Fn(T1, T2, T3, T4);

    fn create_wall_timer_5<T1, T2, T3, T4, T5, F>(
        &self,
        period: std::time::Duration,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
        data_4: T4,
        data_5: T5,
    ) -> crate::Result<()>
    where
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
        T5: Clone + 'static,
        F: 'static + Fn(T1, T2, T3, T4, T5);

    //-------------------------------------------------- Publisher --------------------------------------------------

    fn create_publisher<M>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
    ) -> crate::Result<Self::Publisher<M>>
    where
        M: r2r::WrappedTypesupport;

    //-------------------------------------------------- Subscriber --------------------------------------------------

    fn create_subscription<M, T, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T: Clone + 'static,
        F: 'static + Fn(T, M),
    {
        self.create_subscription_1::<M, T, F>(topic, qos_profile, callback, data)
    }

    fn create_subscription_0<M, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
    ) -> crate::Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        F: 'static + Fn(M);

    fn create_subscription_1<M, T, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T: Clone + 'static,
        F: 'static + Fn(T, M);

    fn create_subscription_2<M, T1, T2, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> crate::Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        F: 'static + Fn(T1, T2, M);

    fn create_subscription_3<M, T1, T2, T3, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
    ) -> crate::Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        F: 'static + Fn(T1, T2, T3, M);

    fn create_subscription_4<M, T1, T2, T3, T4, F>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
        data_4: T4,
    ) -> crate::Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
        F: 'static + Fn(T1, T2, T3, T4, M);

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
    ) -> crate::Result<()>
    where
        M: Send + 'static + r2r::WrappedTypesupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
        T5: Clone + 'static,
        F: 'static + Fn(T1, T2, T3, T4, T5, M);

    //-------------------------------------------------- Service --------------------------------------------------

    fn create_service<S, T, F>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T: Clone + 'static,
        F: 'static + Fn(T, S::Request) -> S::Response,
    {
        self.create_service_1::<S, T, F>(service_name, qos_profile, callback, data)
    }

    fn create_service_0<S, F>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        F: 'static + Fn(S::Request) -> S::Response;

    fn create_service_typed_0<S, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        F: 'static + Fn(S::Request) -> R,
        R: Into<S::Response>,
    {
        self.create_service_0::<S, _>(service_name, qos_profile, move |req| callback(req).into())
    }

    fn create_service_1<S, T, F>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T: Clone + 'static,
        F: 'static + Fn(T, S::Request) -> S::Response;

    fn create_service_typed_1<S, T, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data: T,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T: Clone + 'static,
        F: 'static + Fn(T, S::Request) -> R,
        R: Into<S::Response>,
    {
        self.create_service_1::<S, T, _>(
            service_name,
            qos_profile,
            move |v, req| callback(v, req).into(),
            data,
        )
    }

    fn create_service_2<S, T1, T2, F>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        F: 'static + Fn(T1, T2, S::Request) -> S::Response;

    fn create_service_typed_2<S, T1, T2, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        F: 'static + Fn(T1, T2, S::Request) -> R,
        R: Into<S::Response>,
    {
        self.create_service_2::<S, T1, T2, _>(
            service_name,
            qos_profile,
            move |v1, v2, req| callback(v1, v2, req).into(),
            data_1,
            data_2,
        )
    }

    fn create_service_3<S, T1, T2, T3, F>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        F: 'static + Fn(T1, T2, T3, S::Request) -> S::Response;

    fn create_service_typed_3<S, T1, T2, T3, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        F: 'static + Fn(T1, T2, T3, S::Request) -> R,
        R: Into<S::Response>,
    {
        self.create_service_3::<S, T1, T2, T3, _>(
            service_name,
            qos_profile,
            move |v1, v2, v3, req| callback(v1, v2, v3, req).into(),
            data_1,
            data_2,
            data_3,
        )
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
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
        F: 'static + Fn(T1, T2, T3, T4, S::Request) -> S::Response;

    fn create_service_typed_4<S, T1, T2, T3, T4, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
        data_4: T4,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
        F: 'static + Fn(T1, T2, T3, T4, S::Request) -> R,
        R: Into<S::Response>,
    {
        self.create_service_4::<S, T1, T2, T3, T4, _>(
            service_name,
            qos_profile,
            move |v1, v2, v3, v4, req| callback(v1, v2, v3, v4, req).into(),
            data_1,
            data_2,
            data_3,
            data_4,
        )
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
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
        T5: Clone + 'static,
        F: 'static + Fn(T1, T2, T3, T4, T5, S::Request) -> S::Response;

    fn create_service_typed_5<S, T1, T2, T3, T4, T5, F, R>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: F,
        data_1: T1,
        data_2: T2,
        data_3: T3,
        data_4: T4,
        data_5: T5,
    ) -> crate::Result<()>
    where
        S: 'static + r2r::WrappedServiceTypeSupport,
        T1: Clone + 'static,
        T2: Clone + 'static,
        T3: Clone + 'static,
        T4: Clone + 'static,
        T5: Clone + 'static,
        F: 'static + Fn(T1, T2, T3, T4, T5, S::Request) -> R,
        R: Into<S::Response>,
    {
        self.create_service_5::<S, T1, T2, T3, T4, T5, _>(
            service_name,
            qos_profile,
            move |v1, v2, v3, v4, v5, req| callback(v1, v2, v3, v4, v5, req).into(),
            data_1,
            data_2,
            data_3,
            data_4,
            data_5,
        )
    }

    //-------------------------------------------------- Client --------------------------------------------------

    fn create_client<S>(
        &self,
        service_name: &str,
        qos_profile: r2r::QosProfile,
    ) -> crate::Result<Self::Client<S>>
    where
        S: 'static + r2r::WrappedServiceTypeSupport;

    fn spin(&mut self, timeout: std::time::Duration);
}
