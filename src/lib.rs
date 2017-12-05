#![feature(test)]

extern crate test;
pub mod exprust;
pub mod flag_funcs;

#[macro_use]
extern crate nom;

#[cfg(test)]
mod tests {
    use exprust::ar;
    use flag_funcs::f64_flag_magic::{Flag, FlagFunc};
    use nom::IResult;
    use test::Bencher;

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| ar::expr(b"1+1"));
    }

    #[test]
    fn now() {
        if let IResult::Done(t, r) = ar::expr(b"cos((1+1)\xC2\xB0)") {
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
        assert_eq!(
            ar::expr(b"(0x14)^(2+0x1)"),
            IResult::Done(&b""[..], 8000f64)
        );
        assert_eq!(ar::expr(b"0b11011"), IResult::Done(&b""[..], 27f64));
        assert_eq!(
            ar::expr(b"0b1011*(0b11-0b1)^0b101"),
            IResult::Done(&b""[..], 352f64)
        );
        assert_eq!(
            ar::expr(b"sin((86+(cos(0\xC2\xB0)+1)\xC2\xB0)^2)"),
            IResult::Done(&b""[..], 0.4012299626605139f64)
        );
        assert_eq!(
            ar::expr(b"4*sin((86+((cos(0)+1)^2))\xC2\xB0)"),
            IResult::Done(&b""[..], 4f64)
        );
        assert_eq!(
            ar::expr(b"sin(15+4^2+3*2*cos(4))"),
            IResult::Done(&b""[..], 0.9306537932852477f64)
        );
        assert_eq!(ar::expr(b"|1|"), IResult::Done(&b""[..], 1f64));
        assert_eq!(ar::expr(b"|-1|"), IResult::Done(&b""[..], 1f64));
        assert_eq!(ar::eval(b"4==4"), IResult::Done(&b""[..], true));
        assert_eq!(ar::eval(b"4!=3"), IResult::Done(&b""[..], true));
        assert_eq!(ar::eval(b"2*2==1+3"), IResult::Done(&b""[..], true));
        assert_eq!(ar::eval(b"2^3==8"), IResult::Done(&b""[..], true));
        assert_eq!(ar::eval(b"(4-2)*4==(16-8)"), IResult::Done(&b""[..], true));
    }
}
