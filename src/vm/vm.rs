use std::collections::HashMap;
use std::rc::Rc;

use crate::parser::{Expr, parse_str};
use crate::utils::{SError, SRes};

#[derive(Debug, PartialEq)]
pub enum SValue {
    None,
    Number(i32),
    Bool(bool),
}

impl SValue {
    fn to_number(&self) -> SRes<SValue> { // TODO: Base
        match self {
            SValue::None => Ok(SValue::Number(0)),
            SValue::Number(x) => Ok(SValue::Number(*x)),
            SValue::Bool(value) => Ok(SValue::Number(*value as i32)),
            _ => todo!(),
        }
    }
}

pub struct SContext {
}

impl SContext {
    fn new() -> SContext {
        SContext{
        }
    }
}

fn execute_none(ctx : &mut SContext) -> SRes<SValue> {
    Ok(SValue::None)
}

fn execute_number(x : i32, ctx : &mut SContext) -> SRes<SValue> {
    Ok(SValue::Number(x))
}

fn execute_bool(value : bool, ctx : &mut SContext) -> SRes<SValue> {
    Ok(SValue::Bool(value))
}

fn execute_block(exprs : &Vec<Expr>, ctx : &mut SContext) -> SRes<SValue> {
    exprs.iter().fold(Ok(SValue::None), |_, e| Ok(execute(e, ctx)?))
}

fn execute_add(lhs : &Box<Expr>, rhs : &Box<Expr>, ctx : &mut SContext) -> SRes<SValue> {
    let SValue::Number(l) = execute(lhs, ctx)?.to_number()? else { panic!() };
    let SValue::Number(r) = execute(rhs, ctx)?.to_number()? else { panic!() };
    Ok(SValue::Number(l + r))
}

fn execute_binary_op(op : &String, lhs : &Box<Expr>, rhs : &Box<Expr>, ctx : &mut SContext) -> SRes<SValue> {
    match &**op { // TODO: Call op on lhs with rhs
        "+" => execute_add(lhs, rhs, ctx),
        _ => todo!(), // TODO: Custom binary ops
    }
}

pub fn execute(e : &Expr, ctx : &mut SContext) -> SRes<SValue> {
    match e {
        Expr::None => execute_none(ctx),
        Expr::Number(x) => execute_number(*x, ctx),
        Expr::Bool(value) => execute_bool(*value, ctx),
        Expr::Block(exprs) => execute_block(exprs, ctx),
        Expr::BinaryOp { op, lhs, rhs } => execute_binary_op(op, lhs, rhs, ctx),
        _ => todo!(),
    }
}

pub fn execute_str(s : &str, ctx : &mut SContext) -> SRes<SValue> {
    execute(&parse_str(s)?, ctx)
}

#[test]
fn test_none() {
    assert_eq!(execute_str("none", &mut SContext::new()), Ok(SValue::None));
}

#[test]
fn test_number() {
    assert_eq!(execute_str("0", &mut SContext::new()), Ok(SValue::Number(0)));
    assert_eq!(execute_str("1", &mut SContext::new()), Ok(SValue::Number(1)));
}

#[test]
fn test_bool() {
    assert_eq!(execute_str("true", &mut SContext::new()), Ok(SValue::Bool(true)));
    assert_eq!(execute_str("false", &mut SContext::new()), Ok(SValue::Bool(false)));
}

#[test]
fn test_block() {
    assert_eq!(execute_str("{}", &mut SContext::new()), Ok(SValue::None));
    assert_eq!(execute_str("{0}", &mut SContext::new()), Ok(SValue::Number(0)));
    assert_eq!(execute_str("{0 1}", &mut SContext::new()), Ok(SValue::Number(1)));
}

#[test]
fn test_binary_op() {
    assert_eq!(execute_str("1 + 2", &mut SContext::new()), Ok(SValue::Number(3)));
    //assert_eq!(execute_str("1 - 2"), Ok(SValue::Number(-1)));
    //assert_eq!(execute_str("1 * 2"), Ok(SValue::Number(2)));
    //assert_eq!(execute_str("1 / 2"), Ok(SValue::Number(1 / 2))); // TODO: Floating point
    //assert_eq!(execute_str("1 == 1"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("1 == 2"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("1 != 1"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("1 != 2"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("1 < 0"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("1 <= 0"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("1 < 1"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("1 <= 1"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("1 < 2"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("1 <= 2"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("1 > 0"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("1 >= 0"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("1 > 1"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("1 >= 1"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("1 > 2"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("1 >= 2"), Ok(SValue::Bool(false)));
//
    //assert_eq!(execute_str("true and true"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("true and false"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("true and none"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("false and true"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("false and false"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("false and none"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("none and true"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("none and false"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("none and none"), Ok(SValue::Bool(false)));
//
    //assert_eq!(execute_str("true or true"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("true or false"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("true or none"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("false or true"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("false or false"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("false or none"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("none or true"), Ok(SValue::Bool(true)));
    //assert_eq!(execute_str("none or false"), Ok(SValue::Bool(false)));
    //assert_eq!(execute_str("none or none"), Ok(SValue::Bool(false)));
}
