use dyn_clone::DynClone;

use crate::core::hooks::stats::EditingStat;
use crate::core::mismatches::{Batch, MismatchesVec};

pub mod engine;
pub mod filters;
pub mod stats;

pub trait Hook<T: MismatchesVec>: DynClone + Send {
    fn on_finish(&mut self, _mismatches: &mut Batch<T>) {}
}
dyn_clone::clone_trait_object!(<T> Hook<T> where T: MismatchesVec);

// Stats must be called after filtering at each stage
pub trait HooksEngine<T: MismatchesVec>: Hook<T> {
    fn stats(self) -> Vec<Box<dyn EditingStat<T>>>;
}
