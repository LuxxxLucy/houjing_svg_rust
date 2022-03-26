use crate::core::data::*;
use crate::core::data::Constraint;
use crate::core::SVG::Shape;

pub struct Builder {
    pub spec: Spec,
    // components: Vec<Box<dyn Shape>>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            spec: Spec::empty(),
            // components: Vec<Box<dyn Shape>>::new()
        }
    }

    pub fn add_constraint(&mut self, c: Constraint) {
        self.spec.constraints.push(c);
        self.spec.num_constraints += 1;
    }

    // pub fn add_node(&mut self, c: &dyn Shape) {
    //     self.spec.nodes.push(Box::new(c.clone()));
    // }

    pub fn add_var(&mut self, c: Var) {
        self.spec.vars.push(c);
        // self.add_constraint(c);
    }

    pub fn new_var_(&mut self) -> Var {
        // let k = Id { data: self.spec.num_unique_vars as usize, description: None };
        let id = self.spec.num_unique_vars as usize;
        let k = Id::new(id);
        let var = k.to_var();
        self.add_var(var.clone());
        self.spec.num_unique_vars += 1;
        var 
    }

    pub fn new_var_named_(&mut self, s: &str) -> Var {
        let id = self.spec.num_unique_vars as usize;
        let k = Id::new_named(id, String::from(s));
        let var = k.to_var();
        self.add_var(var.clone());
        self.spec.num_unique_vars += 1;
        var 
    }

    pub fn eq_val_(&mut self, a: Var, b: Var) {
        self.add_constraint(Constraint::EqualVar(a, b));
    }

    pub fn all(&self) -> Spec {
        Spec::new(
            self.spec.constraints.clone(),
            self.spec.num_constraints,
            self.spec.vars.clone(),
            self.spec.num_unique_vars,
            // self.spec.nodes
        )
    } 
}