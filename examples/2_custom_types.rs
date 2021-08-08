static mut LANGUAGE: &str = "Rust";
const MEANING_OF_LIVE: i8 = 42;

// #[derive(Debug)] needed for generating code so that you can print it for debugging
#[derive(Debug)]
struct Point {
    x: i64,
    y: u64,
}

#[derive(Debug)]
struct Rectangle(Point, Point);

#[derive(Debug)]
struct EquilateralTriangle(Point, Point);

#[derive(Debug)]
enum Shape {
    Rectangle(Rectangle),
    EquilateralTriangle(EquilateralTriangle),
}

fn main() {
    type Text = String;

    let what: Text = Text::from("what");

    println!("{}", what);

    let what: Text = "what".into(); // alternative to String::from

    println!("{}", what);

    let top_left = Point { x: 0, y: 0 };
    let botom_right = Point { x: 0, y: 0 };

    let rectangle = Rectangle(top_left, botom_right);

    println!("{:#?}", rectangle);

    let rectangle = Shape::Rectangle(rectangle);

    println!("{:#?}", rectangle);

    let top_left = Point { x: 0, y: 0 };
    let botom_right = Point { x: 0, y: 0 };

    let equilateral_triangle =
        Shape::EquilateralTriangle(EquilateralTriangle(top_left, botom_right));

    println!("{:#?}", equilateral_triangle);

    println!("{:#?}", MEANING_OF_LIVE);

    unsafe {
        println!("{:#?}", LANGUAGE);
        LANGUAGE = "java";
        println!("{:#?}", LANGUAGE);
    }
}
