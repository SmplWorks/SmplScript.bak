use crate::parser::{Expr, parse_str};
use crate::utils::{SError, SRes};

#[derive(Debug, PartialEq)]
pub enum SValue {
    None,
    Number(i32),
}

fn execute_number(x : i32) -> SRes<SValue> {
    Ok(SValue::Number(x))
}

fn execute_block(exprs : &Vec<Expr>) -> SRes<SValue> {
    exprs.iter().fold(Ok(SValue::None), |_, e| Ok(execute(e)?))
}

pub fn execute(e : &Expr) -> SRes<SValue> {
    match e {
        Expr::Number(x) => execute_number(*x),
        Expr::Block(exprs) => execute_block(exprs),
        _ => todo!(),
    }
}

pub fn execute_str(s : &str) -> SRes<SValue> {
    execute(&parse_str(s)?)
}

#[test]
fn test_number() {
    assert_eq!(execute_str("0"), Ok(SValue::Number(0)));
    assert_eq!(execute_str("1"), Ok(SValue::Number(1)));
}

#[test]
fn test_block() {
    assert_eq!(execute_str("{}"), Ok(SValue::None));
    assert_eq!(execute_str("{0}"), Ok(SValue::Number(0)));
    assert_eq!(execute_str("{0 1}"), Ok(SValue::Number(1)));
}
