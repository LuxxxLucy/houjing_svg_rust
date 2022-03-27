#![allow(dead_code)]

use crate::core::SVG::Shape;
// use crate::core::geometry::Shape;

use std;
// use num::Integer;
pub use slotmap::{Key, new_key_type};
use std::hash::{Hash, Hasher};
use rug::Rational;

#[derive(Debug)]
pub enum Error {
    SynthesisUnknown,
    SynthesisUnsatisfiable,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait ToVar {
    fn to_var(&self) -> Var;
}

pub trait Eval {
    fn eval(&self) -> Var;
}

impl ToVar for i32 {
    fn to_var(&self) -> Var {
        Number::from(*self).to_var()
    }
}

impl From<f64> for Number {
    fn from(n: f64) -> Number {
        // we are just going to use f32.
        let n_f32 = Rational::from_f64(n).unwrap().to_f32();
        Number {
            data: Rational::from_f32(n_f32).unwrap()
        }
    }
}

impl ToVar for f64 {
    fn to_var(&self) -> Var {
        let n = Number::from(*self);
        n.to_var()
    }
}

#[derive(Clone, Debug)]
pub struct Number {
    // representing a arbitray precision number
    pub data: Rational,
}

impl From<i32> for Number {
    fn from(i: i32) -> Self {
        Number {
            data: Rational::from((i as i32, 1))
        }
    }
}

// impl From<f32> for Number {
//     fn from(i: f32) -> Self {
//         Number::new(f64::from(i))
//     }
// }

impl Number {

    pub fn get_num_den(&self) -> (i32, i32) {
        // let (num, den) = Rational::from(self.data.recip_ref()).into_numer_denom();
        let (num, den) = self.data.clone().into_numer_denom();
        let num = num.to_i32().unwrap();
        let den = den.to_i32().unwrap();
        (num, den)
    }
    pub fn new(n: i32, d: i32) -> Self {
        Number {
            data: Rational::from((n, d))
        }
    }

    pub fn to_var(&self) -> Var {
        Var {
            var_type: VarType::Constant,
            terminal_var: None,
            intermediate_var: None,
            constant_var: Some(ConstantVar {
                val: self.clone()
            }),
        }
    }

    pub fn to_f64(&self) -> f64 {
        self.data.to_f64()
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (num, den) = self.get_num_den();
        write!(f, "{:.2} = {}/{}", self.data.to_f64(), num, den)
    }
}

#[derive(Debug, Clone)]
pub struct Id {
    // we will implement the PartialEq, Eq and hash to only consider the `data` field for hash.
    pub data: usize,
    pub description: Option<String>,
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
        // match (self, other) {
        //     (&Left(ref a), &Left(ref b)) => a == b,
        //     (&Right(ref a), &Right(ref b)) => a == b,
        //     _ => false,
        // }
    }
}
impl Eq for Id { }

