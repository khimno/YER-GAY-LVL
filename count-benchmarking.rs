use std::time::Instant;
use std::io;
fn main() {
    let now = Instant::now();
    for i in 1..100001 {
        println!("YER GAY LVL: {}", i);
    }
    println!("Took {} seconds to run 100000 times", now.elapsed().as_secs());
    io::stdin().read_line(&mut String::new()).unwrap();
}
