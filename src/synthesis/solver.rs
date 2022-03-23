use crate::core::data::{*};
use crate::core::geometry::Number;
use z3;
use z3::ast::{Ast, Real, Bool};
pub use slotmap::{Key, new_key_type, SlotMap, DefaultKey};
use std::collections::HashMap;

// #[enum_dispatch(Constraint)]
// trait Make {
//     fn make(&self) -> Real<'a>;
// }

// impl Make
//   // let f = match c {
//         //     Constraint::NewVar(id) => {
//         //         let v = z3::ast::Real::new_const(context, id.to_string());
//         //         let k = slot_map.insert(v);
//         //         keys.insert(id.to_string(), k);
//         //         Bool::from_bool(context, true)
//         //     }
//         //     Constraint::Const(id, value) => { 
//         //         // z3::ast::Real 
//         //         let v = &slot_map[keys[&id.to_string()]];
//         //         let const_value = value.to_z3_real(context);
//         //         v._eq(&const_value)
//         //     }
//         //     Constraint::Eq(id1, id2) => {
//         //         let v1 = &slot_map[keys[&id1.to_string()]];
//         //         let v2 = &slot_map[keys[&id2.to_string()]];
//         //         v1._eq(&v2)
//         //     }
//         // };

impl<'a> Number {
    pub fn to_z3_real(&self, context: &'a z3::Context) -> Real<'a> {
        z3::ast::Real::from_real(context, self.num, self.den)
    }
}

pub fn get_value<'a>(var: &Var, context: &'a z3::Context, sm: &'a SlotMap<DefaultKey, Real<'a>>, keys: &HashMap<String, DefaultKey>) 
// -> &'a Real<'a> {
-> Real<'a> {
    match var.var_type {
        VarType::Undefined => { panic!(); }
        VarType::Terminal => {
            let id = var.terminal_var.as_ref().unwrap().id;
            // &sm[keys[&id.to_string()]]
            sm[keys[&id.to_string()]].clone()
        }
        VarType::Intermediate => {
            // panic!();
            let v = var.intermediate_var.as_ref().unwrap();
            let l = get_value(&v.l, context, sm, keys);
            // l = sm.insert(l);
            // l = sm[l];
            let r = get_value(&v.r, context, sm, keys);
            // r = sm.insert(r);
            // r = sm[r];
            match v.op {
                Operator::Add => { 
                    // &l.add(r) 
                    // &z3::ast::Real::<'a>::add(context,&[l,r])
                    z3::ast::Real::<'a>::add(context,&[&l,&r])
                }
                Operator::Sub => { 
                    // &l.add(r) 
                    // &z3::ast::Real::<'a>::add(context,&[l,r])
                    z3::ast::Real::<'a>::add(context,&[&l,&r])
                }
            }
        }
    }
}

