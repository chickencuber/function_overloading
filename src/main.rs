use function_overloading::def;

def! {
    pub foo;
    fn (a: u32, b: u32) -> u32 {
        return a + b;
    };
    fn (a: &str, b: &str) -> usize {
        return a.len() + b.len();
    };
    fn (a: String) {
        println!("{}", a);
    };
    fn (a: &str) {
        println!("{}", a);
    };
    fn(a: f32, b: f32) -> f32 {
        return a * b;
    };
}



fn main() {
    println!("{}", foo(("1", "2")));
    println!("{}", foo((1, 2)));
    println!("{}", foo((1.0, 2.0)));
    foo(String::from("hello"));
    foo("world");
}
