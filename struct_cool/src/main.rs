use core::f32::consts::PI;

struct Point <Type> {
    x: Type,
    y: Type,
}

struct Circle {
    center: Point<f32>,
    radius: f32,
}

impl Circle {
    // method
    fn circumference(&self) -> f32 {
        self.radius * 2.0 * PI
    }
    // method
    fn area(&self) -> f32 {
        self.radius * self.radius * PI
    }
}

fn main() {
    println!("Hello, world!");
    let my_circle : Circle = Circle { center : Point { x : 1.2, y : 3.4 },
                                      radius : 4.0, };

    println!("circumference: {}", my_circle.circumference());
    println!("area: {}", my_circle.area());
    println!("x,y: {:?}", (my_circle.center.x, my_circle.center.y));
}
