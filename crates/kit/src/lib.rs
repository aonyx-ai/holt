/// Compile-time check: the named Props builder accepts both `&str` and `String` for `class`.
#[cfg(test)]
macro_rules! assert_class_prop {
    ($($Props:ty),+ $(,)?) => {
        $(
            let _ = <$Props>::builder().class("lit");
            let _ = <$Props>::builder().class(String::from("dyn"));
        )+
    };
}

pub mod behavior;
pub mod testing;
pub mod visual;
