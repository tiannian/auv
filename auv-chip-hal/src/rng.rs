use core::fmt::Debug;

use crate::Word;

pub trait Rng {
    type Word: Word;

    type Error: Debug;

    fn read(&self) -> Result<Self::Word, Self::Error>;
}
