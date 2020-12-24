struct Person {
    name: Option<String>,
    age: Option<i32>,
    height: Option<i32>,
}

fn get_name(person: &Person) -> Option<&String> {
    match &person.name {
        Some(name) => Some(&name),
        None => None,
    }
}

fn main() {
    let mut jack = Person {
        name: None,
        age: Some(1),
        height: Some(2),
    };

    println!("name: {:?}", get_name(&jack));
    jack.name = Some(String::from("Jack"));
    println!("name: {:?}", get_name(&jack));
    println!("age: {:?}", jack.age);
    println!("height: {:?}", jack.height);
    println!("ref age: {:?}", &jack.age);
    println!("ref height: {:?}", &jack.height);
}
