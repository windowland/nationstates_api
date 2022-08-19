use crate::request::RequestBuilder;

/// Struct to combine multiple queries into one.
/// It is recommended to use the [`and`] macro to combine large amounts of queries.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct And<T, U>(pub T, pub U);

impl<T: Query, U: Query> Query for And<T, U> {
    fn add_query<R: RequestBuilder>(&self, builder: R) -> R {
        self.0.add_query(self.1.add_query(builder))
    }
}

/// Represents an empty query string. If used by itself, it returns a set
/// of default values.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct None;

///trait representing a shard to be queried.
pub trait Query {
    fn add_query<R: RequestBuilder>(&self, builder: R) -> R;
}

#[macro_export]
macro_rules! and {
  ($head:ty, $($tail:ty),+ $(,)?) => {
    $crate::query::And<$head, and!($($tail),+)>
  };
  ($final:ty) =>{
    $final
  }
}
