fn add(x: i64, y: i64) -> i64 {
    return x + y;
}

fn main() {
    let multiply = |x: i64, y: i64| {
        return x * y;
    };

    let x = multiply(2, 2);

    println!("x was {}", x);

    let x = add(1, 2);

    println!("x was {}", x);

    //Need to user generics because function size must be known at compile time
    fn is_even<F>(calc_function: F, x: i64, y: i64) -> bool
    where
        F: Fn(i64, i64) -> i64,
    {
        calc_function(x, y) % 2 == 0
    }

    let x = is_even(add, 4, 4);

    println!("add x was {}", x);

    let x = is_even(multiply, 3, 4);

    println!("add x was {}", x);
}
