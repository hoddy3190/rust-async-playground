use std::process;
use std::time::Duration;

/*
 * woke up 1! 73446 ThreadId(1)
 * woke up 2! 73446 ThreadId(1)
 * Done 73446 ThreadId(1)
 */

#[tokio::main(flavor = "current_thread")]
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
