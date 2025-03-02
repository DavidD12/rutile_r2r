use futures::{executor::LocalSpawner, task::LocalSpawnExt};

pub use super::*;

pub enum Client<T, S>
where
    T: Data,
    S: r2r::WrappedServiceTypeSupport,
{
    Empty,
    Defined {
        name: String,
        r2r_node_mutex: Arc<Mutex<r2r::Node>>,
        r2r_client: Arc<r2r::Client<S>>,
        callback: Arc<dyn Fn(Arc<Mutex<r2r::Node>>, Arc<Mutex<T>>, &S::Response) -> Result<()>>,
        data_mutex: Arc<Mutex<T>>,
        spawner: Arc<LocalSpawner>,
    },
}

impl<T, S> Default for Client<T, S>
where
    T: Data,
    S: r2r::WrappedServiceTypeSupport,
{
    fn default() -> Self {
        Client::Empty
    }
}

impl<T, S> Client<T, S>
where
    T: Data,
    S: r2r::WrappedServiceTypeSupport + 'static,
{
    pub fn call(&self, request: S::Request) -> Result<()> {
        match self {
            Client::Empty => Err("service not initialized".to_string().into()),
            Client::Defined {
                name,
                r2r_node_mutex,
                r2r_client,
                callback,
                data_mutex,
                spawner,
            } => {
                let name = name.to_string();
                let r2r_node_mutex = r2r_node_mutex.clone();
                let r2r_client = r2r_client.clone();
                let data_mutex = data_mutex.clone();
                let callback = Arc::clone(&callback);

                let service_available = r2r::Node::is_available(&*r2r_client)?;
                spawner.spawn_local(async move {
                    match service_available.await {
                        Ok(()) => match r2r_client.request(&request) {
                            Ok(future) => match future.await {
                                Ok(response) => {
                                    match callback(r2r_node_mutex.clone(), data_mutex, &response) {
                                        Ok(()) => {}
                                        Err(e) => {
                                            let r2r_node = r2r_node_mutex.lock().unwrap();
                                            r2r::log_error!(
                                                r2r_node.logger(),
                                                "service callback '{}' error: {:?}",
                                                name,
                                                e
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    let r2r_node = r2r_node_mutex.lock().unwrap();
                                    r2r::log_error!(
                                        r2r_node.logger(),
                                        "service '{}' resquest future error: {:?}",
                                        name,
                                        e
                                    );
                                }
                            },
                            Err(e) => {
                                let r2r_node = r2r_node_mutex.lock().unwrap();
                                r2r::log_error!(
                                    r2r_node.logger(),
                                    "service '{}' resquest error: {:?}",
                                    name,
                                    e
                                );
                            }
                        },
                        Err(e) => {
                            let r2r_node = r2r_node_mutex.lock().unwrap();
                            r2r::log_error!(
                                r2r_node.logger(),
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
