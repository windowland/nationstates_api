use crate::request::RequestBuilder;

/// Struct to combine multiple queries into one.
/// It is recommended to use the [`and`] macro to combine large amounts of queries.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct And<T, U>(T, U);

impl<T: Query, U: Query> Query for And<T, U> {
    fn add_query<R: RequestBuilder>(builder: R) -> R {
        U::add_query(T::add_query(builder))
    }
}

///trait representing a shard to be queried.
pub trait Query {
    fn add_query<R: RequestBuilder>(builder: R) -> R;
}

#[macro_export]
macro_rules! and {
  ($head:ty, $(tail:ty),+ $(,)?) => {
    $crate::query::And<$head, and!($($tail),+)>
  };
  ($final:ty) =>{
    $final
  }
}
