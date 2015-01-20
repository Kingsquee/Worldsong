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

#[macro_export]
macro_rules! schedule {
    ($($process_name:ident($($param:ident),+))+) => {
        mod _hack {
            extern crate state;
            $(
                extern crate $process_name;
            )+
        }

        pub fn execute(data: &mut _hack::state::Data) {
            $(
                _hack::$process_name::execute(
                    $(&mut data.$param),+
                );
            )+
        }
    }
}
