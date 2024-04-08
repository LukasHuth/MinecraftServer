use super::Display;

pub struct BlockDisplay {
    display: Display,
    block_state: i32,
}
impl Default for BlockDisplay {
    fn default() -> Self {
        Self { display: Display::default(), block_state: 0 }
    }
}
