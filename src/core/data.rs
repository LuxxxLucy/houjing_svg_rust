#![allow(dead_code)]

use crate::core::SVG::Shape;
// use crate::core::geometry::Shape;

use std;
use num::Integer;
pub use slotmap::{Key, new_key_type};
use std::hash::{Hash, Hasher};
// use num_rational::Ratio;
// use num_rational::*;

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
        // DISCLAIMER:
        // this code adopted from Rosetta Code for transforming a float into a rational
        // https://rosettacode.org/mw/index.php?title=Convert_decimal_number_to_rational&action=edit&section=52

        let mut n = n;
        assert!(n.is_finite());
        let flag_neg  = n < 0.0;
        if flag_neg { n = n*(-1.0) }
        if n < std::f64::MIN_POSITIVE { 
            // return [0,1] 
            return Number::new(0,1)
        }
        if (n - n.round()).abs() < std::f64::EPSILON { 
            return Number::new(n.round() as i32 ,1)
        }
        let mut a : i64 = 0;
        let mut b : i64 = 1;
        let mut c : i64 = n.ceil() as i64;
        let mut d : i64 = 1;
        let aux1 = i64::max_value()/2;
        while c < aux1  && d < aux1 {
            let aux2 : f64 = (a as f64 + c as f64)/(b as f64 + d as f64);
            if (n - aux2).abs() < std::f64::EPSILON { break } 
            if n > aux2 { 
                a = a + c;
                b = b + d;
            } else {
                c = a + c;
                d = b + d;
            }
        }
        let gcd = (a+c).gcd(&(b+d));
        if flag_neg { 
            Number::new( (-(a + c)/gcd) as i32, ((b + d)/gcd) as i32 )
        } else {
            Number::new(((a + c)/gcd)  as i32, ((b + d)/gcd) as i32)
        }
      
    }
}

// impl Into<f64> for Number {
//     fn into(self) -> f64 {
//         (self.num as f64) / (self.den as f64)
//     }
// }

impl ToVar for f64 {
    fn to_var(&self) -> Var {
        let n = Number::from(*self);
        n.to_var()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Number {
    // representing a real number = num / den
    pub num: i32,
    pub den: i32,
    // pub data: f,
}

impl From<i32> for Number {
    fn from(i: i32) -> Self {
        Number::new(i,1)
    }
}

// impl From<f32> for Number {
//     fn from(i: f32) -> Self {
//         Number::new(f64::from(i))
//     }
// }

impl Number {
    pub fn new(n: i32, d: i32) -> Self {
        Number {
            num: n,
            den: d
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
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.2} = {}/{}", (self.num as f64)/(self.den as f64), self.num, self.den)
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
    EqualVar(Var, Var)
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
        }
    }
}

pub struct Spec {
    pub constraints: Vec<Constraint>,
    pub num_constraints: usize,
    pub vars: Vec<Var>,
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
    pub fn new(c: Vec<Constraint>, n_c: usize, v: Vec<Var>, n_u_v: usize) -> Self {
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

}