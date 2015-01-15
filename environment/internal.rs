//May god's dice roll in your favour, Kimundi.
macro_rules! lazy_static {
    (static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        lazy_static!(PRIV static ref $N : $T = $e; $($t)*);
    };
    (pub static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        lazy_static!(PUB static ref $N : $T = $e; $($t)*);
    };
    ($VIS:ident static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        lazy_static!(MAKE TY $VIS $N);
        impl ::std::ops::Deref for $N {
            type Target = $T;
            fn deref<'a>(&'a self) -> &'a $T {
                use std::sync::{Once, ONCE_INIT};
                use std::mem::transmute;

                #[inline(always)]
                fn require_sync<T: Sync>(_: &T) { }

                unsafe {
                    static mut __static: *const $T = 0 as *const $T;
                    static mut __ONCE: Once = ONCE_INIT;
                    __ONCE.call_once(|| {
                        __static = transmute::<Box<$T>, *const $T>(box() ($e));
                    });
                    let static_ref = &*__static;
                    require_sync(static_ref);
                    static_ref
                }
            }
        }
        lazy_static!($($t)*);
    };
    (MAKE TY PUB $N:ident) => {
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct $N {__private_field: ()}
        pub static $N: $N = $N {__private_field: ()};
    };
    (MAKE TY PRIV $N:ident) => {
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct $N {__private_field: ()}
        static $N: $N = $N {__private_field: ()};
    };
    () => ()
}
