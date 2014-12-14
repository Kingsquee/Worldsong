#![macro_escape]

#[macro_export]
macro_rules! data {
    ($($structname:ident: $structtype:ident {$($var:ident: $vartype:ty = $val:expr)+})+) => {

        $(
            pub struct $structtype {
                $(
                    pub $var: $vartype,
                )+
            }
        )+

        $(
            impl $structtype {
                pub fn new() -> $structtype{
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
        )+

        #[no_mangle]
        #[allow(dead_code)]
        pub struct Data {
            $(
                pub $structname: $structtype,
            )+
        }

        #[no_mangle]
        #[allow(dead_code)]
        impl Data {
            pub fn new() -> Data {
             let data = Data {
                $(
                    $structname: $structtype::new(),
                )+
             };
             data
            }
        }
    }
}

#[macro_export]
macro_rules! schedule {
    ($($process_name:ident($($param:ident),+))+) => {
        mod _hack {
            extern crate common;
            $(
                extern crate $process_name;
            )+
        }

        pub fn execute(data: &mut common::data::Data) {
            use common::data::Data;
            $(
                _hack::$process_name::execute(
                    $(&mut data.$param),+
                );
            )+
        }
    }
}
