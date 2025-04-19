pub use paste::paste;

#[macro_export]
macro_rules! def {
    ($name:ident; $(fn ($($arg:ident : $type:ty),* $(,)?) $(-> $ret:ty)? $body:block);+ $(;)?) => {
        $crate::paste! {
            trait [<Fn $name:camel>] {
                type [<Fn $name:camel Return>];
                fn [<fn_ $name>](&self) -> Self::[<Fn $name:camel Return>];
            }
            fn $name<T: [<Fn $name:camel>]>(args: T) -> T::[<Fn $name:camel Return>] {
                return args.[<fn_ $name>]();
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
            pub trait [<Fn $name:camel>] {
                type [<Fn $name:camel Return>];
                fn [<fn_ $name>](&self) -> Self::[<Fn $name:camel Return>];
            }
            pub fn $name<T: [<Fn $name:camel>]>(args: T) -> T::[<Fn $name:camel Return>] {
                return args.[<fn_ $name>]();
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
macro_rules! helper_type_default {
    ($type:ty, $default:ty) => {
       $type 
    };
    (,$type:ty) => {
        $type
    };
}

