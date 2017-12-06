extern crate nom;

use super::hierarchy::*;

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

pub fn evaluate(expression : &str) -> String{
    eval(expression.as_bytes()).to_result().unwrap_or(false).to_string()
}
