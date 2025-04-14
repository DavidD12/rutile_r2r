use r2r::QosProfile;
use rutile::*;

async fn topic_callback(message: r2r::std_msgs::msg::String) {
    println!("I heard: '{:?}'", message);
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut node = Node::create("minimal_subscriber", "")?;
    //
    node.create_subscription("topic", QosProfile::default(), topic_callback)?;
    //
    node.spin();
    //
    Ok(())
}