pub fn synthesize(
    // context: &'a z3::Context,
    spec: &Spec,
    // library: &Library,
// ) {
) -> Result<usize> {
    println!("synthesize");
    let mut config = z3::Config::new();
    config.set_model_generation(true);
    let context = &z3::Context::new(&config);
    let mut synthesizer = Synthesizer::new(context, spec).unwrap();
    synthesizer.summary();
    let mut slot_map = SlotMap::<DefaultKey, Real<'_>>::new(); 
    // Slot map stores the variables

    let mut keys = HashMap::new(); // Id to Slotmap Key

    // declare variables
    // // for c in spec.vars.iter() {
    //     Var::NewVar(id) => {
    //         let v = z3::ast::Real::new_const(context, id.to_string());
    //         let k = slot_map.insert(v);
    //         keys.insert(id.to_string(), k);
    //     }
    for c in spec.constraints.iter() {
        match c {
            Constraint::NewVar(id) => {
                    let v = z3::ast::Real::new_const(context, id.to_string());
                    let k = slot_map.insert(v);
                    keys.insert(id.to_string(), k);
            }
            Constraint::Const(id, value) => { }
            Constraint::Eq(id1, id2) => { }
            Constraint::EqualVar(var1, var2) => { }
        };
    }

    fn id_to_val<'a>(slotmap: &'a SlotMap<DefaultKey, Real<'a>>, keys: &HashMap<String, DefaultKey>, id: &Id) -> &'a Real<'a> 
        {  
            &slotmap[keys[&id.to_string()]] 
        }

    // declare constraints  
    let mut formulas = Vec::<Bool<'_>>::new();
    for c in spec.constraints.iter() {
    // for c in spec.constraints {
        let f = match c {
            Constraint::NewVar(id) => {
                // {
                    // let v = z3::ast::Real::new_const(context, id.to_string());
                    // let k = slot_map.insert(v);
                    // keys.insert(id.to_string(), k);
                // }
                Bool::from_bool(context, true)
            }
            Constraint::Const(id, value) => { 
                // z3::ast::Real 
                let v = &slot_map[keys[&id.to_string()]];
                let const_value = value.to_z3_real(context);
                v._eq(&const_value)
            }
            Constraint::Eq(id1, id2) => {
                // let v = Var::from_id(*id1);
                // let v1 = get_value(v, &context, &slot_map, &keys);
                // let v = Var::from_id(*id2);
                // let v2 = get_value(v, &context, &slot_map, &keys);

                let v1 = &slot_map[keys[&id1.to_string()]];
                let v2 = &slot_map[keys[&id2.to_string()]];

                // let v1 = id_to_val(&slot_map, &keys, id1);
                // let v2 = id_to_val(&slot_map, &keys, id2);
                v1._eq(&v2)
            }
            Constraint::EqualVar(var1, var2) => {
                let v1 = get_value(var1, &context, &slot_map, &keys);
                let v2 = get_value(var2, &context, &slot_map, &keys);

                // let v1 = id_to_val(&slot_map, &keys, id1);
                // let v2 = id_to_val(&slot_map, &keys, id2);
                v1._eq(&v2)
            }
        };
        // let f = c.make();
        formulas.push(f);
    }

    let solver = synthesizer.solver();
    let query = and(context, formulas.iter());
    solver.assert(&query);

    match solver.check() {
        z3::SatResult::Unknown => Err(Error::SynthesisUnknown),
        z3::SatResult::Unsat => Err(Error::SynthesisUnsatisfiable),
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            for id in 0..spec.num_unique_vars {
                let v = &slot_map[keys[&id.to_string()]];
                let tmp = model.eval(v, true).unwrap().as_real();
                if let Some(val) = tmp {
                    println!("Id {}: {:#?}", id, val);
                } else {
                    println!("Id {}: not exist", id);
                }
            }
            Ok(0)
        }
    }
}

fn and<'a, 'b>(context: &'a z3::Context, exprs: impl IntoIterator<Item = &'b Bool<'a>>) -> Bool<'a>
where
    'a: 'b,
{
    let exprs: Vec<&_> = exprs.into_iter().collect();
    // let m = Bool::from_bool(context, true);
    Bool::and(context, &exprs)
}

#[derive(Debug)]
pub struct Synthesizer<'a> {
    context: &'a z3::Context,
    // spec: &'a dyn Specification,
    spec: &'a Spec,
    // library: &'a Library,
    // locations: LocationVars<'a>,
    // well_formed_program: Bool<'a>,
    // invalid_connections: HashSet<(u32, u32)>,
    // not_invalid_assignments: Bool<'a>,
    // should_synthesize_minimal_programs: bool,
    // timeout: Option<Timeout>,
}

impl<'a> Synthesizer<'a> {
    pub fn new(
        context: &'a z3::Context,
        // library: &'a Library,
        // spec: &'a dyn Specification,
        spec: &'a Spec,
    ) -> Result<Self> {
        // if library.components.is_empty() {
        //     return Err(Error::NoComponents);
        // }

        // let locations = LocationVars::new(context, library, spec.arity());
        // let mut invalid_connections = locations.invalid_connections(library);
        // let well_formed_program =
        //     locations.well_formed_program(context, library, &mut invalid_connections);
        // let not_invalid_assignments = Bool::from_bool(context, true);
        Ok(Synthesizer {
            context,
            // library,
            spec,
            // locations,
            // well_formed_program,
            // invalid_connections,
            // not_invalid_assignments,
            // should_synthesize_minimal_programs: false,
            // timeout: None,
        })
    }

    pub fn summary(&self) {
        println!("context {:#?}\nSpec {:#?}", self.context, self.spec);
    }

    fn solver(&mut self) -> z3::Solver<'a> {
        let solver = z3::Solver::new(self.context);
        // if let Some(timeout) = self.timeout.clone() {
        //     let millis = match timeout {
        //         Timeout::Duration(d) => {
        //             let millis = d.as_millis();
        //             self.timeout = Some(Timeout::Instant(time::Instant::now() + d));
        //             millis as u32
        //         }
        //         Timeout::Instant(instant) => {
        //             let dur = instant.saturating_duration_since(time::Instant::now());
        //             dur.as_millis() as u32
        //         }
        //     };

        //     let mut params = z3::Params::new(self.context);
        //     params.set_u32("timeout", millis);

        //     solver.set_params(&params);
        // }
        solver
    }

}