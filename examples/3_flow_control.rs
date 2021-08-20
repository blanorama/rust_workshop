fn main() {
    if true {
        println!("true strory");
    } else {
        println!("if u see that, rust is broken");
    }

    // there is no ternary operator in rust but u can use if/else
    let x = if true == false { 1 } else { 2 };
    println!("x: {}", x);

    // every block in rust can return something if it is not ended by a ;
    let x = { "block return" };
    println!("x: {}", x);

    let x = match x {
        "block" => 1,
        "block return" => 2,
        _ => 3,
    };
    println!("x: {}", x);

    let x = match x {
        1 | 2 => "block",
        3 => "block return",
        _ => "default",
    };
    println!("x: {}", x);

    let tuple = (1, 3);
    match tuple {
        (1, 3) => {
            println!("first is 1 second is 3")
        }
        (1, _) => {
            println!("first is 1")
        }
        _ => {
            println!("none matched")
        }
    };

    #[derive(Debug)]
    enum RemoteValue {
        NotLoaded(),
        Loading(String),
        Loaded(String),
        Error(String),
    }

    let x = RemoteValue::Loaded("Ok".into());

    match x {
        RemoteValue::Error(x) => println!("{}", x),
        value => println!("{:#?}", value),
    }

    let mut x = 1;

    loop {
        if x % 3 == 0 {
            break;
        }
        println!("loop x was: {}", x);

        x = x + 1;
    }

    let mut x = 1;

    while x % 3 != 0 {
        println!("while x was: {}", x);
        x = x + 1;
    }

    for x in 1..3 {
        println!("for x was: {}", x)
    }

    let vector = vec!["One", "Two", "Three"];

    for elem in vector {
        println!("for elem was: {}", elem)
    }

    let mut remote = RemoteValue::NotLoaded();

    while let RemoteValue::NotLoaded() = remote {
        match remote {
            RemoteValue::NotLoaded() => remote = RemoteValue::Loading("google.de".into()),
            _ => {}
        }
    }

    println!("value: {:#?}", remote);

    if let RemoteValue::Loading(x) = remote {
        println!("{}", x);
    }
}
