#![feature(test)]

extern crate test;
pub mod exprust;

#[macro_use]
extern crate nom;

#[cfg(test)]
mod tests {
    use exprust::ar;
    use nom::IResult;
    use test::Bencher;
    use std::str;

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| ar::expr(b"1+1"));
    }

    #[test]
    fn now() {
        if let IResult::Done(t, r) = ar::factor_term(b"4^cos(3)*4^2") {
            println!("{}", r);
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(ar::expr(b"4"), IResult::Done(&b""[..], 4f64));
        assert_eq!(ar::expr(b"4+4"), IResult::Done(&b""[..], 8f64));
        assert_eq!(ar::expr(b"5*5"), IResult::Done(&b""[..], 25f64));
        assert_eq!(ar::expr(b"4*(4+4)"), IResult::Done(&b""[..], 32f64));
        assert_eq!(ar::expr(b"4*(4-4)"), IResult::Done(&b""[..], 0f64));
        assert_eq!(ar::expr(b"4-4"), IResult::Done(&b""[..], 0f64));
        assert_eq!(ar::expr(b"4+-4"), IResult::Done(&b""[..], 0f64));
        assert_eq!(ar::expr(b"-4+4"), IResult::Done(&b""[..], 0f64));
        assert_eq!(ar::expr(b"4^3"), IResult::Done(&b""[..], 64f64));
        assert_eq!(ar::expr(b"4^(3-1)"), IResult::Done(&b""[..], 16f64));
        assert_eq!(ar::expr(b"-3^(2+3^2)"), IResult::Done(&b""[..], -177147f64));
        assert_eq!(ar::expr(b"(0x14)^(2+0x1)"), IResult::Done(&b""[..], 8000f64));
        assert_eq!(ar::expr(b"0b11011"), IResult::Done(&b""[..], 27f64));
        assert_eq!(ar::expr(b"0b1011*(0b11-0b1)^0b101"), IResult::Done(&b""[..], 352f64));
        assert_eq!(ar::expr(b"sin(86+(cos(0)+1)^2)"),IResult::Done(&b""[..], 1f64));
        assert_eq!(ar::expr(b"4*sin(86+(cos(0)+1)^2)"),IResult::Done(&b""[..], 4f64));
        assert_eq!(ar::expr(b"|1|"), IResult::Done(&b""[..], 1f64));
        assert_eq!(ar::expr(b"|-1|"), IResult::Done(&b""[..], 1f64));
        assert_eq!(ar::eval(b"4==4"), IResult::Done(&b""[..], true));
        assert_eq!(ar::eval(b"4!=3"), IResult::Done(&b""[..], true));
        assert_eq!(ar::eval(b"2*2==1+3"), IResult::Done(&b""[..], true));
        assert_eq!(ar::eval(b"2^3==8"), IResult::Done(&b""[..], true));
        assert_eq!(ar::eval(b"(4-2)*4==(16-8)"), IResult::Done(&b""[..], true));
    }
}
