#![allow(dead_code)]

use crate::core::data::*;
use crate::core::geometry::*;

use std;


pub type TaggerResult = std::result::Result<(), std::fmt::Error>;
pub trait Shape {
    // fn draw<T: std::fmt::Write>(&self, w: &mut tagger::ElemWriter<T>) -> TaggerResult;
}

impl Shape for Circle {
    // fn draw(&self) -> Box<element::Circle> {
//     fn draw(&self) -> Box<dyn svg::node::Node> {
//         Box::new(element::Circle::new().set("cx", 100 as i32 ).set("cy", 100 as i32).set("r", 50 as i32))
//     }
}

//     // fn draw(&self) -> element::Rectangle {
//     fn draw(&self) -> Box<dyn svg::node::Node> {
//         Box::new(svg::node::element::Rectangle::new().set("x", 50 as i32).set("y", 50 as i32).set("height", 100 as i32).set("width", 100 as i32))
//     }
impl Shape for Rectangle {
}

// impl geometry::Circle {
// }

// impl geometry::Rectangle {
// }

// impl Var {
//     pub fn get_val(&mut self) -> f64 {
//         self.terminal_var.as_mut().unwrap().val.unwrap().into()
//     }
// }

// impl Rectangle {
// impl Shape for Rectangle {
//     fn draw<T: std::fmt::Write>(&self, w: &mut tagger::ElemWriter<T>) -> TaggerResult {
//         w.single("rect", |d| {
//             d.attr("x", 0)?;
//             d.attr("y", 0)?;
//             // d.attr("rx", 20)?;
//             // d.attr("ry", 20)?;
//             d.attr("width", 100)?;
//             d.attr("height", 100)?;
//             d.attr("style", "fill:blue")
//         })
//     }
// }

impl Spec {
    pub fn draw(&self) -> TaggerResult {

    let width = 100.0;
    let height = 100.0;
    let buffer = std::fs::File::create("test_new.svg").unwrap();
    let buffer = std::io::BufWriter::new(buffer);
    let mut w = tagger::new(tagger::upgrade_write(buffer));

    w.elem("svg", |d| {
        d.attr("xmlns", "http://www.w3.org/2000/svg")?;
        d.attr("viewBox", format_args!("0 0 {} {}", width, height))
    })?
    .build(|w| {
        for i in self.nodes.iter() {
            w.single("rect", |d| {
                d.attr("x", 0)?;
                d.attr("y", 0)?;
                // d.attr("rx", 20)?;
                // d.attr("ry", 20)?;
                d.attr("width", 100)?;
                d.attr("height", 100)?;
                d.attr("style", "fill:blue")
            })?;
        }

        w.elem("style", tagger::no_attr())?
            .build(|w| w.put_raw(".test{fill:none;stroke:white;stroke-width:3}"))?;
        
        Ok(())
    })
    }
}

// #[cfg(test)]

#[cfg(test)]
mod tests {

    fn set_output(file_name: &str) -> std::path::PathBuf {
        let d = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let d = std::path::Path::new(&d).join("test").join("svg");
        std::fs::create_dir_all(d.clone()).unwrap();
        d.join(file_name)
    }

    #[test]
    pub fn test_tagger() -> std::result::Result<(), std::fmt::Error> {
        // DISCLAIMER
        // this is the sample code of [Tagger](https://github.com/tiby312/tagger), which is used


        let file_name = set_output("tagger_test.svg");

    let width = 100.0;
    let height = 100.0;
    let buffer = std::fs::File::create("test.svg").unwrap();
    let buffer = std::io::BufWriter::new(buffer);
    let mut w = tagger::new(tagger::upgrade_write(buffer));

    w.elem("svg", |d| {
        d.attr("xmlns", "http://www.w3.org/2000/svg")?;
        d.attr("viewBox", format_args!("0 0 {} {}", width, height))
    })?
    .build(|w| {
        w.single("rect", |d| {
            d.attr("x1", 0)?;
            d.attr("y1", 0)?;
            d.attr("rx", 20)?;
            d.attr("ry", 20)?;
            d.attr("width", width)?;
            d.attr("height", height)?;
            d.attr("style", "fill:blue")
        })?;

        w.elem("style", tagger::no_attr())?
            .build(|w| w.put_raw(".test{fill:none;stroke:white;stroke-width:3}"))?;

        w.elem("g", |d| d.attr("class", "test"))?.build(|w| {
            for r in (0..50).step_by(10) {
                w.single("circle", |w| {
                    w.attr("cx", 50.0)?;
                    w.attr("cy", 50.0)?;
                    w.attr("r", r)
                })?;
            }
            Ok(())
        })
    })
    }
}
