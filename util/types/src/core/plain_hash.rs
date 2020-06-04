use plain_hasher::PlainHasher;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use crate::{
    packed,

};

pub type DefaultPlainHasher = BuildHasherDefault<PlainHasher>;
pub type PlainHashMap<T> = HashMap<packed::Byte32, T, DefaultPlainHasher>;
