#![macro_use]

#[macro_export]
macro_rules! data {
    ($structtype:ident {$($var:ident: $vartype:ty = $val:expr)+}) => {
        #[allow(missing_copy_implementations)]
        pub struct $structtype {
            $(
                pub $var: $vartype,
            )+
        }

        impl $structtype {
            pub fn new() -> $structtype {
                $(
                    let $var = $val;
                )+
                $structtype {
                    $(
                        $var: $var,
                    )+
                }
            }
        }
    }
}
