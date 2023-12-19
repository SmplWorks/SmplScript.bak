use std::{collections::HashMap, cell::RefCell, rc::Rc};
use crate::parser::{Expr, parse_str};
use crate::utils::{SError, SRes};

#[derive(Debug, Clone, PartialEq)]
pub enum SValue {
    None,
    Number(i32),
    Bool(bool),
    Function{ params: Vec<String>, body: Expr },
}

impl SValue {
    fn to_number(&self) -> SRes<SValue> { // TODO: Base
        match self {
            SValue::None => Ok(SValue::Number(0)),
            SValue::Number(x) => Ok(SValue::Number(*x)),
            SValue::Bool(value) => Ok(SValue::Number(*value as i32)),
            _ => Err(SError::VMCannotConvertToNumber),
        }
    }
}

#[derive(Debug)]
pub struct SContext {
    vars : HashMap<String, Rc<RefCell<SValue>>>,
}

impl SContext {
    fn new() -> SContext {
        SContext{
            vars: HashMap::new(),
        }
    }
}

fn execute_none(_ctx : &mut SContext) -> SRes<SValue> {
    Ok(SValue::None)
}

fn execute_number(x : i32, _ctx : &mut SContext) -> SRes<SValue> {
    Ok(SValue::Number(x))
}

fn execute_bool(value : bool, _ctx : &mut SContext) -> SRes<SValue> {
    Ok(SValue::Bool(value))
}

fn execute_block(exprs : &Vec<Expr>, ctx : &mut SContext) -> SRes<SValue> {
    exprs.iter().fold(Ok(SValue::None), |_, e| Ok(execute_expr(e, ctx)?))
}

fn execute_function(params : &Vec<String>, body : &Expr, _ctx : &mut SContext) -> SRes<SValue> {
    Ok(SValue::Function { params: params.clone(), body: body.clone() })
}

fn execute_call(callee : &String, args : &Vec<Expr>, ctx : &mut SContext) -> SRes<SValue> {
    let SValue::Function { params, body } = ctx.vars.get(callee).ok_or(SError::VMVariableDoesntExist)?.borrow().clone() else { return Err(SError::VMCannotCallNonFunction) };

    if params.len() != args.len() {
        return Err(SError::VMMismatchArgumentListLength);
    }

    // TODO: Local scope 
    for (p, arg) in params.iter().zip(args.iter()) {
        execute_assign(&Box::new(Expr::VarRef(p.clone())), &Box::new(arg.clone()), ctx)?;
    }

    execute_expr(&body, ctx)
}

fn execute_add(lhs : &Box<Expr>, rhs : &Box<Expr>, ctx : &mut SContext) -> SRes<SValue> {
    let SValue::Number(l) = execute_expr(lhs, ctx)?.to_number()? else { panic!() };
    let SValue::Number(r) = execute_expr(rhs, ctx)?.to_number()? else { panic!() };
    Ok(SValue::Number(l + r))
}

fn execute_assign(lhs : &Box<Expr>, rhs : &Box<Expr>, ctx : &mut SContext) -> SRes<SValue> {
    let Expr::VarRef(var) = &**lhs else { return Err(SError::VMCannotAssignNonVariable) };
    let rhs = execute_expr(rhs, ctx)?;
    match ctx.vars.get_mut(var) {
        None => { ctx.vars.insert(var.clone(), Rc::new(RefCell::new(rhs))); },
        Some(value) => { *value.borrow_mut() = rhs; },
    }

    Ok(SValue::None)
}

fn execute_binary_op(op : &String, lhs : &Box<Expr>, rhs : &Box<Expr>, ctx : &mut SContext) -> SRes<SValue> {
    match &**op { // TODO: Call op on lhs with rhs
        "=" => execute_assign(lhs, rhs, ctx),
        "+" => execute_add(lhs, rhs, ctx),
        _ => todo!(), // TODO: Custom binary ops
    }
}

fn execute_expr(e : &Expr, ctx : &mut SContext) -> SRes<SValue> {
    match e {
        Expr::None => execute_none(ctx),
        Expr::Number(x) => execute_number(*x, ctx),
        Expr::Bool(value) => execute_bool(*value, ctx),
        Expr::Block(exprs) => execute_block(exprs, ctx),
        Expr::Function { params, body } => execute_function(params, body, ctx),
        Expr::Call { callee, args } => execute_call(callee, args, ctx),
        Expr::BinaryOp { op, lhs, rhs } => execute_binary_op(op, lhs, rhs, ctx),
        _ => todo!("{:?}", e),
    }
}

pub fn execute_str(s : &str, ctx : &mut SContext) -> SRes<SValue> {
    execute_expr(&parse_str(s)?, ctx)
}

pub fn execute_string(s : &String, ctx : &mut SContext) -> SRes<SValue> {
    execute_str(&*s, ctx)
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

#[test]
fn test_assign() {
    let mut ctx = SContext::new();
    execute_str("x = 1", &mut ctx).unwrap();
    assert_eq!(ctx.vars.get("x"), Some(&Rc::new(RefCell::new(SValue::Number(1)))));

    let mut ctx = SContext::new();
    execute_str("x = 1", &mut ctx).unwrap();
    execute_str("x = 2", &mut ctx).unwrap();
    assert_eq!(ctx.vars.get("x"), Some(&Rc::new(RefCell::new(SValue::Number(2)))));

    let mut ctx = SContext::new();
    execute_str("x = 1", &mut ctx).unwrap();
    execute_str("y = 2", &mut ctx).unwrap();
    assert_eq!(ctx.vars.get("x"), Some(&Rc::new(RefCell::new(SValue::Number(1)))));
    assert_eq!(ctx.vars.get("y"), Some(&Rc::new(RefCell::new(SValue::Number(2)))));
}

#[test]
fn test_function() {
    let mut ctx = SContext::new();
    execute_str("fn main(x, y) 0", &mut ctx).unwrap();
    assert_eq!(ctx.vars.get("main"), Some(&Rc::new(RefCell::new(SValue::Function{
        params: vec!["x".to_string(), "y".to_string()],
        body: Expr::Number(0),
    }))));
}

#[test]
fn test_call() {
    let mut ctx = SContext::new();
    execute_str("fn zero() 0", &mut ctx).unwrap();
    execute_str("fn one() 1", &mut ctx).unwrap();
    execute_str("foo = 0", &mut ctx).unwrap();
    assert_eq!(execute_str("zero()", &mut ctx), Ok(SValue::Number(0)));
    assert_eq!(execute_str("one()", &mut ctx), Ok(SValue::Number(1)));
    assert_eq!(execute_str("bar()", &mut ctx), Err(SError::VMVariableDoesntExist));
    assert_eq!(execute_str("foo()", &mut ctx), Err(SError::VMCannotCallNonFunction));
    assert_eq!(execute_str("one(1)", &mut ctx), Err(SError::VMMismatchArgumentListLength));

    // TODO: Test correct params values
}

#[test]
fn test_full() {
    let mut ctx = SContext::new();
    assert_eq!(execute_str("{fn zero() 0 zero()}", &mut ctx), Ok(SValue::Number(0)));
}
