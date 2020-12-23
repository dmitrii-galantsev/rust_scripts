use std::collections::HashMap;

fn main() {
    // tuple
    let tuple : (i32, f32) = (420, 0.69);
    println!("{:?}", tuple);

    // array
    let array : [i32; 4] = [0, 1, 2, 3];
    println!("{:?}", array);

    // Hash Map
    let mut map : HashMap<i32, f32> = HashMap::new();
    map.insert(0, 0.0);
    map.insert(1, 0.1);
    map.insert(2, 0.2);
    map.insert(3, 0.3);
    map.insert(4, 0.4);
    println!("{:?}", map);
    println!("hashmap for-loop");
    for element in map.clone() {
        println!("    -> {:?}", element);
    }

    println!("hashmap iter");
    for (k, v) in map.iter() {
        println!("    -> ({:?}:{:?})", k, v);
    }

}
