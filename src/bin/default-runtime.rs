use std::time::Duration;

fn main() {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(3));
        println!("woke up 1! {:?}", std::thread::current().id());
    });

    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(3));
        println!("woke up 2! {:?}", std::thread::current().id());
    });

    std::thread::sleep(Duration::from_secs(5));
    println!("Done");
}
