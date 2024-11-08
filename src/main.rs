use function_overloading::{foo, FnFoo};

fn main() {
    println!("{}", foo!("1", "2"));
    println!("{}", foo!(1, 2));
    println!("{}", foo!(1.0, 2.0));
    foo!(String::from("hello"));
    foo!("world");
}
