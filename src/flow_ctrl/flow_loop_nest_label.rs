pub fn run() {
    'outer: loop {
        println!("Entered the outer loop");
        // 'inner: loop {
        loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("unreacheable here,directly beak to outer loop by 'outer lebal");
    }
    println!("Exited the outer loop");
}
