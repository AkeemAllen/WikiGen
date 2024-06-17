use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeEffectiveness(pub HashMap<String, Vec<String>>);

#[allow(dead_code)]
const N: f32 = 1.0;
#[allow(dead_code)]
const H: f32 = 0.5;
// const X: f32 = f32::NAN;
#[allow(dead_code)]
const D: f32 = 2.0;
#[allow(dead_code)]
const Z: f32 = 0.0;

#[allow(dead_code)]
pub const GEN_DEFAULT: [[f32; 18]; 18] = [
    [N, N, N, N, N, N, N, N, N, N, N, N, H, Z, N, N, H, N],
    [N, H, H, N, D, D, N, N, N, N, N, D, H, N, H, N, D, N],
    [N, D, H, N, H, N, N, N, D, N, N, N, D, N, H, N, N, N],
    [N, N, D, H, H, N, N, N, Z, D, N, N, N, N, H, N, N, N],
    [N, H, D, N, H, N, N, H, D, H, N, H, D, N, H, N, H, N],
    [N, H, H, N, D, H, N, N, D, D, N, N, N, N, D, N, H, N],
    [D, N, N, N, N, D, N, H, N, H, H, H, D, Z, N, D, D, H],
    [N, N, N, N, D, N, N, H, H, N, N, N, H, H, N, N, Z, D],
    [N, D, N, D, H, N, N, D, N, Z, N, H, D, N, N, N, D, N],
    [N, N, N, H, D, N, D, N, N, N, N, D, H, N, N, N, H, N],
    [N, N, N, N, N, N, D, D, N, N, H, N, N, N, N, Z, H, N],
    [N, H, N, N, D, N, H, H, N, H, D, N, N, H, N, D, H, H],
    [N, D, N, N, N, D, H, N, H, D, N, D, N, N, N, N, H, N],
    [Z, N, N, N, N, N, N, N, N, N, D, N, N, D, N, H, N, N],
    [N, N, N, N, N, N, N, N, N, N, N, N, N, N, D, N, H, Z],
    [N, N, N, N, N, N, H, N, N, N, D, N, N, D, N, H, N, H],
    [N, H, H, H, N, D, N, N, N, N, N, N, D, N, N, N, H, D],
    [N, H, N, N, N, N, D, H, N, N, N, N, N, N, D, D, H, N],
];
