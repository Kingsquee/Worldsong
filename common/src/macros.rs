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
                    $structtype {
                        $(
                            $var: $val,
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
             let mut data = Data {
                $(
                    $structname: $structtype::new(),
                )+
             };
             initialize(&mut data);
             data
            }
        }
    }
}
