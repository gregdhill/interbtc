use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

pub(crate) type BalanceOf<T> = <T as currency::Config>::Balance;

pub type UnsignedFixedPoint<T> = <T as currency::Config>::UnsignedFixedPoint;

/// Storage version.
#[derive(Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
pub enum Version {
    /// Initial version.
    V0,
}

pub trait OnAggregateChange<Key, Value> {
    fn on_aggregate_change(key: &Key, value: Value);
}

impl<Key, Value> OnAggregateChange<Key, Value> for () {
    fn on_aggregate_change(_: &Key, _: Value) {}
}
