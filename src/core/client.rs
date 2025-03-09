use super::{Core, NodeInterface};
use crate::Result;
use futures::task::LocalSpawnExt;
use std::sync::{Arc, Mutex};

pub struct Client<T, S>
where
    T: 'static,
    S: r2r::WrappedServiceTypeSupport,
{
    pub(crate) name: String,
    pub(crate) core_mutex: Arc<Mutex<Core>>,
    pub(crate) data_mutex: Arc<Mutex<T>>,
    pub(crate) r2r_client: Arc<r2r::Client<S>>,
    pub(crate) callback: Arc<dyn Fn(Arc<Mutex<Core>>, Arc<Mutex<T>>, &S::Response) -> Result<()>>,
}

impl<T, S> Client<T, S>
where
    T: 'static,
    S: r2r::WrappedServiceTypeSupport + 'static,
{
    pub fn call(&self, request: S::Request) -> Result<()> {
        let name = self.name.clone();
        let core_mutex = self.core_mutex.clone();
        let r2r_client = self.r2r_client.clone();
        let data_mutex = self.data_mutex.clone();
        let callback = Arc::clone(&self.callback);

        let service_available = r2r::Node::is_available(&*r2r_client)?;
        let spawner = {
            let core = core_mutex.lock().unwrap();
            core.spawner.clone()
        };
        let logger = core_mutex.logger();
        spawner.spawn_local(async move {
            match service_available.await {
                Ok(()) => match r2r_client.request(&request) {
                    Ok(future) => match future.await {
                        Ok(response) => match callback(core_mutex.clone(), data_mutex, &response) {
                            Ok(()) => {}
                            Err(e) => {
                                r2r::log_error!(
                                    &logger,
                                    "service callback '{}' error: {:?}",
                                    name,
                                    e
                                );
                            }
                        },
                        Err(e) => {
                            r2r::log_error!(
                                &logger,
                                "service '{}' resquest future error: {:?}",
                                name,
                                e
                            );
                        }
                    },
                    Err(e) => {
                        r2r::log_error!(&logger, "service '{}' resquest error: {:?}", name, e);
                    }
                },
                Err(e) => {
                    r2r::log_error!(
                        &logger,
                        "service '{}' available future error: {:?}",
                        name,
                        e
                    );
                }
            }
            //
        })?;
        Ok(())
    }
}
