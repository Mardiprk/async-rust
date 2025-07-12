use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main(){
    println!("Starting tasks...");

    let task1 = tokio::spawn(async{
        println!("Task 1 Making Noodles..");
        sleep(Duration::from_secs(2)).await;
        println!("Task 1 Done!");
        42
    });

    let task2 = tokio::spawn(async{
        println!("Task 2 Making Sushi...");
        sleep(Duration::from_secs(10)).await;
        println!("Task 2 Done!");
        100
    });

    let res1 = task1.await.unwrap();
    let res2 = task2.await.unwrap();

    println!("📦 Results: Task 1 = {:?}, Task 2 = {:?}", res1, res2);
    println!("🎉 All tasks done!");
}