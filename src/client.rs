pub use super::*;
use futures::channel::oneshot;
use futures::executor::LocalPool;
use futures::{executor::LocalSpawner, task::LocalSpawnExt};

pub enum ClientSync<S>
where
    S: r2r::WrappedServiceTypeSupport,
{
    Empty,
    Defined {
        name: String,
        r2r_client: Arc<r2r::Client<S>>,
        spawner: Arc<LocalSpawner>,
    },
}

impl<S> Default for ClientSync<S>
where
    S: r2r::WrappedServiceTypeSupport,
{
    fn default() -> Self {
        ClientSync::Empty
    }
}

impl<S> ClientSync<S>
where
    S: r2r::WrappedServiceTypeSupport + 'static,
{
    pub fn call(&self, request: S::Request) -> Result<S::Response> {
        match self {
            ClientSync::Empty => Err("service not initialized".to_string().into()),
            ClientSync::Defined {
                name,
                r2r_client,
                spawner: _,
            } => {
                let name = name.to_string();
                let r2r_client = r2r_client.clone();
                let (sender, receiver) = oneshot::channel::<Result<S::Response>>();

                let mut pool = LocalPool::new();
                let spawner = pool.spawner();

                let service_available = r2r::Node::is_available(&*r2r_client)?;
                spawner.spawn_local(async move {
                    match service_available.await {
                        Ok(()) => match r2r_client.request(&request) {
                            Ok(future) => match future.await {
                                Ok(response) => {
                                    let response: S::Response = response;
                                    let _ = sender.send(Ok(response));
                                }
                                Err(e) => {
                                    let error = Err(format!(
                                        "service '{}' resquest future error: {:?}",
                                        name, e
                                    )
                                    .into());
                                    let _ = sender.send(error);
                                }
                            },
                            Err(e) => {
                                let error =
                                    Err(format!("service '{}' resquest error: {:?}", name, e)
                                        .into());
                                let _ = sender.send(error);
                            }
                        },
                        Err(e) => {
                            let error = Err(format!(
                                "service '{}' available future error: {:?}",
                                name, e
                            )
                            .into());
                            let _ = sender.send(error);
                        }
                    }
                    //
                })?;
                let result = pool.run_until(receiver)?;
                result
            }
        }
    }
}

pub enum ClientAsync<T, S>
where
    T: 'static,
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

impl<T, S> Default for ClientAsync<T, S>
where
    T: 'static,
    S: r2r::WrappedServiceTypeSupport,
{
    fn default() -> Self {
        ClientAsync::Empty
    }
}

impl<T, S> ClientAsync<T, S>
where
    T: 'static,
    S: r2r::WrappedServiceTypeSupport + 'static,
{
    pub fn call(&self, request: S::Request) -> Result<()> {
        match self {
            ClientAsync::Empty => Err("service not initialized".to_string().into()),
            ClientAsync::Defined {
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
