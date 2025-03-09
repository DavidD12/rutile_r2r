use super::{Core, NodeInterface};
use crate::Result;
use futures::task::LocalSpawnExt;
use std::sync::{Arc, Mutex};

pub enum Client<T, S>
where
    T: 'static,
    S: r2r::WrappedServiceTypeSupport,
{
    Empty,
    Defined {
        name: String,
        core_mutex: Arc<Mutex<Core>>,
        data_mutex: Arc<Mutex<T>>,
        r2r_client: Arc<r2r::Client<S>>,
        callback: Arc<dyn Fn(Arc<Mutex<Core>>, Arc<Mutex<T>>, &S::Response) -> Result<()>>,
    },
}

impl<T, S> Client<T, S>
where
    T: 'static,
    S: r2r::WrappedServiceTypeSupport + 'static,
{
    pub fn call(&self, request: S::Request) -> Result<()> {
        match self {
            Client::Empty => Err("service not initialized".to_string().into()),
            Client::Defined {
                name,
                core_mutex,
                data_mutex,
                r2r_client,
                callback,
            } => {
                let name = name.clone();
                let core_mutex = core_mutex.clone();
                let r2r_client = r2r_client.clone();
                let data_mutex = data_mutex.clone();
                let callback = Arc::clone(&callback);

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
                                Ok(response) => {
                                    match callback(core_mutex.clone(), data_mutex, &response) {
                                        Ok(()) => {}
                                        Err(e) => {
                                            r2r::log_error!(
                                                &logger,
                                                "service callback '{}' error: {:?}",
                                                name,
                                                e
                                            );
                                        }
                                    }
                                }
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
                                r2r::log_error!(
                                    &logger,
                                    "service '{}' resquest error: {:?}",
                                    name,
                                    e
                                );
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
    }
}
