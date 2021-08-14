use std::{error::Error, io::stdin};

fn main() {
    let string_to_parse = "1.3";

    let parse_int_result = string_to_parse.parse::<i32>();

    println!("tried to parse 1.3 as int and got {:#?}", parse_int_result);

    println!(
        "tried to parse 1.3 as int and got {:#?} as option",
        parse_int_result.ok()
    );

    let parse_float_result = string_to_parse.parse::<f32>();

    println!(
        "tried to parse 1.3 as float and got {:#?}",
        parse_float_result
    );

    println!(
        "tried to parse 1.3 as float and got {:#?} as option",
        parse_float_result.ok()
    );

    read_input_and_add_them_together()
        .map(|(a, b, c, result)| println!("\n{} + {} + {} = {}", a, b, c, result))
        .map_err(|err| println!("{:#?}", err))
        .ok();
}

fn read_input_and_add_them_together() -> Result<(f64, f64, f64, f64), Box<dyn Error>> {
    let number_a = read_float_form_stdin_unsafe();

    let number_b = read_float_form_stdin_smart()?;

    let number_c = read_float_form_stdin_save()?;

    Ok((
        number_a,
        number_b,
        number_c,
        (number_a + number_b + number_c),
    ))
}

fn read_float_form_stdin_unsafe() -> f64 {
    let mut line_buffer = String::new();

    println!("Enter a number and hit enter");

    stdin().read_line(&mut line_buffer).unwrap();

    line_buffer.replace("\n", "").parse::<f64>().unwrap()
}

fn read_float_form_stdin_save() -> Result<f64, Box<dyn Error>> {
    let mut line_buffer = String::new();

    println!("Enter a number and hit enter");

    let result = stdin().read_line(&mut line_buffer);

    if result.is_err() {
        return Err(Box::new(result.unwrap_err()));
    }

    let result = line_buffer.replace("\n", "").parse::<f64>();

    match result {
        Ok(x) => Ok(x),
        Err(err) => Err(Box::new(err)),
    }
}

fn read_float_form_stdin_smart() -> Result<f64, Box<dyn Error>> {
    let mut line_buffer = String::new();

    println!("Enter a number and hit enter");

    stdin().read_line(&mut line_buffer)?;

    Ok(line_buffer.replace("\n", "").parse::<f64>()?)
}
