pub use paste::paste;

#[macro_export]
macro_rules! def {
    ($name:ident; $(fn ($($arg:ident : $type:ty),* $(,)?) $(-> $ret:ty)? $body:block);+ $(;)?) => {
        $crate::paste! {
            $crate::helper! {
                ($d:tt) => {
                    macro_rules! $name {
                        ($d($d args:expr),*) => {
                            ($d($d args),*).[<fn_ $name>]()
                        }
                    }
                }
            }
            trait [<Fn $name:camel>] {
                type [<Fn $name:camel Return>];
                fn [<fn_ $name>](&self) -> Self::[<Fn $name:camel Return>];
            }
            $(  
                impl [<Fn $name:camel>] for ($($type),*) {
                    type [<Fn $name:camel Return>] = $crate::helper_type_default!($($ret)?, ());
                    fn [<fn_ $name>](&self) -> Self::[<Fn $name:camel Return>] {
                        let ($($arg),*) = self;
                        $body
                    }
                }
             )*
        } 
    };
    (pub $name:ident; $(fn ($($arg:ident : $type:ty),* $(,)?) $(-> $ret:ty)? $body:block);+ $(;)?) => {
        $crate::paste! {
            $crate::helper! {
                ($d:tt) => {
                    #[macro_export]
                    macro_rules! $name {
                        ($d($d args:expr),*) => {
                            ($d($d args),*).[<fn_ $name>]()
                        }
                    }
                }
            }
            pub trait [<Fn $name:camel>] {
                type [<Fn $name:camel Return>];
                fn [<fn_ $name>](&self) -> Self::[<Fn $name:camel Return>];
            }
            $(  
                 impl [<Fn $name:camel>] for ($($type),*) {
                    type [<Fn $name:camel Return>] = $crate::helper_type_default!($($ret)?, ());
                    fn [<fn_ $name>](&self) -> Self::[<Fn $name:camel Return>] {
                        let ($($arg),*) = self;
                        $body
                    }
                }
             )*
        } 
    };
}

#[macro_export]
macro_rules! helper {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

#[macro_export]
macro_rules! helper_type_default {
    ($type:ty, $default:ty) => {
       $type 
    };
    (,$type:ty) => {
        $type
    };
}

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


