use tokio::time::{sleep, Duration};
use tokio::{join, select};

#[tokio::main]
async fn main(){
    let (res1, res2) = join!(
        do_slow_task("Pasta", 5),
        do_slow_task("maggie", 2),
    );

    println!("Join Done {res1}, {res2}");

    let selected = select!{
        val1 = do_slow_task("Donut", 3) => val1,
        val2 = do_slow_task("Poha", 4) => val2
    };

    println!("Slected Done: {selected}");
}

async fn do_slow_task(name: &str, seconds: u64) -> String{
    println!("Starting {name} in ({})s", seconds);
    sleep(Duration::from_secs(seconds)).await;
    format!("Finished {name}")
}