use std::ops::{Add, AddAssign, Div, Sub, Mul};
use std::fmt;


pub trait Num: Add<Self, Output=Self> + AddAssign<Self> + Sub<Self, Output=Self> +
    Div<Self, Output=Self> + Mul<Self> + Copy {}

pub struct UViewPacket<T: Num> {
    value: T
}

pub type ValueType = u64;

impl Num for ValueType {}
impl UViewPacket<ValueType> {
    pub fn new() -> UViewPacket<ValueType> {
        UViewPacket { value: 0 as ValueType }
    }
}

impl<T> UViewPacket<T> where T: Num {
    pub fn scale(self, min: T, max: T) -> UViewPacket<T> {
        UViewPacket { value: (self.value - min) / (max - min) }
    }
}

impl<T> ToString for UViewPacket<T> where T: Num + ToString {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl<T> fmt::Debug for UViewPacket<T> where T: Num + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value: {:5}", self.value)
    }
}

impl<T> Add<T> for UViewPacket<T> where T: Num {
    type Output = UViewPacket<T>;
    fn add(self, other: T) -> UViewPacket<T> {
        UViewPacket { value: self.value + other }
    }
}

impl<T> AddAssign<T> for UViewPacket<T> where T: Num {
    fn add_assign(&mut self, other: T) {
        self.value += other;
    }
}
