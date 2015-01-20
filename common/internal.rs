macro_rules! define_str (
    ($name: ident, $($arg: expr), +)
    =>
    (
        macro_rules! $name (
            () => (concat!($($arg), +))
        );
        const $name: &'static str = $name!();
    );
);
