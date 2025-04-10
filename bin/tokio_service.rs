use r2r::{QosProfile, example_interfaces::srv::AddTwoInts};
use rutile::tokio::*;

async fn add(request: AddTwoInts::Request) -> AddTwoInts::Response {
    println!("request: '{:?}'", request);
    let response = AddTwoInts::Response {
        sum: request.a + request.b,
    };
    response
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut node = Node::create("minimal_service", "")?;
    //
    node.create_service::<AddTwoInts::Service, _, _>("add_two_ints", QosProfile::default(), add)?;
    //
    node.spin();
    //
    Ok(())
}
