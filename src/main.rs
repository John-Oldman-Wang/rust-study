fn struct_study() {
    let a = Point { x: 1, y: 2 };
    println!("a.x {}, a.y {}", a.x, a.y);

    struct Point {
        x: i32,
        y: i32,
    }

    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let mut top_left = Point { x: 0, y: 0 };

    top_left.x = 3;
    top_left.y = 10;
    let bottom_right = Point { x: 12, y: 2 };

    let mut rect = Rectangle {
        top_left,
        bottom_right,
    };

    rect.top_left.x = 4;


    println!("rect area {}", get_rect_area(rect));
    println!("1,2,3,4 area {}", get_rect_area(make_rect(1, 2, 3, 4)));
    println!("1,1,3,4 area {}", get_rect_area(make_rect(1, 1, 3, 4)));

    fn make_rect(x1: i32, y1: i32, x2: i32, y2: i32) -> Rectangle {
        return Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        };
    }

    fn get_rect_area(r: Rectangle) -> i32 {
        let Rectangle {
            top_left,
            bottom_right,
        } = r;
        let Point { x: x1, y: y1 } = top_left;
        let Point { x: x2, y: y2 } = bottom_right;

        return (x2 - x1) * (y2 - y1);
    }
}

fn main() {
    println!("hello wolrd");
    struct_study();
}
