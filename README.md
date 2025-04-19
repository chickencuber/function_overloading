# Function Overloading

This is a Rust library that adds function overloading through Rust macros. It supports normal function syntax but does not currently support advanced features like async, generics, or lifetimes.

## Usage

Define overloaded functions using the `def!` macro:

```rust
def! {
    foo; // The name of the function
    fn (a: u32, b: u32) {
        println!("{}", a + b);
    }; // The comma is not optional
    fn (s: &str) -> () { // you dont need the return type explicitly
        println!("{}", s);
    };
    fn (s: char) -> String { // You can also have different return types
       return s.to_string();
    } // You can add a comma at the end, but it's not required
}
```

You can use the overloaded function by calling it as a function with the args wrapped in a tuple:

```rust
fn main() {
    foo((1, 2)); // Outputs: 3
    foo(("bar")); // Outputs: bar
    let bar: String = foo(('a'));
}
```

## Future Plans

- [ ] **Additional Features**: Adding support for generics, lifetimes, async functions, etc.
- [x] **Proc Macros**: Using proc macros to make `-> ()` implicit.
    - turns out I didn't need a proc macro for this
- [x] **Add to crates.io**
