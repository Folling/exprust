pub mod ar {
    extern crate nom;
    use nom::digit;
    // Parser definition

    use std::f64;
    use std::str;
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
        take_while!(
            call!(
                |c| c == '+' as u8 || c == '-' as u8
            )
        )
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

    named!(pub parens<f64>,
        delimited!(tag!("("), expr, tag!(")"))
    );

    named!(pub par_s<f64>, do_parse!(
        init: alt_complete!(parens | hex | binary | float | function_term | constant) >>
        res: fold_many0!(
            alt_complete!(
                 function_term |
                 parens |
                 unsigned_hex |
                 unsigned_binary |
                 unsigned_float |
                 constant
            ),
            init,
            |acc, f : f64| {
                acc*f
            }
        ) >>
        (res)
    ));

    named!(pub abs<f64>, map!(
            delimited!(tag!("|"), expr, tag!("|")),
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

    named!(pub factor<f64>,
        alt_complete!(
            par_s |
            hex |
            binary |
            float |
            constant |
            abs |
            parens
        )
    );

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
                    b"sin"   => val.to_radians().sin(),
                    b"cos"   => val.to_radians().cos(),
                    b"tan"   => val.to_radians().tan(),
                    _        => val
                }
            }
        )
    );

    named!(pub expr<f64>, do_parse!(
        init: alt_complete!(factor_term | function_term) >>
        res:  fold_many0!(
            pair!(alt!(tag!("+") | tag!("-")), alt_complete!(factor_term | function_term) ),
            init,
            |acc, (op, val): (&[u8], f64)| {
                if (op[0] as char) == '+' { acc + val } else { acc - val }
            }
        ) >>
    (res)
    )
    );

    named!(pub comp<&[u8]>,
        alt_complete!(
            tag!("==") |
            tag!("<=") |
            tag!(">=") |
            tag!("!=") |
            tag!("<")  |
            tag!(">")  |
            tag!("=")
        )
    );

    named!(pub eval<bool>, map!(
        tuple!(expr, comp, expr),
        | (left, comp_op, right) : (f64, &[u8], f64)|{
            match comp_op {
                b"==" => left == right,
                b"<=" => left <= right,
                b">=" => left >= right,
                b"!=" => left != right,
                b"<" => left < right,
                b">" => left > right,
                b"=" => left == right,
                _ => false
            }
        }
    )
    );
}
