extern crate nom;

use nom::{digit, oct_digit};

use super::hierarchy::*;
use std::str;
use std::f64;
use std::str::FromStr;

named!(
    unsigned_float<f64>,
    map_res!(
        map_res!(
            recognize!(alt_complete!(
                delimited!(opt!(digit), tag!("."), digit) | digit
            )),
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(pub signs<&[u8]>,
    ws!(take_while!(
        call!(
            |c| c == '+' as u8 || c == '-' as u8
        )
    ))
);

named!(pub eval_signs<u8>,
    map!(
        signs,
        |c : &[u8]| {
            let neg_count = c.iter().filter(|&&x| x == '-' as u8).count();
            match neg_count%2{
                1 => '-' as u8,
                _ => '+' as u8
            }
        }
    )
);

named!(pub float <f64>, map!(
    pair!(
      opt!(eval_signs),
      unsigned_float
    ),
    |(sign, value): (Option<u8>, f64)| {
      match sign{
        Some(b'-') => -1f64*value,
        _ => 1f64* value
      }
    }
));

named!(pub unsigned_hex<f64>, map!(
    preceded!(tag!("0x"), nom::hex_digit),
        | bytes : &[u8]|{
            (i64::from_str_radix(str::from_utf8(bytes).unwrap(), 16).unwrap_or(0) as f64)
        }
));

named!(pub hex<f64>, map!(
    pair!(
      opt!(eval_signs),
      unsigned_hex
    ),
    |(sign, value): (Option<u8>, f64)| {
      match sign{
        Some(b'-') => -1f64*value,
        _ => 1f64 * value
      }
    }
));

named!(pub bits<&[u8]>,
    take_while!(
        call!(
            |c| c == '1' as u8 || c == '0' as u8
        )
    )
);

named!(pub unsigned_binary<f64>, map!(
    preceded!(tag!("0b"), bits),
        | bytes : &[u8]|{
            (i64::from_str_radix(str::from_utf8(bytes).unwrap(), 2).unwrap_or(0) as f64)
}));

named!(pub binary<f64>, map!(
    pair!(
      opt!(eval_signs),
      unsigned_binary
    ),
    |(sign, value): (Option<u8>, f64)| {
      match sign{
        Some(b'-') => -1f64*value,
        _ => 1f64 * value
      }
    }
));

named!(pub unsigned_oct<f64>, map!(
    preceded!(tag!("0o"), oct_digit),
        | bytes : &[u8]|{
            (i64::from_str_radix(str::from_utf8(bytes).unwrap(), 8).unwrap_or(0) as f64)
}));

named!(pub oct<f64>, map!(
    pair!(
      opt!(eval_signs),
      unsigned_oct
    ),
    |(sign, value): (Option<u8>, f64)| {
      match sign{
        Some(b'-') => -1f64*value,
        _ => 1f64 * value
      }
    }
));

named!(pub parens<f64>,
    ws!(delimited!(tag!("("), expr, tag!(")")))
);

named!(pub abs<f64>, map!(
        ws!(delimited!(tag!("|"), expr, tag!("|"))),
        |x : f64| {
            x.abs()
        }
    )
);

named!(pub constant<f64>, map!(
        alt_complete!(
            tag!("pi") |
            tag!("e")  |
            tag!("phi")
        ),
        |id : &[u8]|{
            match id {
                b"pi" => f64::consts::PI,
                b"e"  => f64::consts::E,
                b"phi"=> 1.61803398874989484820458683436563811f64,
                _ => 0f64
            }
        }
    )
);

named!(pub degrees<f64>,
    map!(
        ws!(pair!( alt_complete!(parens | hex | binary | float), tag!("°"))),
        | (x, _) : (f64, &[u8])|{
            x.to_radians()
        }
    )
);

named!(pub factor<f64>,
    alt_complete!(
        degrees |
        function_term |
        hex |
        binary |
        oct |
        float |
        constant |
        abs |
        parens
    )
);
