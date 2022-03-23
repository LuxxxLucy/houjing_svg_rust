
// use crate::core::shape::*;

// pub struct Collection {
//     instances: Vec<Box<dyn Shape>>,
//     // constraint: Vec<Box<Constraint>>,
// }

// impl Shape for Collection {
//     fn bound(&self) -> Bound {

//         let lefts: Vec<_> = self.instances.iter().map(|a| {let t = a.bound(); t.left}).collect();
//         let rights: Vec<_> = self.instances.iter().map(|a| {let t = a.bound(); t.right}).collect();
//         let tops: Vec<_> = self.instances.iter().map(|a| {let t = a.bound(); t.top}).collect();
//         let bottoms : Vec<_> = self.instances.iter().map(|a| {let t = a.bound(); t.bottom}).collect();

//         Bound {
//             left: lefts.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
//             right: rights.iter().fold(f64::INFINITY, |a, &b| a.max(b)),
//             top: tops.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
//             bottom: bottoms.iter().fold(f64::INFINITY, |a, &b| a.max(b)),
//         }
//     }

//     fn solve(&self) {
//     }
// }