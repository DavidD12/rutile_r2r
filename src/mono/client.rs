use crate::MutexLockOrLog;
use futures::task::LocalSpawnExt;
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll};

#[derive(Clone)]
pub enum Client<S>
where
    S: r2r::WrappedServiceTypeSupport,
{
    Empty,
    Defined {
        r2r_client: Arc<r2r::Client<S>>,
        r2r_node: crate::SMutex<r2r::Node>,
        local_spawner: futures::executor::LocalSpawner,
    },
}

impl<S> Default for Client<S>
where
    S: r2r::WrappedServiceTypeSupport,
{
    fn default() -> Self {
        Self::Empty
    }
}

impl<S> Client<S>
where
    S: r2r::WrappedServiceTypeSupport + 'static,
{
    pub fn call_blocking(&self, request: S::Request) -> crate::Result<S::Response> {
        match self {
            Client::Empty => Err("service not initialized".to_string().into()),
            Client::Defined {
                r2r_client,
                r2r_node,
                ..
            } => {
                let r2r_client = r2r_client.clone();
                let service_available = r2r::Node::is_available(&*r2r_client)?;
                let mut service_available = Box::pin(service_available);

                loop {
                    let waker = futures::task::noop_waker_ref();
                    let mut cx = Context::from_waker(waker);
                    match service_available.as_mut().poll(&mut cx) {
                        Poll::Ready(result) => {
                            result?;
                            break;
                        }
                        Poll::Pending => {
                            let mut node = r2r_node.lock_or_log("r2r_node in client.call");
                            node.spin_once(std::time::Duration::from_millis(1));
                        }
                    }
                }

                let response_future = r2r_client.request(&request)?;
                let mut response_future = Box::pin(response_future);

                loop {
                    let waker = futures::task::noop_waker_ref();
                    let mut cx = Context::from_waker(waker);
                    match response_future.as_mut().poll(&mut cx) {
                        Poll::Ready(result) => return result.map_err(|e| e.into()),
                        Poll::Pending => {
                            let mut node = r2r_node.lock_or_log("r2r_node in client.call_blocking");
                            node.spin_once(std::time::Duration::from_millis(1));
                        }
                    }
                }
            }
        }
    }

    pub fn call<F>(&self, request: S::Request, callback: F) -> crate::Result<()>
    where
        F: FnOnce(crate::Result<S::Response>) + 'static,
    {
        match self {
            Client::Empty => Err("service not initialized".to_string().into()),
            Client::Defined {
                r2r_client,
                local_spawner,
                ..
            } => {
                let r2r_client = r2r_client.clone();

                local_spawner.spawn_local(async move {
                    let result: crate::Result<S::Response> = async {
                        let service_available = r2r::Node::is_available(&*r2r_client)?;
                        service_available.await?;

                        let response_future = r2r_client.request(&request)?;
                        let response = response_future.await?;
                        Ok(response)
                    }
                    .await;

                    callback(result);
                })?;

                Ok(())
            }
        }
    }
}
