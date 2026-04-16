use r2r::QosProfile;
use rutile_r2r::future_mono::*;

async fn topic_callback(message: r2r::std_msgs::msg::String) {
    println!("I heard: '{:?}'", message);
}

fn main() -> Result<()> {
    let mut node = Node::create("minimal_subscriber", "")?;
    node.create_subscription_0("topic", QosProfile::default(), topic_callback)?;
    node.spin(std::time::Duration::from_millis(10));
    Ok(())
}
