use std::fmt;

/// Wraps different primitive variants used as values in html
/// This is needed since html attributes can have different value types
/// such as checked(bool), name(String), tab_index(i32)
/// Note: memory size of Value is 32 bytes, in comparison String is 24 bytes
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    /// bool value
    Bool(bool),
    /// &'static str value
    Str(&'static str),
    /// String value
    String(String),
    /// a vec of values
    Vec(Vec<Value>),
    /// u8 value
    U8(u8),
    /// u16 value
    U16(u16),
    /// u32 value
    U32(u32),
    /// u64 value
    U64(u64),
    /// usize value
    Usize(usize),
    /// u128 value
    U128(u128),
    /// i8 value
    I8(i8),
    /// i16 value
    I16(i16),
    /// i32 value
    I32(i32),
    /// i64 value
    I64(i64),
    /// i128 value
    I128(i128),
    /// isize value
    Isize(isize),
    /// f32 value
    F32(f32),
    /// f64 value
    F64(f64),
}

impl Value {
    /// returns an &str reference if this value is `Str` or `String` variant
    /// Note: This doesn't convert other variant into str representation
    /// Use the `to_string()` for that.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(ref v) => Some(v),
            Value::Str(v) => Some(v),
            _ => None,
        }
    }

    /// returns the bool value if this a Bool variant
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// converts to f64 if the variants are numerical representation
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Bool(_) => None,
            Value::String(_v) => None,
            Value::Str(_v) => None,
            Value::Vec(_v) => None,
            Value::U8(v) => Some(f64::from(*v)),
            Value::U16(v) => Some(f64::from(*v)),
            Value::U32(v) => Some(f64::from(*v)),
            Value::U64(v) => Some(*v as f64),
            Value::U128(v) => Some(*v as f64),
            Value::Usize(v) => Some(*v as f64),
            Value::I8(v) => Some(f64::from(*v)),
            Value::I16(v) => Some(f64::from(*v)),
            Value::I32(v) => Some(f64::from(*v)),
            Value::I64(v) => Some(*v as f64),
            Value::I128(v) => Some(*v as f64),
            Value::Isize(v) => Some(*v as f64),
            Value::F32(v) => Some(f64::from(*v)),
            Value::F64(v) => Some(*v),
        }
    }

    /// converts to i32 if the variants are numerical representation
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Value::Bool(_) => None,
            Value::String(_v) => None,
            Value::Str(_v) => None,
            Value::Vec(_v) => None,
            Value::U8(v) => Some(i32::from(*v)),
            Value::U16(v) => Some(i32::from(*v)),
            Value::U32(v) => Some(*v as i32),
            Value::U64(v) => Some(*v as i32),
            Value::U128(v) => Some(*v as i32),
            Value::Usize(v) => Some(*v as i32),
            Value::I8(v) => Some(i32::from(*v)),
            Value::I16(v) => Some(i32::from(*v)),
            Value::I32(v) => Some(*v),
            Value::I64(v) => Some(*v as i32),
            Value::I128(v) => Some(*v as i32),
            Value::Isize(v) => Some(*v as i32),
            Value::F32(v) => Some(*v as i32),
            Value::F64(v) => Some(*v as i32),
        }
    }

    /// If this is Value::Vec variant, append the new value
    /// otherwise, turn this value into Value::Vec(Vec<Value>) variant
    /// and append the new value.
    pub fn append(&mut self, new_value: Value) {
        match self {
            Value::Vec(values) => {
                values.push(new_value);
            }
            _ => {
                *self = Value::Vec(vec![self.clone(), new_value]);
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bool(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
            Value::Str(v) => write!(f, "{}", v),
            Value::Vec(v) => {
                write!(
                    f,
                    "{}",
                    v.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Value::U8(v) => write!(f, "{}", v),
            Value::U16(v) => write!(f, "{}", v),
            Value::U32(v) => write!(f, "{}", v),
            Value::U64(v) => write!(f, "{}", v),
            Value::U128(v) => write!(f, "{}", v),
            Value::Usize(v) => write!(f, "{}", v),
            Value::I8(v) => write!(f, "{}", v),
            Value::I16(v) => write!(f, "{}", v),
            Value::I32(v) => write!(f, "{}", v),
            Value::I64(v) => write!(f, "{}", v),
            Value::I128(v) => write!(f, "{}", v),
            Value::Isize(v) => write!(f, "{}", v),
            Value::F32(v) => write!(f, "{}", v),
            Value::F64(v) => write!(f, "{}", v),
        }
    }
}

impl From<&String> for Value {
    fn from(v: &String) -> Self {
        Value::String(v.to_string())
    }
}

impl<T, const N: usize> From<[T; N]> for Value
where
    T: Into<Value> + Clone,
{
    fn from(v: [T; N]) -> Self {
        Value::Vec(
            v.iter()
                .map(|i| i.to_owned().into())
                .collect::<Vec<Value>>(),
        )
    }
}

macro_rules! impl_from {
    ($ty:ty => $variant:ident) => {
        impl From<$ty> for Value {
            fn from(f: $ty) -> Self {
                Value::$variant(f)
            }
        }
    };
}

impl_from!(bool => Bool);
impl_from!(String => String);
impl_from!(&'static str => Str);
impl_from!(u8 => U8);
impl_from!(u16 => U16);
impl_from!(u32 => U32);
impl_from!(u64 => U64);
impl_from!(u128 => U128);
impl_from!(usize => Usize);
impl_from!(i8 => I8);
impl_from!(i16 => I16);
impl_from!(i32 => I32);
impl_from!(i64 => I64);
impl_from!(i128 => I128);
impl_from!(isize => Isize);
impl_from!(f32 => F32);
impl_from!(f64 => F64);

macro_rules! impl_from_tuple {
    (($($T:ident),*) => $($n:tt),*) => {
        impl<$($T),*>From<($($T),*)> for Value
            where
                $($T: Into<Value>),*
        {
            fn from(v: ($($T),*)) -> Self {
                Value::Vec(vec![$(v.$n.into()),*])
            }
        }
    };
}

impl_from_tuple!((T, U) => 0, 1);
impl_from_tuple!((T, U, V) => 0, 1, 2);
impl_from_tuple!((T, U, V, X) => 0, 1, 2, 3);
impl_from_tuple!((T, U, V, X, Z) => 0, 1, 2, 3, 4);
