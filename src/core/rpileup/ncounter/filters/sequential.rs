use std::marker::PhantomData;

use super::{AlignedRead, ReadsFilter};

// TODO: it should have a static variant based on a macro of some kind
// You create it for 1-12 arguments in a compile time AND a from declaration for them using tuples
// macro_rules! sequential {
//     ($($x:ident),*) => {
//         struct SequentialReadsFilter<$($x: ReadsFilter,)*> {
//             $($x: $x,)*
//         }
//
//         impl<R, $($x: ReadsFilter,)*> ReadsFilter<R> for SequentialReadsFilter<R, $($x,)*> {
//             #[inline]
//             fn is_read_ok(&self, record: &R) -> bool {
//                 true $(&& self.$x.is_read_ok(record))*
//                 // self.first.is_read_ok(record) & self.second.is_read_ok(record)
//             }
//
//             #[inline]
//             fn is_base_ok(&self, record: &R, base: usize) -> bool {
//                 true $(&& self.$x.is_base_ok(record, base))*
//             }
//         }
//     };
// }
//
// sequential!(a, b, c);

#[derive(Copy, Clone)]
pub struct Sequential<R: AlignedRead, First: ReadsFilter<R>, Second: ReadsFilter<R>> {
    first: First,
    second: Second,
    phantom: PhantomData<fn() -> R>,
}

impl<R: AlignedRead, First: ReadsFilter<R>, Second: ReadsFilter<R>> Sequential<R, First, Second> {
    pub fn new(first: First, second: Second) -> Self {
        Sequential { first, second, phantom: Default::default() }
    }
}

impl<R, First, Second> ReadsFilter<R> for Sequential<R, First, Second>
where
    R: AlignedRead,
    First: ReadsFilter<R>,
    Second: ReadsFilter<R>,
{
    #[inline]
    fn is_read_ok(&self, record: &R) -> bool {
        self.first.is_read_ok(record) & self.second.is_read_ok(record)
    }

    #[inline]
    fn is_base_ok(&self, record: &R, base: usize) -> bool {
        self.first.is_base_ok(record, base) & self.second.is_base_ok(record, base)
    }
}
