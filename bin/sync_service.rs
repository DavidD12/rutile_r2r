use r2r::{QosProfile, example_interfaces::srv::AddTwoInts};
use rutile::Result;
use rutile::sync::*;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default)]
pub struct MyData {}

impl Data for MyData {
    fn initialize(&mut self, _: &Node<Self>) -> Result<()> {
        Ok(())
    }
}

fn service_callback(
    _: Arc<Mutex<r2r::Node>>,
    _: Arc<Mutex<MyData>>,
    request: &AddTwoInts::Request,
) -> Result<AddTwoInts::Response> {
    println!("request: {:?}", request);
    let response = AddTwoInts::Response {
        sum: request.a + request.b,
    };
    Ok(response)
}

fn main() -> Result<()> {
    let mut node = Node::create("wall_timer_node", "")?;

    node.create_service::<AddTwoInts::Service>("/add", QosProfile::default(), service_callback)?;

    node.spin();

    Ok(())
}
