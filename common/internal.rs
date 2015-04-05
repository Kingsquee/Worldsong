macro_rules! define_str (
    ($name: ident, $($arg: expr), +)
    =>
    (
        macro_rules! $name (
            () => (concat!($($arg), +))
        );
        #[allow(dead_code)]
        const $name: &'static str = $name!();
    );
);
