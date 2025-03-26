use r2r::{QosProfile, example_interfaces::srv::AddTwoInts};
use rutile::future::*;

#[derive(Default, Clone)]
pub struct MyData {
    pub i: i64,
}

impl MyData {
    pub fn new(i: i64) -> Self {
        Self { i }
    }

    pub fn i(&self) -> i64 {
        self.i
    }

    pub fn inc(&mut self) {
        self.i += 1
    }
}

async fn client_timer_callback(data: FutureMutex<MyData>, ros: Ros) {
    println!("-------------------------");
    let request = AddTwoInts::Request {
        a: data.lock().await.i,
        b: 1,
    };
    println!("request: {:?}", request);
    let response = ros.client.call(request).await.unwrap();
    println!("response: {:?}", response);
    data.lock().await.i = response.sum;
}

// async fn client_timer_callback(data: FutureMutex<MyData>, ros: Ros) -> Result<()> {
//     let mut data = data.lock().await;
//     //
//     println!("-------------------------");
//     let request = AddTwoInts::Request { a: data.i, b: 1 };
//     println!("request: {:?}", request);
//     let response = ros.client.call(request).await?;
//     println!("response: {:?}", response);
//     data.i = response.sum;
//     //
//     Ok(())
// }

#[derive(Clone)]
pub struct Ros {
    pub client: Client<AddTwoInts::Service>,
}

async fn timer_callback(data: FutureMutex<MyData>) {
    println!("-------------------------");
    let i = { data.lock().await.i };
    println!("hello: {}", i);
}

fn main() -> Result<()> {
    //
    let data = MyData::new(10);
    let data_mutex = FutureMutex::create(data);
    //
    let mut node = Node::create("async_core", "")?;
    //
    let ros = Ros {
        client: node.create_client("/add", QosProfile::default())?,
    };
    //
    node.create_wall_timer_2(
        std::time::Duration::from_secs(1),
        client_timer_callback,
        data_mutex.clone(),
        ros.clone(),
    )?;

    node.create_wall_timer_1(
        std::time::Duration::from_secs(1),
        timer_callback,
        data_mutex.clone(),
    )?;

    node.spin();

    Ok(())
}
