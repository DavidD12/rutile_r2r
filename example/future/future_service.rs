use r2r::{QosProfile, example_interfaces::srv::AddTwoInts};
use rutile_r2r::future::*;

async fn add(request: AddTwoInts::Request) -> AddTwoInts::Response {
    println!("request: '{:?}'", request);
    let response = AddTwoInts::Response {
        sum: request.a + request.b,
    };
    response
}

fn main() -> Result<()> {
    let mut node = Node::create("minimal_service", "")?;
    //
    node.create_service_0::<AddTwoInts::Service, _, _>("add_two_ints", QosProfile::default(), add)?;
    //
    node.spin(std::time::Duration::from_millis(10));
    //
    Ok(())
}
