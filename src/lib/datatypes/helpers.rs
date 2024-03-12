macro_rules! create_data_type {
    ($name:ident, $previous:ty, $size:expr) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $name($previous);
        impl $name {
            const MAX_VALUE: $previous = (1 << ($size-1)) - 1;
            const NEG_MASK: $previous = ((Self::MAX_VALUE<<1)+1) ^ Self::MAX_VALUE;

        pub fn new(value: $previous) -> Option<Self> {
            if value <= Self::MAX_VALUE {
                Some(Self((value & Self::MAX_VALUE) * if (value&Self::NEG_MASK) == 0 { 1 } else { -1 }))
            } else {
                None
            }
        }

        pub fn value(&self) -> $previous {
            self.0
        }
        pub fn as_data(&self) -> $previous {
            self.0 & Self::MAX_VALUE | if self.0 < 0 { Self::NEG_MASK } else { 0 }
        }
        }
    };
}
create_data_type!(I26, i32, 26);
create_data_type!(I12, i16, 12);
