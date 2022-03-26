#![allow(dead_code)]

use crate::core::data::*;
use crate::core::geometry::*;

use std;


pub type TaggerResult = std::result::Result<(), std::fmt::Error>;
pub trait Shape {
    fn svg_tag(&self) -> &str;
    // fn svg_attr_vals(&self) -> Vec<(String, f64)>;
}

impl Shape for Circle {
    fn svg_tag(&self) -> &str {
        "circle"
    }
    // fn svg_attr_vals(&self) -> Vec<(String, f64)> {

    // }
}

impl Shape for Rectangle {
    fn svg_tag(&self) -> &str {
        "rect"
    }
    // fn svg_attr_vals(&self) -> Vec<(String, f64)> {

    // }
}

fn draw<T: std::fmt::Write>(w: &mut tagger::ElemWriter<T>, x: &dyn Shape) -> TaggerResult {
    let attrs = vec!(("x", 0), ("y", 0), ("width", 100), ("height", 100));
    w.single(x.svg_tag(), |d| {
        for attr in attrs.iter() {
            d.attr(attr.0, attr.1)?;
        }
        d.attr("style", "fill:none; stroke: black; stroke-width: 3")
    })
}

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
            let i_ref: &dyn Shape = i.as_ref();
            draw(w, i_ref)?;
            // w.single("rect", |d| {
            //     d.attr("x", 0)?;
            //     d.attr("y", 0)?;
            //     d.attr("width", 100)?;
            //     d.attr("height", 100)?;
            //     d.attr("style", "fill:blue")
            // })?;
        }
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
            .build(|w| w.put_raw("{fill:none;stroke:white;stroke-width:3}"))?;

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
