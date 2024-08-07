use function_overloading::def;

def! {
    test,
    fn (a: u32, b: u32) -> u32 {
        return a + b;
    },
    fn (a: &str, b: &str) -> usize {
        return a.len() + b.len();
    }
}

fn main() {
    println!("{}", test!("1", "2"));
}
