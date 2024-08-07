# function overloading
this is a rust library that adds function overloading through rust macros  
it only supports normal function syntax(no async, generics, lifetimes, ect...)  

```rust

def!{
    foo, //the name of the function
    fn (a: u32, b: u32) -> () { // you have to explicitly put the return value
        println!("{}", a + b)
    }
}

``` 
I am planning on making the syntax better and adding generics and other features  
I am also planning on using proc macros to make it have '-> ()' implisitly

