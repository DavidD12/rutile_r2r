use r2r::{QosProfile, std_msgs};
use rutile::*;

#[derive(Default)]
pub struct MyData {}

impl Data for MyData {
    fn initialize(&mut self, _: &Node<Self>) -> Result<()> {
        Ok(())
    }
}

fn subscription_callback(
    _: Arc<Mutex<r2r::Node>>,
    _: Arc<Mutex<MyData>>,
    message: &std_msgs::msg::String,
) -> Result<()> {
    println!("message received: {}", message.data);

    Ok(())
}

fn main() -> Result<()> {
    let mut node = Node::create("wall_timer_node", "")?;

    node.create_subscription("/topic", QosProfile::default(), subscription_callback)?;

    node.spin();

    Ok(())
}
