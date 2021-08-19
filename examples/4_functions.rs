use std::fmt::Display;

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

    println!("My Point is {}", Point2d::new(1, 2))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

impl Point2d {
    pub fn new(x: i32, y: i32) -> Self {
        Point2d { x, y }
    }
}

impl Display for Point2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X: {}, Y: {}", self.x, self.y)
    }
}
