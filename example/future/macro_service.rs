use r2r::{QosProfile, example_interfaces::srv::AddTwoInts};
use rutile_r2r::future::*;

async fn srv_0(request: AddTwoInts::Request) -> AddTwoInts::Response {
    println!("future service arity 0: {} {}", request.a, request.b);
    AddTwoInts::Response {
        sum: request.a + request.b,
    }
}

async fn srv_2(a: i64, b: i64, request: AddTwoInts::Request) -> AddTwoInts::Response {
    println!("future service arity 2: {} {}", a, b);
    AddTwoInts::Response {
        sum: request.a + request.b + a + b,
    }
}

async fn srv_5(
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    request: AddTwoInts::Request,
) -> AddTwoInts::Response {
    println!("future service arity 5: {} {} {} {} {}", a, b, c, d, e);
    AddTwoInts::Response {
        sum: request.a + request.b + a + b + c + d + e,
    }
}

fn main() -> Result<()> {
    let mut node = Node::create("macro_service_future", "")?;

    rutile_r2r::create_service!(
        node,
        AddTwoInts::Service,
        "add_two_ints_0",
        QosProfile::default(),
        srv_0,
    )?;
    rutile_r2r::create_service!(
        node,
        AddTwoInts::Service,
        "add_two_ints_2",
        QosProfile::default(),
        srv_2,
        10_i64,
        20_i64,
    )?;
    rutile_r2r::create_service!(
        node,
        AddTwoInts::Service,
        "add_two_ints_5",
        QosProfile::default(),
        srv_5,
        1_i64,
        2_i64,
        3_i64,
        4_i64,
        5_i64,
    )?;

    node.spin(std::time::Duration::from_millis(10));
    Ok(())
}
