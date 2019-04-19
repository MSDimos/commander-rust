use std::iter::FromIterator;
use crate::{ Instance, ArgumentType, Argument };
use crate::errors::{ error, ARG_DONT_MATCH };
use std::ops::{ Deref, DerefMut };

/// Encapsulation of `Instance`.
///
/// Being used for type converting.
/// Any type that implements `From<Raw>` can be used as a parameter type for command processing methods.
/// By default, we implement several types of inheritance.
#[derive(Debug, Clone)]
pub struct Raw(Vec<String>);

impl Raw {
    #[doc(hidden)]
    #[inline]
    pub fn push(&mut self, ele: String) {
        (self.0).push(ele);
    }

    #[doc(hidden)]
    #[inline]
    pub fn remove(&mut self, idx: usize) -> String {
        (self.0).remove(idx)
    }

    #[doc(hidden)]
    #[inline]
    pub fn new(v: Vec<String>) -> Raw {
        Raw(v)
    }

    #[doc(hidden)]
    #[inline]
    pub fn is_empty(&self) -> bool {
        (self.0).len() > 0
    }

    #[doc(hidden)]
    pub fn divide_cmd(ins: &Instance, args: &Vec<Argument>) -> Vec<Raw> {
        let mut raws = vec![];
        let len = ins.args.len();
        let mut iter = ins.args.iter();

        for arg in args {
            let ty = &arg.ty;

            match ty {
                ArgumentType::RequiredSingle => {
                    let mut raw = Raw::new(vec![]);

//                    if len == 0 {
//                        error(ARG_DONT_MATCH);
//                    }
                    if let Some(raw_str) = iter.next() {
                        raw.push(raw_str.clone());
                    }
                    raws.push(raw);
                },
                ArgumentType::OptionalSingle => {
                    let mut raw = Raw::new(vec![]);

                    if let Some(raw_str) = iter.next() {
                        raw.push(raw_str.clone());
                    }
                    raws.push(raw);
                },
                ArgumentType::RequiredMultiple => {
                    let mut raw = Raw::new(vec![]);

//                    if len == 0 {
//                        error(ARG_DONT_MATCH);
//                    }
                    while let Some(raw_str) = iter.next() {
                        raw.push(raw_str.clone());
                    }
                    raws.push(raw);
                },
                ArgumentType::OptionalMultiple => {
                    let mut raw = Raw::new(vec![]);

                    while let Some(raw_str) = iter.next() {
                        raw.push(raw_str.clone());
                    }
                    raws.push(raw);
                }
            }
        }

        raws
    }

    #[doc(hidden)]
    pub fn divide_opt(ins: &Instance, arg: &Option<Argument>) -> Raw {
        if let Some(arg) = arg {
            let len = ins.args.len();
            let mut iter = ins.args.iter();

            match arg.ty {
                ArgumentType::RequiredSingle => {
                    let mut raw = Raw::new(vec![]);

//                    if len == 0 {
//                        error(ARG_DONT_MATCH);
//                    }
                    if let Some(raw_str) = iter.next() {
                        raw.push(raw_str.clone());
                    }

                    raw
                },
                ArgumentType::OptionalSingle => {
                    let mut raw = Raw::new(vec![]);

                    if let Some(raw_str) = iter.next() {
                        raw.push(raw_str.clone());
                    }

                    raw
                },
                ArgumentType::RequiredMultiple => {
                    let mut raw = Raw::new(vec![]);

//                    if len == 0 {
//                        error(ARG_DONT_MATCH);
//                    }

                    while let Some(raw_str) = iter.next() {
                        raw.push(raw_str.clone());
                    }

                    raw
                },
                ArgumentType::OptionalMultiple => {
                    let mut raw = Raw::new(vec![]);

                    while let Some(raw_str) = iter.next() {
                        raw.push(raw_str.clone());
                    }

                    raw
                }
            }
        } else {
            Raw::new(vec![])
        }
    }
}

impl FromIterator<String> for Raw {
    fn from_iter<I: IntoIterator<Item=String>>(iter: I) -> Raw {
        let mut v = vec![];

        for item in iter.into_iter() {
            v.push(item);
        }

        Raw::new(v)
    }
}

impl Deref for Raw {
    type Target = Vec<String>;

    fn deref(&self) -> &Vec<String> {
        &(self.0)
    }
}

impl DerefMut for Raw {
    fn deref_mut(&mut self) -> &mut Vec<String> {
        &mut (self.0)
    }
}

macro_rules! impl_primitive {
    ($($ty: ty),*) => {
        $(
            impl From<Raw> for $ty {
                fn from(raw: Raw) -> $ty {
                    raw.get(0)
                        .unwrap_or(&String::from("0"))
                        .parse()
                        .unwrap_or(<$ty>::default())
                }
            }
        )*
    };
}

macro_rules! impl_option {
    ($($ty: ty),*) => {
        $(
            impl From<Raw> for Option<$ty> {
                fn from(raw: Raw) -> Option<$ty> {
                    if raw.is_empty() {
                        None
                    } else {
                        Some(<$ty>::from(raw))
                    }
                }
            }
        )*
    };
}

macro_rules! impl_vec {
    ($($ty: ty),*) => {
        $(
            impl From<Raw> for Vec<$ty> {
                fn from(raw: Raw) -> Vec<$ty> {
                    raw.iter().map(|i| i.parse().unwrap_or(<$ty>::default())).collect()
                }
            }
        )*
    };
}

macro_rules! impl_option_vec {
    ($($ty: ty),*) => {
        $(
            impl From<Raw> for Option<Vec<$ty>> {
                fn from(raw: Raw) -> Option<Vec<$ty>> {
                    if raw.is_empty() {
                        None
                    } else {
                        Some(<Vec<$ty>>::from(raw))
                    }
                }
            }
        )*
    };
}

macro_rules! impl_all {
    ($($ty: ty),*) => {
        impl_primitive![$($ty),*];
        impl_option![$($ty),*];
        impl_vec![$($ty),*];
        impl_option_vec![$($ty),*];
    };
}

impl_all![i8, i16, i32, i64, i128, isize];
impl_all![u8, u16, u32, u64, u128, usize];
impl_all![f32, f64, bool];


impl From<Raw> for String {
    fn from(raw: Raw) -> String {
        raw.get(0).unwrap_or(&String::new()).clone()
    }
}

impl From<Raw> for Vec<String> {
    fn from(raw: Raw) -> Vec<String> {
        raw.iter().map(|s| s.clone()).collect()
    }
}

impl From<Raw> for Option<String> {
    fn from(raw: Raw) -> Option<String> {
        if raw.is_empty() {
            None
        } else {
            Some(String::from(raw))
        }
    }
}

impl From<Raw> for Option<Vec<String>> {
    fn from(raw: Raw) -> Option<Vec<String>> {
        if raw.is_empty() {
            None
        } else {
            Some(<Vec<String>>::from(raw))
        }
    }
}