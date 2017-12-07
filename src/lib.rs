#![feature(test)]

extern crate test;
pub mod parser;

#[macro_use]
extern crate nom;

pub use parser::parse;
pub use parser::comparison::evaluate;
pub use parser::{NONE, BIN, OCT, HEX, AS_DEGREES};

#[cfg(test)]
mod tests {
    use parse;
    use evaluate;
    use NONE;
    use test::Bencher;

    #[bench]
    fn bench_parse(b: &mut Bencher) {
         b.iter(|| parse("sin(15+4^2+3*2*cos(4))", NONE));
    }

    #[test]
    fn now() {
        println!("{}", evaluate("cos(4°) == sin(86°)"));
    }

    #[test]
    fn it_works() {
    }
}
