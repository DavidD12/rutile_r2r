use rutile::Result;
use rutile::sync::*;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default)]
pub struct MyData {
    count: i32,
    pub period: u64,
}

impl Data for MyData {
    fn initialize(&mut self, node: &Node<Self>) -> Result<()> {
        let period: i64 = node.get_parameter_with_default("period", 1000)?;
        // let array: Vec<String> = node.get_parameter_with_default("list", vec![])?;

        self.period = period as u64;
        Ok(())
    }
}

fn timer_callback(_: Arc<Mutex<r2r::Node>>, data_mutex: Arc<Mutex<MyData>>) -> Result<()> {
    let mut data = data_mutex.lock().unwrap();

    data.count += 1;
    println!("count = {}", data.count);

    Ok(())
}

fn main() -> Result<()> {
    let mut node: Node<MyData> = Node::create("wall_timer_node", "")?;
    {
        let data = node.data_mutex.lock().unwrap();
        node.create_wall_timer(
            std::time::Duration::from_millis(data.period),
            timer_callback,
        )?;
    }

    node.spin();

    Ok(())
}
