use super::*;
use std::sync::{Arc, Mutex};

use futures::executor::LocalPool;

pub struct Node {
    core_mutex: CoreMutex,
    pool: LocalPool,
}

impl Node {
    pub fn create(name: &str, namespace: &str) -> Result<Self> {
        let ctx = r2r::Context::create()?;
        let r2r_node = r2r::Node::create(ctx, name, namespace)?;
        //
        let pool = LocalPool::new();
        let spawner = pool.spawner();
        let core = Core { r2r_node, spawner };
        let core_mutex = Arc::new(Mutex::new(core));
        //
        let node = Self { core_mutex, pool };
        Ok(node)
    }

    pub fn core_mutex(&self) -> Arc<Mutex<Core>> {
        self.core_mutex.clone()
    }

    pub fn spin(&mut self) {
        loop {
            {
                let mut core = self.core_mutex.lock().unwrap();
                core.r2r_node
                    .spin_once(std::time::Duration::from_millis(10_000));
            }
            self.pool.run_until_stalled();
        }
    }
}

// impl NodeInterface for Node {
//     fn logger(&self) -> String {
//         self.core_mutex.logger()
//     }

//     fn get_parameter<P>(&self, name: &str) -> r2r::Result<P>
//     where
//         r2r::ParameterValue: TryInto<P, Error = r2r::WrongParameterType>,
//     {
//         self.core_mutex.get_parameter(name)
//     }

//     fn create_wall_timer<T>(
//         &self,
//         data_mutex: Arc<Mutex<T>>,
//         period: std::time::Duration,
//         callback: fn(Arc<Mutex<Core>>, Arc<Mutex<T>>) -> Result<()>,
//     ) -> Result<()>
//     where
//         T: 'static,
//     {
//         self.core_mutex
//             .create_wall_timer(data_mutex, period, callback)
//     }
// }
