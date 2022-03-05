pub fn run() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop express break return value = {} ", result);
    assert_eq!(result, 20);
}
