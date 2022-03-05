enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

pub fn run() {
    let number = 13;
    println!("Tell me abount {}", number);
    match number {
        1 => println!("one!"),
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {} ", boolean, binary);

    // destruct
    //1. tuples
    let triple = (2, -2, 4);
    println!("Tell me abount {:?}", triple);
    match triple {
        (0, y, z) => println!("First is `0`,`y` is {:?} , and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1`, other do't care"),
        _ => println!("do't care anything"),
    }
    //2. arrays/slices
    let array = [4, -2, 6];
    match array {
        [0, second, third] => println!("array[0]=0,array[1]={},array[2]={}", second, third),
        [1, _, third] => println!("array[0]=1,array[2]={} and array[1] is ignore", third),
        [-1, second, ..] => println!("array[0]=-1.array[1]={} and rest is ignore", second),
        [3, secoind, tail @ ..] => println!(
            "array[0]=3,second is {} and rest is array or slice {:?}",
            secoind, tail
        ),
        [first, middle @ .., last] => println!("first={},middle={:?},last={}", first, middle, last),
        _ => (),
    }
    //3. enum
    let color = Color::RGB(122, 12, 40);
    println!("whtat color is it?");
    match color {
        Color::Red => println!("The color is Red"),
        Color::RGB(r, g, b) => {
            println!("r={},g={},b={}", r, g, b)
        }
        _ => (),
    }
    //4. pointers/ref
    // * deref
    // & destruct with match
    let reference = &4;
    match reference {
        &val => println!("Get a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("Get a value vie dereferencing:{:?}", val),
    }
    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("Get a reference to a value:{:?}", r),
    }
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10, `mut_value`:{:?}", m);
        }
    }
    //5. structs
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (9, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1 ,b={},y={}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i= {:?}", i),
        Foo { y, .. } => println!("y is {},do't care other", y),
    }

    // guards 警卫
    let pair = (2, -2);
    println!("Tell me abount {:?}", pair);
    match pair {
        (x, y) if x == y => println!("these are twins"),
        (x, y) if x + y == 0 => println!("plus sum is zero"),
        (x, _) if x % 2 == 1 => println!("the first is odd"),
        _ => println!("other pipes,do't care"),
    }

    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("gt Zero"),
        _ => (), //unreachable ,but must writee,because match guards no test all pipes
    }
    // binding
    println!("Please tell me what type of person you are");
    match age() {
        0 => println!("Zero"),
        n @ 1..=12 => println!("Child: age is {:?}", n),
        n @ 13..=18 => println!("teen of age {:?}", n),
        _ => (),
    }

    // binding on option destruct
    match some_number() {
        Some(n @ 42) => println!("binding Answer : {}!", n),
        Some(n) => println!("not care ... {}", n),
        _ => (),
    }
    // if let
    let number = Some(7);
    if let Some(i) = number {
        println!("matched i is {:?}", i);
    }

    // while let
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 8 {
            println!("gt 8,quit");
            optional = None;
        } else {
            println!(" i is {:?},try again", i);
            optional = Some(i + 1);
        }
    }
}
fn age() -> u32 {
    15
}
fn some_number() -> Option<u32> {
    Some(42)
}
