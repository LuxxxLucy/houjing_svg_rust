#![allow(dead_code)]

use crate::core::data::*;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;


pub trait Shape {

}

impl Spec {
    pub fn draw(&self) {
        // let document = Document::new().set("viewBox", (0 as i32, 0, 200, 200));

        // for i in self.nodes.iter() {
        //     let n = 
        //     document.add(n)
        // }
        let path = svg::node::element::Circle::new().set("cx", 100 as i32 ).set("cy", 100 as i32).set("r", 50 as i32); 

        let document = Document::new()
            .set("viewBox", (0, 0, 200, 200))
            .add(path);

        svg::save("image.svg", &document).unwrap();
    }
}

// #[cfg(test)]

#[cfg(test)]
mod tests {
    use svg::Document;
    // use svg::node::element::Path;
    use svg::node::element;
    use svg::node::element::path::Data;
    use std::path::{Path, PathBuf};


    fn set_output(file_name: &str) -> PathBuf {
        let d = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let d = Path::new(&d).join("test").join("svg");
        std::fs::create_dir_all(d.clone()).unwrap();
        d.join(file_name)
    }

    #[test]
    fn basic_svg() {
        // DISCLAIMER
        // this the basic demo code of the SVG drawing library that I use, which is "svg"
        let output_path = set_output("basic_svg.svg");

        let data = Data::new()
            .move_to((10, 10))
            .line_by((0, 50))
            .line_by((50, 0))
            .line_by((0, -50))
            .close();

        let path = element::Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", data);

        let document = Document::new()
            .set("viewBox", (0, 0, 70, 70))
            .add(path);
        
        svg::save(output_path.to_str().unwrap(), &document).unwrap();
    }

    #[test]
    fn circle() {
        let output_path = set_output("circle_svg.svg");

        let path = svg::node::element::Circle::new().set("cx", 100).set("cy", 100).set("r", 50); 

        let document = Document::new()
            .set("viewBox", (0, 0, 200, 200))
            .add(path);

        svg::save(output_path.to_str().unwrap(), &document).unwrap();
    }

        #[test]
    fn rectangle() {
        let output_path = set_output("rectangle_svg.svg");

        // let path = svg::node::element::Rectangle::new().set("x", 50 as i32 ).set("y", 50 as i32).set("height", 100 as i32).set("width", 100 as i32); 
        let path = svg::node::element::Rectangle::new().set("x", 50).set("y", 50).set("height", 100).set("width", 100); 

        let document = Document::new()
            .set("viewBox", (0, 0, 200, 200))
            .add(path);

        svg::save(output_path.to_str().unwrap(), &document).unwrap();
    }
}
