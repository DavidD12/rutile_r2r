use r2r::{QosProfile, std_msgs};
use rutile::Result;
use rutile::sync::*;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default)]
pub struct MyData {
    count: i32,
    publisher: Publisher<std_msgs::msg::String>,
}

impl Data for MyData {
    fn initialize(&mut self, node: &Node<Self>) -> Result<()> {
        self.count = 0;
        self.publisher = node.create_publisher("/topic", QosProfile::default())?;
        Ok(())
    }
}

fn timer_callback(_: Arc<Mutex<r2r::Node>>, data_mutex: Arc<Mutex<MyData>>) -> Result<()> {
    let mut data = data_mutex.lock().unwrap();
    data.count += 1;
    let s = format!("count = {}", data.count);
    let msg = std_msgs::msg::String { data: s };
    data.publisher.publish(&msg)?;

    Ok(())
}

fn main() -> Result<()> {
    let mut node = Node::create("wall_timer_node", "")?;

    node.create_wall_timer(std::time::Duration::from_millis(100), timer_callback)?;

    node.spin();

    Ok(())
}
