macro_rules! function_generator {
    ($name_func:ident,$type:ty) => {
        fn $name_func(value: $type) {
            println!(
                "doing dynamics for function {} for parameter {:?}",
                stringify!($name_func),
                value
            );
        }
    };
}
function_generator!(integer_adder, i32);
function_generator!(string_adder, &str);

macro_rules! my_print{
    ($val:expr) => {
        println!("my print macro: {}", $val);
    };
}


fn main() {
    println!("hello , world!");
    my_print!("This is a test");
    my_print!(42);
    integer_adder(10);
    string_adder("hello");
}
