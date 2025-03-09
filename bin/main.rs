use r2r::QosProfile;
use rutile::core::*;

pub struct MyData {
    pub core_mutex: CoreMutex,
}

impl MyData {
    pub fn new(core_mutex: CoreMutex) -> Self {
        Self { core_mutex }
    }

    pub fn initialize(core_mutex: CoreMutex, self_mutex: Arc<Mutex<Self>>) -> Result<()> {
        core_mutex.create_subscription(
            self_mutex,
            "my_topic",
            QosProfile::default(),
            Self::sub_callback,
        )?;

        Ok(())
    }

    pub fn sub_callback(
        core_mutex: CoreMutex,
        self_mutex: Arc<Mutex<Self>>,
        _: &r2r::std_msgs::msg::Empty,
    ) -> Result<()> {
        core_mutex.create_wall_timer(
            self_mutex,
            std::time::Duration::from_secs(1),
            Self::timer_callback,
        )?;
        Ok(())
    }

    pub fn timer_callback(_: CoreMutex, _: Arc<Mutex<Self>>) -> Result<()> {
        println!("hello");
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut node = Node::create("rutile_node", "")?;
    let data = MyData::new(node.core_mutex());
    let data_mutex = Arc::new(Mutex::new(data));

    MyData::initialize(node.core_mutex(), data_mutex)?;

    node.spin();

    Ok(())
}
