pub fn run() {
    let n = -50;
    if n < 0 {
        print!("{} is less than zero", n);
    } else if n > 0 {
        print!("{} si biger than zero", n);
    } else {
        print!("{} is zero", n);
    }

    let n_big = if n < 10 && n > -10 {
        println!(",and is small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is big number, halve the number");
        n / 2
    };
    println!("{} -> {}", n, n_big);
}
