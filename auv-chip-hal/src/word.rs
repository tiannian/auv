pub trait Word {}

macro_rules! define_word {
    ($word:ty) => {
        impl Word for $word {}
    };
}

define_word!(u8);
define_word!(u16);
define_word!(u32);
define_word!(u64);
define_word!(u128);

