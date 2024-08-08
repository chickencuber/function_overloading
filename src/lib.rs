pub use paste::paste;

#[macro_export]
macro_rules! def {
    ($name:ident, $(fn ($($arg:ident : $type:ty),* $(,)?) $(-> $ret:ty)? $body:block),+ $(,)?) => {
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
                    type [<Fn $name:camel Return>] = $crate::helper_type_defualt!($($ret)?, ());
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
macro_rules! helper_type_defualt {
    ($type:ty, $default:ty) => {
       $type 
    };
    (,$type:ty) => {
        $type
    };
}

