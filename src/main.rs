use std::collections::HashMap;
use std::ops::Deref;
#[derive(Debug, Clone)]
enum Expr<'a> {
    EInt {
        value: f64,
    },
    EVar {
        name: &'a str,
    },
    EFunc {
        param: &'a str,
        body: Box<Expr<'a>>,
    },
    ECall {
        func: Box<Expr<'a>>,
        arg: Box<Expr<'a>>,
    },
    ECond {
        cond: Box<Expr<'a>>,
        true_branch: Box<Expr<'a>>,
        false_branch: Box<Expr<'a>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type<'a> {
    TNamed(String),
    TVar(&'a str),
    TFun {
        from: Box<Type<'a>>,
        to: Box<Type<'a>>,
    },
}
pub type Env<'a> = HashMap<String, Type<'a>>;
pub type Substitution<'a> = HashMap<String, Type<'a>>;

pub struct Context<'a> {
    pub env: Env<'a>,
    current: usize,
}
impl<'a> Context<'a> {
    pub fn new_from_current(&mut self, name: String, new_type: Type<'a>) {
        self.env.insert(name, new_type);
        self.current += 1;
    }
}
pub fn apply_to_subst<'a>(subst: &Substitution<'a>, t: Type<'a>) -> Type<'a> {
    match t {
        Type::TNamed(_) => t.clone(),
        Type::TVar(name) => match subst.get(name.clone()) {
            Some(x) => x.clone(),
            None => t.clone(),
        },
        Type::TFun { from, to } => {
            let (from, to) = (
                Box::new(apply_to_subst(subst, from.deref().clone()).clone()),
                Box::new(apply_to_subst(subst, from.deref().clone()).clone()),
            );
            Type::TFun { from, to }
        }
    }
}
pub fn new_t_var<'a>(ctx: &mut Context<'a>) -> Type<'a> {
    Type::TNamed((ctx.current + 1).to_string())
}
fn infer<'a>(
    ctx: &mut Context<'a>,
    expr: Expr<'a>,
) -> Result<(Type<'a>, Substitution<'a>), String> {
    match expr {
        Expr::EInt { .. } => Ok((Type::TNamed("Int".to_string()), HashMap::new())),
        Expr::EVar { name } => match ctx.env.get(name) {
            Some(x) => Ok((x.clone(), HashMap::new())),
            None => Err(format!("Use of undeclared variable {}", name)),
        },
        Expr::EFunc { param, body } => {
            let new_type = new_t_var(ctx);
            ctx.new_from_current(param.to_string(), new_type.clone());
            let (body_type, subst) = infer(ctx, body.deref().clone())?;
            let body_type = Box::new(body_type);
            let infered_type = Type::TFun {
                from: Box::new(apply_to_subst(&subst, new_type).clone()),
                to: body_type,
            };
            Ok((infered_type, subst))
        }
        _ => unimplemented!(),
    }
}
fn main() {
    println!("Hello, world!");
}
