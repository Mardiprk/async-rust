use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main(){
    println!("START!");
    say_hello().await;
    println!("END!");
}

async fn say_hello(){
    println!("Hello from Async Function!");
    sleep(Duration::from_secs(5)).await;
    println!("Finished after 5 sec wait!");
}