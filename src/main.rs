mod core;
mod synthesis;
use crate::core::geometry::{Circle, Rectangle};
use crate::core::proc::Builder;
use crate::core::data::*;
use crate::synthesis::solver;
use crate::solver::synthesize;

fn main() {
    println!("Test Main:");

    // let SQRT_2 = std::num::sqrt(2);
    let sqrt_2 = 2_f64.sqrt();
    // 2**0.5;

    let width = 300;
    let height = 300;

    // CIRCLE_STYLE = {"stroke": "#0000ff", "fill_opacity": "0"}
    // RECT_STYLE = {"stroke": "#ff0000", "fill_opacity": "0"}

    // circle = Circle(style=CIRCLE_STYLE)
    // square = Rectangle(style=RECT_STYLE)

    // g = Group([circle, square], [
    //     circle.center |Eq| square.center,
    //     circle.radius |Eq| WIDTH / 4,
    //     square.width |Eq| square.height,
    //     square.width |Eq| circle.radius * SQRT_2
    // ])

    let mut builder = Builder::new();

    let bg = Rectangle::new(&mut builder);
    let circle = Circle::new(&mut builder);
    let rect = Rectangle::new(&mut builder);

    // builder.shapes.add
    builder.eq_val_(bg.x.clone(), 0.to_var());
    builder.eq_val_(bg.y.clone(), 0.to_var());

    builder.eq_val_(bg.width.clone(), 400.to_var());
    builder.eq_val_(bg.height.clone(), 400.to_var());

    builder.eq_val_(circle.center_x(), bg.center_x());
    builder.eq_val_(circle.center_y(), bg.center_y());

    builder.eq_val_(circle.radius.clone() * 2.to_var(), bg.width.clone() / 2.to_var());

    builder.eq_val_(circle.center_x(), rect.center_x());
    builder.eq_val_(circle.center_y(), rect.center_y());

    builder.eq_val_(rect.height.clone(), rect.width.clone());
    builder.eq_val_(rect.width.clone(), circle.radius.clone() * 2_f64.sqrt().to_var());

    let mut spec = builder.all();
    let _ = synthesize(&mut spec);

    for var in spec.vars.iter_mut() {
        let v = var.terminal_var.as_ref().unwrap();
        // println!("Id {}: {}", &v.id, &v.val);
        // println!("{}", v)
        println!("{}", v.val.as_ref().unwrap())
    }

    // println!("{}", spec);
    // let d = drawer::new(result);
    // bg.forward(&spec);
    spec.draw();

    // let data = Data::new()
    //     .move_to((10, 10))
    //     .line_by((0, 50))
    //     .line_by((50, 0))
    //     .line_by((0, -50))
    //     .close();

    // let path = Path::new()
    //     .set("fill", "none")
    //     .set("stroke", "black")
    //     .set("stroke-width", 3)
    //     .set("d", data);

    // circle.draw(&d);
    // rect.draw(&d);
    // d.finish("output.svg")
    // canvas = Canvas(g, WIDTH, HEIGHT, bg_color="#e0e0e0")
    // canvas.save_png("gallery/circle_and_square.png")
    // canvas.save_svg("gallery/circle_and_square.svg")
    println!("{} {} {}", sqrt_2, width, height);

}
