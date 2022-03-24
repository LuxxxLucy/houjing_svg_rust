// #![allow(dead_code)]

use crate::core::data::*;
use crate::core::proc::Builder;

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

pub struct Circle {
    pub x: Id,
    pub y: Id,
    pub radius: Id,
}

impl Circle {
    pub fn new(builder: &mut Builder) -> Self {
        Circle {
            x: builder.new_var_(),
            y: builder.new_var_(),
            radius: builder.new_var_(),
        }
    }
}

impl Circle {
    pub fn center_x(&self) -> Var {
        self.x.to_var()
    }
    pub fn center_y(&self) -> Var {
        self.y.to_var()
    }
}

pub struct Rectangle {
    pub x: Id,
    pub y: Id,
    pub width: Id,
    pub height: Id,
}

impl Rectangle {
    pub fn new(builder: &mut Builder) -> Self {
        Rectangle {
            x: builder.new_var_named_("rectangle"),
            // x: builder.new_var_(),
            y: builder.new_var_(),
            width: builder.new_var_(),
            height: builder.new_var_(),
        }
    }
}

impl Rectangle {
    pub fn center_x(&self) -> Var {
        let l = self.x.to_var();
        let r = self.width.to_var();
        let t = l + r;
        let c = 2.to_var();
        t / c
    }
    pub fn center_y(&self) -> Var {
        let l = self.y.to_var();
        let r = self.height.to_var();
        let t = l + r;
        let c = 2.to_var();
        t / c
    }
}