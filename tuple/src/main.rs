fn println(tuple: (i32, &str, f32)) {
    println!("i32: {}, str: {}, f32: {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple);
}

fn main() {
    let x: (i32, &str, f32) = (420, "owo", 0.69);
    println(x);
}