impl Hash for Id {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl Id {
    pub fn new(id: usize) -> Self {
        Id { data: id, description: None }
    }
    pub fn new_named(id: usize, s: String) -> Self {
        Id { data: id, description: Some(s) 
        }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(s) = &self.description {
            write!(f, "{} with name \"{}\"", self.data.to_string(), s)
        } else{
            write!(f, "{}", self.data.to_string())
        }
    }
}

impl Id {
    pub fn to_var(&self) -> Var {
        Var {
            var_type: VarType::Terminal,
            terminal_var: Some(TerminalVar {
                id: self.clone(),
                val: None
            }),
            intermediate_var: None,
            constant_var: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operator{
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VarType{
    Undefined,
    Terminal,
    Intermediate,
    Constant,
    Inferred,
}

#[derive(Clone, Debug)]
pub struct IntermediateVar {
    pub op: Operator,
    pub l: Box<Var>,
    pub r: Box<Var>,
}

#[derive(Clone, Debug)]
pub struct TerminalVar {
    pub id: Id,
    pub val: Option<Number>,
}

#[derive(Clone, Debug)]
pub struct ConstantVar {
    pub val: Number, 
}

#[derive(Clone, Debug)]
pub struct Var {
    pub var_type: VarType,
    pub terminal_var: Option<TerminalVar>,
    pub intermediate_var: Option<IntermediateVar>,
    pub constant_var: Option<ConstantVar>,
}


impl Var {
    pub fn empty() -> Var {
        Var {
            var_type: VarType::Undefined,
            terminal_var: None,
            intermediate_var: None,
            constant_var: None,
        }
    }

    pub fn new(op: Operator, l: Var, r: Var) -> Var {
        Var {
            var_type: VarType::Intermediate,
            terminal_var: None,
            intermediate_var: Some(IntermediateVar{
                op: op,
                l: Box::new(l),
                r: Box::new(r) 
            }),
            constant_var: None,
        }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.var_type {
            VarType::Undefined => {
                write!(f, "Undefine Var!")
            }
            VarType::Terminal => {
                write!(f, "An terminal Var!")
            }
            VarType::Intermediate => {
                write!(f, "An intermediate Var!")
            }
            VarType::Constant => {
                write!(f, "An Constant Var!")
            }
            VarType::Inferred => {
                let tmp = self.terminal_var.as_ref().unwrap();
                let id = &tmp.id;
                if let Some(val) = &tmp.val {
                    write!(f, "An Inferred Var! Id: {} Value: {:.2} ", id, val)
                } else {
                    write!(f, "An Inferred Var! Id: {} but with no inferred value (None)", id)
                }
            }
        }
    }
}

// impl From<Id> for Var {
//     fn from(id: Id) -> Self {
//         Var {
//             var_type: VarType::Terminal,
//             terminal_var: Some(TerminalVar{ id: id, val: None }),
//             intermediate_var: None,
//             constant_var: None,
//         }
//     }
// }

impl std::ops::Add<Var> for Var {
    type Output = Var;
    fn add(self, _rhs: Var) -> Var {
        Var::new(Operator::Add, self, _rhs)
    }
}

impl std::ops::Div<Var> for Var {
    type Output = Var;
    fn div(self, _rhs: Var) -> Var {
        Var::new(Operator::Div, self, _rhs)
    }
}

impl std::ops::Mul<Var> for Var {
    type Output = Var;
    fn mul(self, _rhs: Var) -> Var {
        Var::new(Operator::Mul, self, _rhs)
    }
}

#[derive(Clone, Debug)]
pub enum Constraint {
    Const(Id, Number),
    EqualVar(Var, Var),
    GT(Var, Var),
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            // Constraint::NewVar(id) =>  {
            //     write!(f, "New Var: {}", id)
            // }
            Constraint::Const(id, n) =>  {
                write!(f, "Set {} to const {}", id, n)
            }
            Constraint::EqualVar(v1, v2) =>  {
                write!(f, "Var {:#?} == Var {:#?}", v1, v2)
            }
            Constraint::GT(v1, v2) => {
                write!(f, "Var {:#?} >= Var {:#?}", v1, v2)
            }
        }
    }
}

pub type VarContainer = Vec<Var>;

pub struct Spec {
    pub constraints: Vec<Constraint>,
    pub num_constraints: usize,
    pub vars: VarContainer,
    pub num_unique_vars: usize,

    pub nodes: Vec<Box<dyn Shape>>,
}

impl std::fmt::Display for Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "# of unique vars: {}\n", self.num_constraints).unwrap();
        write!(f, "# of constraints: {}\n", self.num_constraints).unwrap();
        for c in self.constraints.iter() {
            write!(f, "{}\n", c).unwrap();
        }
        Ok(())
    }
}

impl Spec {
    pub fn new(c: Vec<Constraint>, n_c: usize, v: VarContainer, n_u_v: usize) -> Self {
        Spec {
            constraints: c,
            num_constraints: n_c,
            vars: v,
            num_unique_vars: n_u_v,
            nodes: vec![]
        }
    }

    pub fn empty() -> Self {
        Spec {
            constraints: vec![],
            num_constraints: 0,
            vars: vec![],
            num_unique_vars: 0,
            nodes: vec![]
        }
    }

    pub fn set_values_to(&mut self) {
        
        let vars = &self.vars;
        for n in self.nodes.iter_mut() {
            n.set_inferred_data(vars);
        }
    }

}