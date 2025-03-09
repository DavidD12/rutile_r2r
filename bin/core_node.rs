use r2r::{QosProfile, example_interfaces::srv::AddTwoInts};
use rutile::*;

#[derive(Default)]
pub struct MainData {
    pub a: i64,
    pub b: i64,
    pub sub_data_mutex: Arc<Mutex<SubData>>,
}

impl MainData {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            sub_data_mutex: Arc::new(Mutex::new(SubData::default())),
        }
    }

    pub fn initialize(&mut self, node: &CoreNode) -> Result<()> {
        SubData::initialize(self.sub_data_mutex.clone(), node)
    }
}

#[derive(Default)]
pub struct SubData {
    pub client: ClientAsync<Self, AddTwoInts::Service>,
}

impl SubData {
    fn initialize(self_mutex: Arc<Mutex<Self>>, node: &CoreNode) -> Result<()> {
        //
        let client = node.create_client::<Self, AddTwoInts::Service>(
            self_mutex.clone(),
            "/add",
            QosProfile::default(),
            client_callback,
        )?;

        let mut this = self_mutex.lock().unwrap();
        this.client = client;

        Ok(())
    }
}

fn client_callback(
    _: Arc<Mutex<r2r::Node>>,
    _: Arc<Mutex<SubData>>,
    response: &AddTwoInts::Response,
) -> Result<()> {
    println!("response: {:?}", response);
    Ok(())
}

fn timer_callback(_: Arc<Mutex<r2r::Node>>, data_mutex: Arc<Mutex<MainData>>) -> Result<()> {
    let mut data = data_mutex.lock().unwrap();

    let request = AddTwoInts::Request {
        a: data.a,
        b: data.b,
    };
    {
        let sub = data.sub_data_mutex.lock().unwrap();
        sub.client.call(request)?;
    }

    data.a += 1;
    data.b += 2;

    Ok(())
}

fn main() -> Result<()> {
    let mut node = CoreNode::create("wall_timer_node", "")?;

    let mut data = MainData::default();
    data.initialize(&node)?;

    let data = Arc::new(Mutex::new(data));

    node.create_wall_timer(data, std::time::Duration::from_millis(1000), timer_callback)?;

    node.spin();

    Ok(())
}
