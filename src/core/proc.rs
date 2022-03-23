use crate::core::data::*;
use crate::core::data::Constraint;
use crate::core::geometry::Number;

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
        let k = Id { data: self.spec.num_unique_vars as usize, description: None };
        self.add_var(Constraint::NewVar(k));
        self.spec.num_unique_vars += 1;
        k
    }

    pub fn new_var_named_(&mut self, s: String) -> Id {
        let k = Id { data: self.spec.num_unique_vars as usize, description: Some(s)};
        self.add_var(Constraint::NewVar(k));
        self.spec.num_unique_vars += 1;
        k
    }

    pub fn new_var_as_const_(&mut self, c: Number) -> Id {
        let a = self.new_var_();
        self.const_(a, c);
        a
        // let k = Id { data: self.spec.num_unique_vars as usize };
        // self.add_var(Var::NewVar(k));
        // self.spec.num_unique_vars += 1;
        // k
    }

    // pub fn Var(&mut self ) {
    //     self.add_constraint(Constraint::Const(a, b));
    // }

    pub fn const_<'a>(&mut self, a: Id, b: Number) {
        self.add_constraint(Constraint::Const(a, b));
    }

    pub fn eq_(&mut self, a: Id, b: Id) {
        self.add_constraint(Constraint::Eq(a, b));
    }

    pub fn eq_val_(&mut self, a: Var, b: Var) {
        self.add_constraint(Constraint::EqualVar(a, b));
    }

    pub fn all(&self) -> Spec {
        Spec::new(
            self.spec.constraints.clone(),
            self.spec.num_constraints,
            // vars: self.spec.vars.clone(),
            self.spec.num_unique_vars,
        )
    } 
}