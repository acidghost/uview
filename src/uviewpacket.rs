use std::ops::{Add, AddAssign, Div, Sub, Mul};
use std::fmt;


pub trait Num: Add<Self, Output=Self> + AddAssign<Self> + Sub<Self, Output=Self> +
    Div<Self, Output=Self> + Mul<Self> + Copy {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMode {
    Chart,
    Font
}

pub struct UViewPacket<T: Num> {
    value: T,
    display_mode: DisplayMode
}

pub type ValueType = u64;


#[inline]
pub fn scale<T: Num>(value: T, min: T, max: T) -> T {
    (value - min) / (max - min)
}


impl Num for ValueType {}
impl UViewPacket<ValueType> {
    pub fn zero(&mut self) {
        self.value = 0 as ValueType;
    }
    pub fn scale(&mut self, min: ValueType, max: ValueType) {
        self.value = scale(self.value, min, max);
    }
}

impl<T> UViewPacket<T> where T: Num {
    pub fn new(x: T, display_mode: DisplayMode) -> UViewPacket<T> {
        UViewPacket { value: x, display_mode: display_mode }
    }
}

impl ToString for DisplayMode {
    fn to_string(&self) -> String {
        match self {
            &DisplayMode::Chart => 0,
            &DisplayMode::Font => 1
        }.to_string()
    }
}

impl<T> ToString for UViewPacket<T> where T: Num + ToString {
    fn to_string(&self) -> String {
        format!("v{}m{}", self.value.to_string(), self.display_mode.to_string())
    }
}

impl<T> fmt::Debug for UViewPacket<T> where T: Num + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value: {:5}\tMode: {:?}", self.value, self.display_mode)
    }
}

impl<T> Add<T> for UViewPacket<T> where T: Num {
    type Output = UViewPacket<T>;
    fn add(self, other: T) -> UViewPacket<T> {
        UViewPacket {
            value: self.value + other,
            display_mode: self.display_mode
        }
    }
}

impl<T> Add<UViewPacket<T>> for UViewPacket<T> where T: Num {
    type Output = UViewPacket<T>;
    fn add(self, other: UViewPacket<T>) -> UViewPacket<T> {
        UViewPacket {
            value: self.value + other.value,
            display_mode: self.display_mode
        }
    }
}

impl<T> AddAssign<T> for UViewPacket<T> where T: Num {
    fn add_assign(&mut self, other: T) {
        self.value += other;
    }
}

impl<T> AddAssign<UViewPacket<T>> for UViewPacket<T> where T: Num {
    fn add_assign(&mut self, other: UViewPacket<T>) {
        self.value += other.value;
    }
}
