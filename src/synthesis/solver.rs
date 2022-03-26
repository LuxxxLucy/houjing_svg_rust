use crate::core::data::{*};
use z3;
use z3::ast::{Ast, Real, Bool};
pub use slotmap::{Key, new_key_type, SlotMap, DefaultKey};
use std::collections::HashMap;

impl<'a> Number {
    pub fn to_z3_real(&self, context: &'a z3::Context) -> Real<'a> {
        z3::ast::Real::from_real(context, self.num, self.den)
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

pub fn get_value<'a>(var: &Var, context: &'a z3::Context, sm: &'a SlotMap<DefaultKey, Real<'a>>, keys: &HashMap<Id, DefaultKey>) 
// -> &'a Real<'a> {
-> Real<'a> {
    match var.var_type {
        VarType::Undefined => { panic!("undefined var!"); }
        VarType::Inferred => { panic!("inferred var!"); }
        VarType::Terminal => {
            let id = var.terminal_var.as_ref().unwrap().id.clone();
            sm[keys[&id]].clone()
        }
        VarType::Constant => {
            let constant = var.constant_var.as_ref().unwrap(); 
            constant.val.to_z3_real(context)
        }
        VarType::Intermediate => {
            let v = var.intermediate_var.as_ref().unwrap();
            let l = get_value(&v.l, context, sm, keys);
            let r = get_value(&v.r, context, sm, keys);
            match v.op {
                Operator::Add => { 
                    z3::ast::Real::<'a>::add(context,&[&l,&r])
                }
                Operator::Sub => { 
                    z3::ast::Real::<'a>::sub(context,&[&l,&r])
                }
                Operator::Div => { 
                    l.div(&r)
                }
                Operator::Mul=> { 
                    z3::ast::Real::<'a>::mul(context,&[&l,&r])
                }
            }
        }
    }
}

pub fn synthesize(
    // context: &'a z3::Context,
    spec: &mut Spec,
    // library: &Library,
// ) {
) -> Result<usize> {
    println!("synthesize");
    let mut config = z3::Config::new();
    config.set_model_generation(true);
    let context = &z3::Context::new(&config);
    let mut synthesizer = Synthesizer::new(context).unwrap();
    let mut slot_map = SlotMap::<DefaultKey, Real<'_>>::new(); 
    // Slot map stores the variables

    let mut keys = HashMap::new(); // Id to Slotmap Key

    println!("initialize the vars");
    for c in spec.vars.iter() {
        println!("{:#?}",&c);
        if let Some(v) = &c.terminal_var {
            let id = &v.id;
            println!("string of id is {}", id.to_string());
            let v = z3::ast::Real::new_const(context, id.to_string());
            let k = slot_map.insert(v);
            keys.insert(id.clone(), k);
        } else {
            println!("{:#?}",c);
        }
        println!("{:#?}",c);
    }


    println!("start solving");
    // declare constraints  
    let mut formulas = Vec::<Bool<'_>>::new();
    for c in spec.constraints.iter() {
    // for c in spec.constraints {
        let f = match c {
            Constraint::Const(id, value) => { 
                // z3::ast::Real 
                let v = &slot_map[keys[&id]];
                let const_value = value.to_z3_real(context);
                v._eq(&const_value)
            }
            Constraint::EqualVar(var1, var2) => {
                let v1 = get_value(var1, &context, &slot_map, &keys);
                let v2 = get_value(var2, &context, &slot_map, &keys);
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
            synthesizer.summary();
            let model = solver.get_model().unwrap();
            println!("number of unique vars {}", spec.num_unique_vars);
            // for idx in 0..spec.num_unique_vars {
            for var in spec.vars.iter_mut() {
                assert_eq!(var.var_type, VarType::Terminal);
                let v = var.terminal_var.as_mut().unwrap();
                let id = v.id.clone();
                let k = keys[&id];
                let z3_val = &slot_map[k];
                let tmp = model.eval(z3_val, true).unwrap().as_real();
                if let Some(val) = tmp {
                    let val = Number::from( (val.0 as f64)/(val.1 as f64));
                    v.val = Some(val);
                } else {
                    println!("Id {}: not exist", id);
                }
            }
            Ok(0)
        }
    }
}

#[derive(Debug)]
pub struct Synthesizer<'a> {
    context: &'a z3::Context,
    // spec: &'a dyn Specification,
    // spec: &'a Spec,
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
        // spec: &'a Spec,
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
            // spec,
            // locations,
            // well_formed_program,
            // invalid_connections,
            // not_invalid_assignments,
            // should_synthesize_minimal_programs: false,
            // timeout: None,
        })
    }

    pub fn summary(&self) {
        println!("context {:#?}", self.context);
        // println!("Spec {}", self.spec);
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