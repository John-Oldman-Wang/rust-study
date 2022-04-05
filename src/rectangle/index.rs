pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Rectangle {
        return Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        };
    }

    fn get_xy(&self) -> [i32; 4] {
        let Rectangle {
            top_left,
            bottom_right,
        } = self;
        let Point { x: x1, y: y1 } = top_left;
        let Point { x: x2, y: y2 } = bottom_right;
        return [*x1, *y1, *x2, *y2];
    }

    pub fn area(&self) -> i32 {
        let [x1, y1, x2, y2] = self.get_xy();
        return (x2 - x1) * (y2 - y1);
    }

    pub fn get_top_right(&self) -> Point {
        let [x1, _, _, y2] = self.get_xy();
        return Point { x: x1, y: y2 };
    }

    pub fn clone(&self) -> Rectangle {
        let [x1, y1, x2, y2] = self.get_xy();
        return Rectangle::new(x1, y1, x2, y2);
    }
}
