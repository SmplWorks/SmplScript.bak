use crate::parser::{Expr, parse_str};
use crate::utils::{SError, SRes};

#[derive(Debug, PartialEq)]
pub enum SValue {
    None,
    Number(i32),
    Bool(bool),
}

fn execute_none() -> SRes<SValue> {
    Ok(SValue::None)
}

fn execute_number(x : i32) -> SRes<SValue> {
    Ok(SValue::Number(x))
}

fn execute_bool(value : bool) -> SRes<SValue> {
    Ok(SValue::Bool(value))
}

fn execute_block(exprs : &Vec<Expr>) -> SRes<SValue> {
    exprs.iter().fold(Ok(SValue::None), |_, e| Ok(execute(e)?))
}

fn execute_binary_op(op : &String, lhs : &Box<Expr>, rhs : &Box<Expr>) -> SRes<SValue> {
    todo!()
}

pub fn execute(e : &Expr) -> SRes<SValue> {
    match e {
        Expr::None => execute_none(),
        Expr::Number(x) => execute_number(*x),
        Expr::Bool(value) => execute_bool(*value),
        Expr::Block(exprs) => execute_block(exprs),
        Expr::BinaryOp { op, lhs, rhs } => execute_binary_op(op, lhs, rhs),
        _ => todo!(),
    }
}

pub fn execute_str(s : &str) -> SRes<SValue> {
    execute(&parse_str(s)?)
}

#[test]
fn test_none() {
    assert_eq!(execute_str("none"), Ok(SValue::None));
}

#[test]
fn test_number() {
    assert_eq!(execute_str("0"), Ok(SValue::Number(0)));
    assert_eq!(execute_str("1"), Ok(SValue::Number(1)));
}

#[test]
fn test_bool() {
    assert_eq!(execute_str("true"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("false"), Ok(SValue::Bool(false)));
}

#[test]
fn test_block() {
    assert_eq!(execute_str("{}"), Ok(SValue::None));
    assert_eq!(execute_str("{0}"), Ok(SValue::Number(0)));
    assert_eq!(execute_str("{0 1}"), Ok(SValue::Number(1)));
}

#[test]
fn test_binary_op() {
    assert_eq!(execute_str("1 + 2"), Ok(SValue::Number(3)));
    assert_eq!(execute_str("1 - 2"), Ok(SValue::Number(-1)));
    assert_eq!(execute_str("1 * 2"), Ok(SValue::Number(2)));
    assert_eq!(execute_str("1 / 2"), Ok(SValue::Number(1 / 2))); // TODO: Floating point
    assert_eq!(execute_str("1 == 1"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("1 == 2"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("1 != 1"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("1 != 2"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("1 < 0"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("1 <= 0"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("1 < 1"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("1 <= 1"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("1 < 2"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("1 <= 2"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("1 > 0"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("1 >= 0"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("1 > 1"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("1 >= 1"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("1 > 2"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("1 >= 2"), Ok(SValue::Bool(false)));

    assert_eq!(execute_str("true and true"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("true and false"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("true and none"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("false and true"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("false and false"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("false and none"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("none and true"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("none and false"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("none and none"), Ok(SValue::Bool(false)));

    assert_eq!(execute_str("true or true"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("true or false"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("true or none"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("false or true"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("false or false"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("false or none"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("none or true"), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("none or false"), Ok(SValue::Bool(false)));
    assert_eq!(execute_str("none or none"), Ok(SValue::Bool(false)));
}
