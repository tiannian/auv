#![feature(trace_macros)]

use aurt_macro::cimport;

trace_macros!(true);
extern "C" {
    #[cimport]
    fn test(a: u8, b: &[u8]) -> usize;
}
trace_macros!(false);

fn main() {}

// #[aurt::]
// pub fn test() -> Vec<u8> {}


