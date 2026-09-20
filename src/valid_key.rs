use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub trait ValidKey:
    Clone + Eq + Hash + Sync + Send + Serialize + for<'a> Deserialize<'a> + 'static
{
}

impl<T> ValidKey for T where
    T: Clone + Eq + Hash + Sync + Send + Serialize + for<'a> Deserialize<'a> + 'static
{
}
