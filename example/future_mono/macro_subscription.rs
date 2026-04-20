use r2r::QosProfile;
use rutile_r2r::future_mono::*;

async fn sub_0(message: r2r::std_msgs::msg::String) {
    println!("future_mono subscription arity 0: {:?}", message.data);
}

async fn sub_2(a: i32, b: i32, message: r2r::std_msgs::msg::String) {
    println!(
        "future_mono subscription arity 2: {} {} {:?}",
        a, b, message.data
    );
}

async fn sub_5(a: i32, b: i32, c: i32, d: i32, e: i32, message: r2r::std_msgs::msg::String) {
    println!(
        "future_mono subscription arity 5: {} {} {} {} {} {:?}",
        a, b, c, d, e, message.data
    );
}

fn main() -> Result<()> {
    let mut node = Node::create("macro_subscription_future_mono", "")?;

    rutile_r2r::create_subscription!(node, "topic", QosProfile::default(), sub_0)?;
    rutile_r2r::create_subscription!(
        node,
        "topic",
        QosProfile::default(),
        sub_2,
        10,
        20,
    )?;
    rutile_r2r::create_subscription!(
        node,
        "topic",
        QosProfile::default(),
        sub_5,
        1,
        2,
        3,
        4,
        5,
    )?;

    node.spin(std::time::Duration::from_millis(10));
    Ok(())
}
