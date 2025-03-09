use super::*;
use std::sync::{Arc, Mutex};

pub trait NodeInterface {
    fn logger(&self) -> String;

    fn get_parameter<P>(&self, name: &str) -> r2r::Result<P>
    where
        r2r::ParameterValue: TryInto<P, Error = r2r::WrongParameterType>;

    fn create_wall_timer<T>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        period: std::time::Duration,
        callback: fn(Arc<Mutex<Core>>, Arc<Mutex<T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: 'static;

    fn create_publisher<M>(
        &self,
        topic: &str,
        qos_profile: r2r::QosProfile,
    ) -> Result<r2r::Publisher<M>>
    where
        M: r2r::WrappedTypesupport;

    fn create_subscription<T, M>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        topic: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<Core>>, Arc<Mutex<T>>, &M) -> Result<()>,
    ) -> Result<()>
    where
        T: 'static,
        M: 'static + r2r::WrappedTypesupport;

    fn create_service<T, S>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<Core>>, Arc<Mutex<T>>, &S::Request) -> Result<S::Response>,
    ) -> Result<()>
    where
        T: 'static,
        S: 'static + r2r::WrappedServiceTypeSupport;

    fn create_client<T, S>(
        &self,
        data_mutex: Arc<Mutex<T>>,
        service_name: &str,
        qos_profile: r2r::QosProfile,
        callback: fn(Arc<Mutex<Core>>, Arc<Mutex<T>>, &S::Response) -> Result<()>,
    ) -> Result<Client<T, S>>
    where
        T: 'static,
        S: 'static + r2r::WrappedServiceTypeSupport;
}
