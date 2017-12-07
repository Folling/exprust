extern crate nom;
use super::numbers::*;

named!(pub power_term<f64>, do_parse!(
        init: factor >>
        res: fold_many0!(
            pair!(tag!("^"), factor),
            init,
            |acc : f64, (_, val) : ( _, f64)| {
                acc.powf(val)
            }
        ) >>
        (res)
    )
);

named!(pub factor_term<f64>, do_parse!(
        init: ws!(power_term) >>
        res: fold_many0!(
            ws!(pair!(alt!(tag!("*")| tag!("/")), power_term)),
            init,
            | acc, (op,val): (&[u8], f64)| {
                if (op[0] as char) == '/' { acc / val } else { acc * val}
            }
        ) >>
        (res)
    )
);

named!(pub function_term<f64>,
    map!(
        ws!(pair!(alt_complete!(
                tag!("floor") |
                tag!("ceil")  |
                tag!("round") |
                tag!("trunc") |
                tag!("fract") |
                tag!("abs")   |
                tag!("sig")   |
                tag!("log2")  |
                tag!("log10") |
                tag!("ln")    |
                tag!("sqrt")  |
                tag!("root")  |
                tag!("cbrt")  |
                tag!("asinh") |
                tag!("acosh") |
                tag!("atanh") |
                tag!("sinh")  |
                tag!("cosh")  |
                tag!("tanh")  |
                tag!("asin")  |
                tag!("acos")  |
                tag!("atan")  |
                tag!("sin")   |
                tag!("cos")   |
                tag!("tan")
            ),
            delimited!(opt!(tag!("(")), expr, opt!(tag!(")"))))
        ),
        |(func, val): (&[u8], f64)|{
            match func {
                b"floor" => val.floor(),
                b"ceil" => val.ceil(),
                b"round" => val.round(),
                b"trunc" => val.trunc(),
                b"fract" => val - val.floor(),
                b"abs"   => val.abs(),
                b"log2"  => val.log2(),
                b"log10" => val.log10(),
                b"ln"    => val.ln(),
                b"sig"   => val.signum(),
                b"sqrt"  => val.sqrt(),
                b"root"  => val.sqrt(),
                b"cbrt"  => val.cbrt(),
                b"asinh" => val.asinh(),
                b"acosh" => val.acosh(),
                b"atanh" => val.atanh(),
                b"sinh"  => val.sinh(),
                b"cosh"  => val.cosh(),
                b"tanh"  => val.tanh(),
                b"asin"  => val.asin().to_degrees(),
                b"acos"  => val.acos().to_degrees(),
                b"atan"  => val.atan().to_degrees(),
                b"sin"   => val.sin(),
                b"cos"   => val.cos(),
                b"tan"   => val.tan(),
                _        => val
            }
        }
    )
);

named!(pub expr<f64>, do_parse!(
    init: ws!(alt_complete!(factor_term | function_term)) >>
    res:  fold_many0!(
        ws!(pair!(alt!(tag!("+") | tag!("-")), factor_term)),
        init,
        |acc, (op, val): (&[u8], f64)| {
            if (op[0] as char) == '+' { acc + val } else { acc - val }
        }
    ) >>
(res)
)
);

pub const NONE : u64 = 0x0;
pub const BIN : u64 = 0x01;
pub const OCT : u64 = 0x02;
pub const HEX : u64 = 0x03;
pub const AS_DEGREES : u64 = 0x04;

pub fn parse( expression : &str, flags : u64) -> String {
    if let nom::IResult::Done(_ , mut underlying_float ) = expr(expression.as_bytes()) {
        let mut res = underlying_float.to_string();
        if flags & AS_DEGREES == AS_DEGREES {
            underlying_float = underlying_float.to_degrees();
        }
        if flags & HEX == HEX {
            res = format!("0x{:x}", underlying_float as u64);
        } else if flags & OCT == OCT {
            res = format!("0o{:o}", underlying_float as u64);
        } else if flags & BIN == BIN {
            res = format!("0b{:b}", underlying_float as u64);
        }
        res
    } else {
        "".to_string()
    }
}
