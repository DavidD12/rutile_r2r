use super::*;

#[derive(Clone)]
pub enum Publisher<M>
where
    M: r2r::WrappedTypesupport,
{
    Empty,
    Defined {
        r2r_publisher: SyncMutex<r2r::Publisher<M>>,
    },
}

impl<M> Default for Publisher<M>
where
    M: r2r::WrappedTypesupport,
{
    fn default() -> Self {
        Self::Empty
    }
}

impl<M> Publisher<M>
where
    M: r2r::WrappedTypesupport + 'static,
{
    pub fn publish(&self, msg: &M) -> Result<()> {
        match self {
            Publisher::Empty => Err("publisher not initialized".to_string().into()),
            Publisher::Defined { r2r_publisher } => {
                r2r_publisher.lock().unwrap().publish(msg)?;
                Ok(())
            }
        }
    }
}
