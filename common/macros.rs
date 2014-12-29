#![macro_escape]

#[macro_export]
macro_rules! data {
    ($structname:ident $structtype:ident {$($var:ident: $vartype:ty = $val:expr)+}) => {
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

macro_rules! statedata {
    ($($substructname:ident: $substructtype:ident {$($var:ident: $vartype:ty = $val:expr)+})+) => {

        $(
            pub struct $substructtype {
                $(
                    pub $var: $vartype,
                )+
            }
        )+

        $(
            impl $substructtype {
                pub fn new() -> $substructtype{
                    $(
                        let $var = $val;
                    )+

                    $substructtype {
                        $(
                            $var: $var,
                        )+
                    }
                }
            }
        )+

        pub struct Data {
            $(
                pub $substructname: $substructtype,
            )+
        }

        impl Data {
            pub fn new() -> Data {
             let data = Data {
                $(
                    $substructname: $substructtype::new(),
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

        pub fn execute(data: &mut common::state::Data) {
            use common::state::Data;
            $(
                _hack::$process_name::execute(
                    $(&mut data.$param),+
                );
            )+
        }
    }
}
