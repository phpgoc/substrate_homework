trait Shape {
    fn area(&self) -> f32;
}

struct Circle {
    radius: f32,
}
impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }
}
struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}
impl Shape for Triangle {
    fn area(&self) -> f32 {
        let p = (self.a + self.b + self.c) / 2.0;
        (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt()
    }
}
struct Rectangle {
    length :f32,
    width: f32,
}
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }
}
fn area<T:Shape>(t : T) ->f32{
    t.area()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn do_test() {
        let circle = Circle{ radius: 2.0 };
        println!("circle area : {}",area(circle));
        let triangle = Triangle{
            a: 3.0,
            b: 4.0,
            c: 5.0
        };
        println!("triangle area : {}",area(triangle));

        let rectangle = Rectangle{ length: 3.0, width: 2.0 };
        println!("rectangle area : {}",area(rectangle));

    }

}