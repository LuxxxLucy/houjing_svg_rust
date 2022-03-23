#![allow(dead_code)]

use crate::core::shape::*;
use crate::core::data::*;
use crate::core::proc::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Number {
    // representing a real number = num / den
    pub num: i32,
    pub den: i32,
}

impl Number {
    pub fn new(n: i32, d: i32) -> Self {
        Number {
            num: n,
            den: d
        }
    }

    pub fn from_int(i: i32) -> Self {
        Number::new(i,1)
    }

    pub fn from_real(n: i32, d: i32) -> Self {
        Number::new(n,d)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.2} ({}/{})", (self.num/self.den) as f64, self.num, self.den)
    }
}

pub struct Point {
    x: Id,
    y: Id,
}

impl Point {
    pub fn new(builder: &mut Builder) -> Self {
        Point {
            x: builder.new_var_(),
            y: builder.new_var_(),
        }
    }
}

impl Shape for Point {
//     fn bound(&self) -> Bound {
//         Bound {
//             left: self.x,
//             right: self.x,
//             top: self.y,
//             bottom: self.y,
//         }
//     }

//     fn solve(&self) {}
}

// pub struct Circle<'a> {
pub struct Circle {
    x: Id,
    y: Id,
    radius: Id,
    // builder: &'a Builder, 
}

impl Circle {
    pub fn new(builder: &mut Builder) -> Self {
        // let y = ast::Real::new_const(&ctx, "y");
        Circle {
            x: builder.new_var_(),
            y: builder.new_var_(),
            radius: builder.new_var_(),
            // builder: builder
        }
    }
}

impl Circle {
    pub fn center_x(&self) -> Var {
        let l = Var::from_id(&self.x);
        let r = Var::from_id(&self.radius);
        Var::new(Operator::Add, l, r)
    }
    pub fn center_y(&self) -> Var {
        let l = Var::from_id(&self.y);
        let r = Var::from_id(&self.radius);
        Var::new(Operator::Add, l, r)
    }
}

impl Shape for Circle {
//     fn bound(&self) -> Bound {
//         Bound {
//             left: self.x - self.radius,
//             right: self.x + self.radius,
//             top: self.y - self.radius,
//             bottom: self.y + self.radius,
//         }
//     }

//     fn solve(&self) {}
}

pub struct Rectangle {
    pub x: Id,
    pub y: Id,
    pub width: Id,
    pub height: Id,
    // builder: &'a Builder, 
}

impl Rectangle {
    pub fn new(builder: &mut Builder) -> Self {
        Rectangle {
            x: builder.new_var_named_("rectangle"),
            // x: builder.new_var_(),
            y: builder.new_var_(),
            width: builder.new_var_(),
            height: builder.new_var_(),
            // builder: builder
        }
    }
}

impl Rectangle {
    pub fn center_x(&self) -> Var {
        let l = Var::from_id(&self.x);
        let r = Var::from_id(&self.width);
        Var::new(Operator::Add, l, r)
    }
    pub fn center_y(&self) -> Var {
        let l = Var::from_id(&self.y);
        let r = Var::from_id(&self.height);
        Var::new(Operator::Add, l, r)
    }
}

impl Shape for Rectangle {
//     fn bound(&self) -> Bound {
//         Bound {
//             left: self.x,
//             right: self.x + self.width,
//             top: self.y,
//             bottom: self.y + self.height,
//         }
//     }

//     fn solve(&self) {}
}