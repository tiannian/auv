pub trait Rng {
    type Word;

    type Error;

    fn read(&self) -> Result<Self::Word, Self::Error>;
}
