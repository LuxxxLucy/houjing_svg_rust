#![allow(dead_code)]

use crate::core::geometry::Number;
use std;
pub use slotmap::{Key, new_key_type};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum Error {
    SynthesisUnknown,
    SynthesisUnsatisfiable,
}

pub type Result<T> = std::result::Result<T, Error>;


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
        Id {
            data: id,
            description: None 
        }
    }

    pub fn new_named(id: usize, s: String) -> Self {
        Id {
            data: id,
            description: Some(s) 
        }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(s) = &self.description {
            write!(f, "\"{}\"{}", s, self.data.to_string())
        } else{
            write!(f, "{}", self.data.to_string())
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operator{
    Add,
    Sub,
}

#[derive(Clone, Debug)]
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

    pub fn from_id(id: &Id) -> Var {
        Var {
            var_type: VarType::Terminal,
            terminal_var: Some(TerminalVar {
                id: id.clone()
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

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Constraint::Eq(id1, id2) =>  {
                write!(f, "Id {} == Id {}", id1, id2)
            }
            Constraint::NewVar(id) =>  {
                write!(f, "New Var: {}", id)
            }
            Constraint::Const(id, n) =>  {
                write!(f, "Set {} to const {}", id, n)
            }
            Constraint::EqualVar(v1, v2) =>  {
                write!(f, "Var {:#?} == Var {:#?}", v1, v2)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Spec {
    pub constraints: Vec<Constraint>,
    pub num_constraints: usize,
    // pub vars: Vec<Var>,
    pub num_unique_vars: usize,
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