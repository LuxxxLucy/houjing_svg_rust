mod core;
mod synthesis;
use crate::core::geometry::{Circle, Rectangle};
use crate::core::proc::Builder;
use crate::core::geometry::Number;
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


    // let library = Library::brahma_std();
    let mut builder = Builder::new();
    // let a = builder.new_var_as_const_(Number::from_int(2));
    // // builder.const_(a, Number::from_int(2));
    // let b = builder.new_var_();
    // builder.eq_(a,b);

    let bg = Rectangle::new(&mut builder);
    let circle = Circle::new(&mut builder);
    let square = Rectangle::new(&mut builder);

    let c1 =builder.new_var_as_const_(Number::from_int(400));
    builder.eq_(bg.width, c1);
    let c2 =builder.new_var_as_const_(Number::from_int(400));
    builder.eq_(bg.height, c2);
    // builder.eq_(circle.center_x(), bg.center_x());
    builder.eq_val_(circle.center_y(), bg.center_y());

    // let g = Group(
    //     [bg, square, circle],
    //     [
    //     // circle is centered
    //     circ.bounds.center == bg.bounds.center,

    //     // circle diameter is 1/2 of canvas width
    //     2*circ.radius == width/2,

    //     // rectangle is centered on circle
    //     rect.bounds.center == circ.bounds.center,

    //     // rectangle is a square
    //     rect.width == rect.height,

    //     // rectangle is circumscribed
    //     rect.width == circ.radius*2**0.5
    //     ]
    // )

    // let render = Renderer(g);

    // let _ = builder.mul(a, b);
    let spec = builder.all();

    // g.solve()
    // let mut p = synthesis::Synthesizer::synthesize(&context, &spec).unwrap();
    // Synthesizer::synthesize(&context, &spec);
    let _ = synthesize(&spec);
    // synthesizer.synthesize();
    // p.dce();
    // println!("{}", p.to_string());

    // println!("{:#?}", model);
    // println!("x:{:#?}", model.eval(&x, true).unwrap().as_real().unwrap());
    // println!("{:#?}", y.as_f32().unwrap());


    // canvas = Canvas(g, WIDTH, HEIGHT, bg_color="#e0e0e0")
    // canvas.save_png("gallery/circle_and_square.png")
    // canvas.save_svg("gallery/circle_and_square.svg")
    println!("{} {} {}", sqrt_2, width, height);

}
