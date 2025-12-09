use std::process;
use std::time::Duration;

/*
 * woke up 1! 73194 ThreadId(2)
 * woke up 2! 73194 ThreadId(3)
 * Done 73194 ThreadId(1)
 */

fn main() {
    std::thread::spawn(|| loop {
        std::thread::sleep(Duration::from_secs(1));
        println!(
            "woke up 1! {} {:?}",
            process::id(),
            std::thread::current().id()
        );
    });

    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(1));
        println!(
            "woke up 2! {} {:?}",
            process::id(),
            std::thread::current().id()
        );
    });

    std::thread::sleep(Duration::from_secs(5));
    println!("Done {} {:?}", process::id(), std::thread::current().id());
}
