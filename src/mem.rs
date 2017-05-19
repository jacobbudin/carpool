use std::mem;

pub fn get_size_of_string(s: &String) -> usize {
    return s.len() * mem::size_of::<u8>();
}
