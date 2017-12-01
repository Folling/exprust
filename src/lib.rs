#![feature(test)]

extern crate test;
pub mod ar;

#[macro_use]

extern crate nom;

#[cfg(test)]
mod tests {
    use ar::ar;
    use nom::IResult;
    use test::Bencher;

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| ar::expr(b"(4+5)^(2^2*2)-(4^4)"));
    }

    // #[test]
    fn now() {
        if let IResult::Done(_, r) = ar::expr(b"4sin(86+(cos(0)+1)^2)"){
            println!("{}", r);
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(ar::expr(b"4"), IResult::Done(&b""[..],4f64));
        assert_eq!(ar::expr(b"4+4"), IResult::Done(&b""[..],8f64));
        assert_eq!(ar::expr(b"5*5"), IResult::Done(&b""[..],25f64));
        assert_eq!(ar::expr(b"4*(4+4)"), IResult::Done(&b""[..],32f64));
        assert_eq!(ar::expr(b"4*(4-4)"), IResult::Done(&b""[..],0f64));
        assert_eq!(ar::expr(b"4-4"), IResult::Done(&b""[..],0f64));
        assert_eq!(ar::expr(b"4+-4"), IResult::Done(&b""[..],0f64));
        assert_eq!(ar::expr(b"4(4-4)-4"), IResult::Done(&b""[..],-4f64));
        assert_eq!(ar::expr(b"-4+4"), IResult::Done(&b""[..],0f64));
        assert_eq!(ar::expr(b"-4(4-2)+3"), IResult::Done(&b""[..],-5f64));
        assert_eq!(ar::expr(b"4^3"), IResult::Done(&b""[..],64f64));
        assert_eq!(ar::expr(b"4^(3-1)"), IResult::Done(&b""[..],16f64));
        assert_eq!(ar::expr(b"-3^(2+3^2)"), IResult::Done(&b""[..],-177147f64));
        assert_eq!(ar::expr(b"sin(86+(cos(0)+1)^2)"), IResult::Done(&b""[..],1f64));
        assert_eq!(ar::expr(b"4*sin(86+(cos(0)+1)^2)"), IResult::Done(&b""[..],4f64));
        assert_eq!(ar::expr(b"4sin(86+(cos(0)+1)^2)"), IResult::Done(&b""[..],4f64));
    }
}