#![feature(test)]

extern crate test;
pub mod parser;

#[macro_use]
extern crate nom;

#[cfg(test)]
mod tests {
    use parser::hierarchy::{expr, parse, AS_DEGREES, NONE, HEX, OCT, BIN};
    use parser::comparison::eval;
    use nom::IResult;
    use test::Bencher;

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| expr(b"sin(15+4^2+3*2*cos(4))"));
    }

    #[test]
    fn now() {
        println!("{}", parse("0b1011*(0b11-0b1)^0b101", HEX | AS_DEGREES))
    }

    #[test]
    fn it_works() {
        assert_eq!(expr(b"4"), IResult::Done(&b""[..], 4f64));
        assert_eq!(expr(b"4+4"), IResult::Done(&b""[..], 8f64));
        assert_eq!(expr(b"5*5"), IResult::Done(&b""[..], 25f64));
        assert_eq!(expr(b"4*(4+4)"), IResult::Done(&b""[..], 32f64));
        assert_eq!(expr(b"4*(4-4)"), IResult::Done(&b""[..], 0f64));
        assert_eq!(expr(b"4-4"), IResult::Done(&b""[..], 0f64));
        assert_eq!(expr(b"4+-4"), IResult::Done(&b""[..], 0f64));
        assert_eq!(expr(b"-4+4"), IResult::Done(&b""[..], 0f64));
        assert_eq!(expr(b"4^3"), IResult::Done(&b""[..], 64f64));
        assert_eq!(expr(b"4^(3-1)"), IResult::Done(&b""[..], 16f64));
        assert_eq!(expr(b"-3^(2+3^2)"), IResult::Done(&b""[..], -177147f64));
        assert_eq!(expr(b"(0x14)^(2+0x1)"), IResult::Done(&b""[..], 8000f64));
        assert_eq!(expr(b"0b11011"), IResult::Done(&b""[..], 27f64));
        assert_eq!(
            expr(b"0b1011*(0b11-0b1)^0b101"),
            IResult::Done(&b""[..], 352f64)
        );
        assert_eq!(
            expr(b"sin((86+(cos(0\xC2\xB0)+1)\xC2\xB0)^2)"),
            IResult::Done(&b""[..], 0.4012299626605139f64)
        );
        assert_eq!(
            expr(b"4*sin((86+((cos(0)+1)^2))\xC2\xB0)"),
            IResult::Done(&b""[..], 4f64)
        );
        assert_eq!(
            expr(b"sin(15+4^2+3*2*cos(4))"),
            IResult::Done(&b""[..], 0.9306537932852477f64)
        );
        assert_eq!(
            expr(b"(0o12455+0b1101)^(cos(0xAb123)*4)"),
            IResult::Done(&b""[..], 711858057953021.3f64)
        );
        assert_eq!(expr(b"|1|"), IResult::Done(&b""[..], 1f64));
        assert_eq!(expr(b"|-1|"), IResult::Done(&b""[..], 1f64));
        assert_eq!(eval(b"4==4"), IResult::Done(&b""[..], true));
        assert_eq!(eval(b"4!=3"), IResult::Done(&b""[..], true));
        assert_eq!(eval(b"2*2==1+3"), IResult::Done(&b""[..], true));
        assert_eq!(eval(b"2^3==8"), IResult::Done(&b""[..], true));
        assert_eq!(eval(b"(4-2)*4==(16-8)"), IResult::Done(&b""[..], true));
    }
}
