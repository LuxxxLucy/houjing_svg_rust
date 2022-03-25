#![allow(dead_code)]

use crate::core::data::*;
use crate::core::shape::Shape;
use crate::core::proc::Builder;

pub struct Point {
    x: Var,
    y: Var,
}

impl Point {
    pub fn new(builder: &mut Builder) -> Self {
        Point {
            x: builder.new_var_(),
            y: builder.new_var_(),
        }
    }
}

pub struct Circle {
    pub x: Var,
    pub y: Var,
    pub radius: Var,
}

impl Circle {
    pub fn new(builder: &mut Builder) -> Self {
        Circle {
            x: builder.new_var_named_("circle x"),
            y: builder.new_var_named_("circle y"),
            radius: builder.new_var_named_("circle radius"),
        }
    }
}

impl Circle {
    pub fn center_x(&self) -> Var {
        self.x.clone()
    }
    pub fn center_y(&self) -> Var {
        self.y.clone()
    }
}

pub struct Rectangle {
    pub x: Var,
    pub y: Var,
    pub width: Var,
    pub height: Var,
}

impl Rectangle {
    pub fn new(builder: &mut Builder) -> Self {
        Rectangle {
            x: builder.new_var_named_("rectangle x"),
            // x: builder.new_var_(),
            y: builder.new_var_named_("rectangle y"),
            width: builder.new_var_named_("rectangle width"),
            height: builder.new_var_named_("rectangle height"),
        }
    }
}

impl Rectangle {
    pub fn center_x(&self) -> Var {
        let l = self.x.clone();
        let r = self.width.clone();
        let t = l + r;
        let c = 2.to_var();
        t / c
    }
    pub fn center_y(&self) -> Var {
        let l = self.y.clone();
        let r = self.height.clone();
        let t = l + r;
        let c = 2.to_var();
        t / c
    }
}

impl Shape for Rectangle {

}