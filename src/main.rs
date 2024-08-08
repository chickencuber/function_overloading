use function_overloading::def;

def! {
    foo,
    fn (a: u32, b: u32) -> u32 {
        return a + b;
    },
    fn (a: &str, b: &str) -> usize {
        return a.len() + b.len();
    },
    fn (a: String) {
        println!("{}", a);
    },
}

fn main() {
    println!("{}", foo!("1", "2"));
    println!("{}", foo!(1, 2));
    foo!(String::from("hello"));
}
