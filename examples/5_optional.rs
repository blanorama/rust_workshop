fn main() {
    let option = Some(1);

    //Actually the value in the option gets copied on unwrap
    println!("{:#?} has value {}", option, option.unwrap());

    let option = option.map(|value| value.to_string());

    //Need too use as_ref because String can only have one owner
    println!("{:#?} has value {}", option, option.as_ref().unwrap());

    let option: Option<i32> = Option::None;

    println!("{:#?} has value {}", option, option.unwrap_or(2));

    match option {
        Some(value) => println!("Got some {}", value),
        None => println!("Got None"),
    }
}
