mod core;
mod synthesis;
use crate::core::geometry::{Circle, Rectangle};
use crate::core::proc::Builder;
use crate::core::data::*;
use crate::core::SVG::*;
use crate::synthesis::solver;
use crate::solver::synthesize;

fn main() {
    println!("Test Main:");

    let width = 300;
    let height = 300;

    let mut builder = Builder::new();

    let bg = Rectangle::new(&mut builder);
    let circle = Circle::new(&mut builder);
    let rect = Rectangle::new(&mut builder);

    // builder.shapes.add
    builder.eq_val_(bg.x.clone(), 0.to_var());
    builder.eq_val_(bg.y.clone(), 0.to_var());

    builder.eq_val_(bg.width.clone(), width.to_var());
    builder.eq_val_(bg.height.clone(), height.to_var());

    builder.eq_val_(circle.center_x(), bg.center_x());
    builder.eq_val_(circle.center_y(), bg.center_y());

    builder.eq_val_(circle.radius.clone() * 4.to_var(), bg.width.clone());

    builder.eq_val_(circle.center_x(), rect.center_x());
    builder.eq_val_(circle.center_y(), rect.center_y());

    builder.eq_val_(rect.height.clone(), rect.width.clone());
    builder.eq_val_(rect.width.clone(), circle.radius.clone() * 2_f64.sqrt().to_var());

    builder.gt_val_(rect.width.clone(), 0.to_var());
    builder.gt_val_(rect.height.clone(), 0.to_var());

    builder.spec.nodes.push(Box::new(bg));
    builder.spec.nodes.push(Box::new(circle));
    builder.spec.nodes.push(Box::new(rect));

    // let mut spec = builder.all();
    let mut spec = builder.spec;
    let _ = synthesize(&mut spec);

    for i in spec.vars.iter() {
        // let tmp = i.terminal_var.as_ref().unwrap();
        println!("{}", i);
    }

    spec.set_values_to();

    spec.draw();
    // println!("{} {} {}", sqrt_2, width, height);

}
