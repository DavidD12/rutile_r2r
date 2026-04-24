use r2r::{example_interfaces::srv::AddTwoInts, QosProfile};
use rutile_r2r::multi::*;

fn add(request: AddTwoInts::Request) -> AddTwoInts::Response {
    println!("request: '{:?}'", request);
    AddTwoInts::Response {
        sum: request.a + request.b,
    }
}

fn main() -> Result<()> {
    let mut node = Node::create("minimal_service", "")?;
    node.create_service_0::<AddTwoInts::Service, _>("add_two_ints", QosProfile::default(), add)?;
    node.spin(std::time::Duration::from_millis(10));
    Ok(())
}
