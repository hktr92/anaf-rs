use std::collections::BTreeMap;

use serde::{ser::SerializeMap, Serialize};

use super::{BalanceIndicator, RawBalance};

#[derive(Debug, Clone)]
pub struct Balance {
    inner: BTreeMap<BalanceIndicator, RawBalance>,
}

impl From<BTreeMap<BalanceIndicator, RawBalance>> for Balance {
    fn from(inner: BTreeMap<BalanceIndicator, RawBalance>) -> Self {
        Self { inner }
    }
}

impl Balance {
    pub fn new(inner: BTreeMap<BalanceIndicator, RawBalance>) -> Self {
        Self { inner }
    }

    pub fn get(&self, indicator: BalanceIndicator) -> Option<&RawBalance> {
        self.inner.get(&indicator)
    }

    pub fn get_mut(&mut self, indicator: BalanceIndicator) -> Option<&mut RawBalance> {
        self.inner.get_mut(&indicator)
    }

    pub fn insert(&mut self, indicator: BalanceIndicator, balance: RawBalance) {
        self.inner.insert(indicator, balance);
    }

    pub fn remove(&mut self, indicator: BalanceIndicator) -> Option<RawBalance> {
        self.inner.remove(&indicator)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&BalanceIndicator, &RawBalance)> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&BalanceIndicator, &mut RawBalance)> {
        self.inner.iter_mut()
    }

    pub fn into_inner(self) -> BTreeMap<BalanceIndicator, RawBalance> {
        self.inner
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Serialize for Balance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(&k.to_string(), v)?;
        }
        map.end()
    }
}
