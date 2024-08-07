#[macro_export]
macro_rules! def {
    ($name:ident, $(fn ($($arg:ident : $type:ty),* $(,)?) -> $ret:ty $body:block),+ $(,)?) => {
        paste::paste! {
            function_overloading::helper! {
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
                    type [<Fn $name:camel Return>] = $ret;
                    fn [<fn_ $name>](&self) -> Self::[<Fn $name:camel Return>] {
                        let ($($arg),*) = *self;
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

