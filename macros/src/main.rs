macro_rules! say_hi {
    ($name:literal) => {
        println!("hi {:?}", $name);
    };
    () => {
        println!("no arg given!");
    };
}

fn test_hi() {
    say_hi!("uwu");
    say_hi!();
    say_hi!(1);
    say_hi!(1.2);
}

macro_rules! create_fn {
    ($fn_name:ident) => {
        fn $fn_name() {
            println!("function {:?}()", stringify!($fn_name));
        }
    };
}

create_fn!(bruv);
create_fn!(chav);

fn test_create_fn() {
    bruv();
    chav();
}

macro_rules! eval_expr {
    ($expr_name:expr) => {
        println!("{:?} = {:?}",
            stringify!($expr_name),
            $expr_name);
    };
}

fn test_eval_expr() {
    eval_expr!(1 + 2);
    eval_expr!(100.0 + 3.14159 * 2.0);
}


fn main() {
    test_hi();
    test_create_fn();
    test_eval_expr();
}
