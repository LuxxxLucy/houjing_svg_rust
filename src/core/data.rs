use crate::core::geometry::Number;
use std;
pub use slotmap::{Key, new_key_type};

#[derive(Debug)]
pub enum Error {
    SynthesisUnknown,
    SynthesisUnsatisfiable,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id {
    pub data: usize,
    pub description: Option<String>,
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(s) = self.description {
            write!(f, "\"{}\"{}", s, self.data.to_string())

        } else{
            write!(f, "{}", self.data.to_string())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator{
    Add,
    Sub,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VarType{
    Undefined,
    Terminal,
    Intermediate
}

#[derive(Clone, Debug)]
pub struct IntermediateVar {
    pub op: Operator,
    pub l: Box<Var>,
    pub r: Box<Var>,
}

#[derive(Clone, Debug)]
pub struct TerminalVar {
    pub id: Id
}

#[derive(Clone, Debug)]
pub struct Var {
    pub var_type: VarType,
    pub terminal_var: Option<TerminalVar>,
    pub intermediate_var: Option<IntermediateVar>,
}

impl Var {
    pub fn new(op: Operator, l: Var, r: Var) -> Var {
        Var {
            var_type: VarType::Intermediate,
            terminal_var: None,
            intermediate_var: Some(IntermediateVar{
                op: op,
                l: Box::new(l),
                r: Box::new(r) 
            }),
        }
    }

    pub fn empty() -> Var {
        Var {
            var_type: VarType::Undefined,
            terminal_var: None,
            intermediate_var: None,
        }
    }

    pub fn from_id(id: Id) -> Var {
        Var {
            var_type: VarType::Terminal,
            terminal_var: Some(TerminalVar {
                id: id
            }),
            intermediate_var: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Constraint {
    Eq(Id,Id),
    NewVar(Id),
    Const(Id, Number),
    EqualVar(Var, Var)
    // Add(Id,Id),
    // Add(Id,Id),
}

#[derive(Clone, Debug)]
pub struct Spec {
    pub constraints: Vec<Constraint>,
    pub num_constraints: usize,
    // pub vars: Vec<Var>,
    pub num_unique_vars: usize,
}

impl Spec {
    pub fn new(c: Vec<Constraint>, n_c: usize, n_u_v: usize) -> Self {
        Spec {
            constraints: c,
            num_constraints: n_c,
            // vars: vec![],
            num_unique_vars: n_u_v,
        }
    }

    pub fn empty() -> Self {
        Spec {
            constraints: vec![],
            num_constraints: 0,
            // vars: vec![],
            num_unique_vars: 0,
        }
    }
}

// impl Shape for Circle {
//     fn bound(&self) -> Bound {
//         Bound {
//             left: self.x - self.radius,
//             right: self.x + self.radius,
//             top: self.y - self.radius,
//             bottom: self.y + self.radius,
//         }
//     }

//     fn solve(&self) {}
// }