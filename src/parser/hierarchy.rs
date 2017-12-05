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
        init: power_term >>
        res: fold_many0!(
            pair!(alt!(tag!("*")| tag!("/")), power_term),
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
        pair!(alt_complete!(
                tag!("floor") |
                tag!("ceil")  |
                tag!("round") |
                tag!("trunc") |
                tag!("fract") |
                tag!("abs")   |
                tag!("sig")   |
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
            delimited!(opt!(tag!("(")), expr, opt!(tag!(")")))
        ),
        |(func, val): (&[u8], f64)|{
            match func {
                b"floor" => val.floor(),
                b"ceil" => val.ceil(),
                b"round" => val.round(),
                b"trunc" => val.trunc(),
                b"fract" => val - val.floor(),
                b"abs"   => val.abs(),
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
    init: alt_complete!(factor_term | function_term) >>
    res:  fold_many0!(
        pair!(alt!(tag!("+") | tag!("-")), factor_term),
        init,
        |acc, (op, val): (&[u8], f64)| {
            if (op[0] as char) == '+' { acc + val } else { acc - val }
        }
    ) >>
(res)
)
);
