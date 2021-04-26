use aurt_macro::import;

extern "C" {
    #[import]
    fn test(a: u8, b: &[u8]) -> usize;
}

fn main() {}

// #[aurt::]
// pub fn test() -> Vec<u8> {}


