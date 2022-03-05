pub fn run() {
    // for n in 1..101 {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("{} is fold of 15", n);
        } else if n % 3 == 0 {
            println!("{} is flod of 3", n);
        } else if n % 5 == 0 {
            println!("{} is flod of 5", n);
        } else {
            println!("{} normal", n);
        }
    }
    // iteration from collection
    // 1. iter   just borrow , useable after for loop
    // 2. into_iter take/consumees/move from collection ,unuseable after for loop
    // 3. iter_mut borrow and mutable the item, modified after for loop
    // let  names = vec!["Bob", "David", "Frank", "Ferris"];
    let mut names = vec!["Bob", "David", "Frank", "Ferris"];
    // for name in names.iter() {
    // for name in names.into_iter() {
    // for name in names {
    // default into_iter
    for name in names.iter_mut() {
        *name = match name {
            // &"David" => println!("match {}", name),
            //  "David" => println!("match {}", name),
            &mut "David" => "David mudified",
            _ => name,
        }
    }
    // println!("names: {:?}", names);
    println!("names: {:?}", names);
}
