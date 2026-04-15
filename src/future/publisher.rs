use crate::MutexLockOrLog;

#[derive(Clone)]
pub enum Publisher<M>
where
    M: r2r::WrappedTypesupport,
{
    Empty,
    Defined {
        logger: String,
        r2r_publisher: crate::SMutex<r2r::Publisher<M>>,
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
    pub fn publish(&self, msg: &M) {
        match self {
            Publisher::Empty => {
                r2r::log_error!("", "publisher not initialized");
            }
            Publisher::Defined {
                logger,
                r2r_publisher,
            } => {
                if let Err(e) = r2r_publisher.lock_or_log("r2r_publisher").publish(msg) {
                    r2r::log_error!(logger, "{}", e);
                }
            }
        }
    }
}
