use std::hash::Hash;

#[derive(Hash, PartialEq, Eq)]
pub struct IVec3(pub i32, pub i32, pub i32);
