struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // in method definition
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    // implement method only on point<f32> values
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    // multiple generic types
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    println!("p.x = {}", integer.x());

    let float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    let str_char = Point2 { x: "kage", y: 'c' };

    let p3 = integer_and_float.mixup(str_char);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
