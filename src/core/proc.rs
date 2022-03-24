use crate::core::data::*;
use crate::core::data::Constraint;

pub struct Builder {
    spec: Spec,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            spec: Spec::empty()
        }
    }

    pub fn add_constraint(&mut self, c: Constraint) {
        self.spec.constraints.push(c);
        self.spec.num_constraints += 1;
    }

    pub fn add_var(&mut self, c: Constraint) {
        // self.spec.vars.push(c);
        self.add_constraint(c);
    }

    pub fn new_var_(&mut self) -> Id {
        // let k = Id { data: self.spec.num_unique_vars as usize, description: None };
        let id = self.spec.num_unique_vars as usize;
        let k = Id::new(id);
        self.add_var(Constraint::NewVar(k.clone()));
        self.spec.num_unique_vars += 1;
        k
    }

    pub fn new_var_named_(&mut self, s: &str) -> Id {
        let id = self.spec.num_unique_vars as usize;
        let k = Id::new_named(id, String::from(s));
        self.add_var(Constraint::NewVar(k.clone()));
        self.spec.num_unique_vars += 1;
        k
    }

    pub fn eq_val_(&mut self, a: Var, b: Var) {
        self.add_constraint(Constraint::EqualVar(a, b));
    }

    pub fn all(&self) -> Spec {
        Spec::new(
            self.spec.constraints.clone(),
            self.spec.num_constraints,
            self.spec.num_unique_vars,
        )
    } 
}