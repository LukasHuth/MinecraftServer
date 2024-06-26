use datatypes::{BitMask, Float, UnsignedByte};

#[allow(missing_docs)]
pub struct PlayerInput {
    pub sideways: Float,
    pub forward: Float,
    pub flags: BitMask<UnsignedByte, player_input_extra::Flags>,
}
#[allow(missing_docs)]
pub mod player_input_extra {
    use datatypes::ToBitPos;

    pub enum Flags {
        Jump,
        Unmount,
    }
    impl ToBitPos for Flags {
        fn to_bit_pos(&self) -> u64 {
            match self {
                Self::Jump => 0,
                Self::Unmount => 1,
            }
        }

        fn iterator<'a>() -> std::slice::Iter<'a, Self> where Self: 'a + Sized {
            use Flags::*;
            static LIST: [Flags;2] = [Jump, Unmount];
            LIST.iter()
        }
    }
}
