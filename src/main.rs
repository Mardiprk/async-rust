use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main(){

    let task1 = tokio::spawn(async{
        println!("🍜 Cooking Ramen!");
        sleep(Duration::from_secs(2)).await;
        println!("🍜 Ramen Cooked!");
        "Delicious Ramen" 
    });

    let task2 = tokio::spawn(async{
        println!("☕ Brewing coffee");
        sleep(Duration::from_secs(5)).await;
        println!("☕ Coffee Ready");
        "Tasty Coffee"
    });

    let res1 = task1.await.unwrap();
    let res2 = task2.await.unwrap();

    println!("📦 Results - Task 1: {:?}, Task 2: {:?}", res1, res2);
    println!("🎉 All tasks done!");

}