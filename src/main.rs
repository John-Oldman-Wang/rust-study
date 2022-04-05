#[path = "rectangle/index.rs"]
mod rectangle;
use rectangle::Rectangle;

fn struct_study() {
    let mut rect = rectangle::Rectangle::new(1, 1, 3, 4);

    // change top_left x
    rect.top_left.x = 0;

    let rect1 = Rectangle::new(1, 2, 3, 4);

    let mut rect2 = rect1.clone();

    rect2.top_left.x = 2;

    println!("rect area {}", rect.area());
    println!("rect1 area {}", rect1.area());
    println!("rect2 area {}", rect2.area());
    let p = rect.get_top_right();
    let p2 = rect2.get_top_right();
    println!("p.x {}", p.x);
    println!("p2.x {}", p2.x);
}

mod math {
    pub fn sin() {
        println!("sin function be call")
    }
}

use math::sin;

fn main() {
    println!("hello wolrd");
    struct_study();
    math::sin();
    sin();
}
