use std::process;
use std::time::Duration;

/*
 * woke up 1! 73731 ThreadId(4)
 * woke up 2! 73731 ThreadId(8)
 * Done 73731 ThreadId(1)
 */

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!(
            "woke up 1! {} {:?}",
            process::id(),
            std::thread::current().id()
        );
    });

    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!(
            "woke up 2! {} {:?}",
            process::id(),
            std::thread::current().id()
        );
    });

    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Done {} {:?}", process::id(), std::thread::current().id());
}
