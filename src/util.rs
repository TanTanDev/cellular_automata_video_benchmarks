use std::collections::HashMap;

use crate::ivec3::IVec3;

pub fn prepare_hashmap(size: usize) -> HashMap<IVec3, u8> {
    let mut values_hashmap: HashMap<IVec3, u8> = HashMap::with_capacity(size * size * size);
    (0..size as i32).for_each(|z| {
        (0..size as i32).for_each(|y| {
            (0..size as i32).for_each(|x| {
                let _ = values_hashmap.insert(IVec3(x, y, z), 0u8);
            })
        })
    });
    values_hashmap
}
