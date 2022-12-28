use lru_timed_cache::LRUTimedCache;

fn fibonacci(val: u32) -> u64 {
    match val {
        0|1 => 1,
        _ => fibonacci(val-1) + fibonacci(val-2)
    }
}
fn main() {
    let mut cache = LRUTimedCache::<u32,u64>::new(8, fibonacci);
    let mut timer = std::time::Instant::now();
    cache.get(40);
    println!("getting 40 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(10);
    println!("getting 10 took: {} milliseconds", timer.elapsed().as_millis());
    let mut timer = std::time::Instant::now();
    cache.get(40);
    println!("getting 40 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(30);
    println!("getting 30 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(1);
    println!("getting 1 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(2);
    println!("getting 2 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(3);
    println!("getting 3 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(4);
    println!("getting 4 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(5);
    println!("getting 5 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(6);
    println!("getting 6 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(7);
    println!("getting 7 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(8);
    println!("getting 8 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(9);
    println!("getting 9 took: {} milliseconds", timer.elapsed().as_millis());
    timer = std::time::Instant::now();
    cache.get(40);
    println!("getting 40 took: {} milliseconds", timer.elapsed().as_millis());
}
