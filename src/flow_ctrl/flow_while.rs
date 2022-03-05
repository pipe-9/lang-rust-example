pub fn run() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("{} is fold of 15", n);
        } else if n % 3 == 0 {
            println!("{} is flod of 3", n);
        } else if n % 5 == 0 {
            println!("{} is flod of 5", n);
        } else {
            println!("{} normal", n);
        }
        n += 1;
    }
}
