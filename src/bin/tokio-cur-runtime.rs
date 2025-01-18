use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("woke up1! {:?}", std::thread::current().id());
    });

    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("woke up2! {:?}", std::thread::current().id());
    });

    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Done {:?}", std::thread::current().id());
}
