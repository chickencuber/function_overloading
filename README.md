# function overloading
this is a rust library that adds function overloading through rust macros  
it only supports normal function syntax(no async, generics, lifetimes, ect...)  

```rust

def!{
    foo, //the name of the function
    fn (a: u32, b: u32) -> () { // you have to explicitly put the return value
        println!("{}", a + b);
    }, //the , is not optional
    fn (s: &str) -> () {
        println!("{}", s);
    },
    fn (s: char) -> String { //you can also add other return values
       return s.to_string(); 
    } //you can add a ',' at the end, but you dont have to
}

``` 

you can use the function by calling it as a macro

```rust

    fn main() {
        foo!(1, 2); // 3
        foo!("bar"); // bar
        let bar: String = foo!('a');
    }

```

I am planning on making the syntax better and adding generics and other features  
I am also planning on using proc macros to make it have '-> ()' implicitly

