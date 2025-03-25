use super::*;

#[derive(Clone)]
pub enum Client<S>
where
    S: r2r::WrappedServiceTypeSupport,
{
    Empty,
    Defined { r2r_client: Arc<r2r::Client<S>> },
}

impl<S> std::default::Default for Client<S>
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
    pub async fn call(&self, request: S::Request) -> Result<S::Response> {
        match self {
            Client::Empty => Err("service not initialized".to_string().into()),
            Client::Defined { r2r_client } => {
                let r2r_client = r2r_client.clone();

                let service_available = r2r::Node::is_available(&*r2r_client)?;

                match service_available.await {
                    Ok(()) => match r2r_client.request(&request) {
                        Ok(future) => match future.await {
                            Ok(response) => {
                                return Ok(response);
                            }
                            Err(e) => Err(e.into()),
                        },
                        Err(e) => Err(e.into()),
                    },
                    Err(e) => Err(e.into()),
                }
            }
        }
    }
}
