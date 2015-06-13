#[macro_export]
macro_rules! schedule {
    ($($process_name:ident($($param:ident),*))*) => {
        mod _hack {
            extern crate state;
            $(
                extern crate $process_name;
            )*
        }
        #[allow(unused_variables)]
        pub fn execute(data: &mut _hack::state::Data) {
            $(
                _hack::$process_name::execute($(&mut data.$param),*);
            )*
        }
    };
}