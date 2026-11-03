//! Facts as data: one component holding every fact a [`Planner`](crate::prelude::Planner)
//! reasons over.
//!
//! The alternative this sits beside is a component per fact
//! ([`DatumComponent`](crate::prelude::DatumComponent)), which the ECS can query directly and
//! which suits a world whose vocabulary is fixed and known at compile time.
//!
//! A world whose vocabulary grows with its content wants the other shape. `LocalState` is a map
//! of `String -> Datum` already, and a planner's preconditions and mutators name their facts by
//! string, so the types were never load-bearing: adding `iron` to a world meant adding a type, an
//! arm in every match over the facts, a registration and an entry in the planner's datum list,
//! none of which the planner itself needed. With [`Facts`] the vocabulary is whatever the world's
//! content puts in the map, and a new verb needs no code at all.
//!
//! It is also the shape a fact database wants for anything other than planning — a bark rule, a
//! dialogue condition, a query over "what do we know about this character" — which is why it is
//! one component rather than many: the whole dictionary travels together.

use crate::prelude::*;
use dogoap::prelude::{Datum, InternalData};

/// Every fact an entity's planner reasons over, by name.
///
/// Values are [`Datum`], so a fact is a bool, a float, a counter or an enum index; a planner's
/// preconditions compare against them and its mutators simulate changes to them. Nothing in here
/// is written by the planner: an action's effects are simulated while planning, and the world
/// writes the fact when the work is actually done.
#[derive(Component, Default, Clone, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct Facts(pub InternalData);

impl Facts {
    /// No facts at all: a world's content fills them in.
    pub fn new() -> Self {
        Self(InternalData::new())
    }

    /// Builder form, for seeding a character's starting facts.
    #[must_use]
    pub fn with(mut self, key: impl Into<String>, value: impl Into<Datum>) -> Self {
        self.0.insert(key.into(), value.into());
        self
    }

    /// What a fact holds, or `None` where nothing has set it.
    pub fn get(&self, key: &str) -> Option<Datum> {
        self.0.get(key).copied()
    }

    /// Write a fact, whatever it held before.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<Datum>) {
        self.0.insert(key.into(), value.into());
    }

    /// A counter fact, zero when the world has never set it.
    pub fn count(&self, key: &str) -> i64 {
        match self.0.get(key).copied() {
            Some(Datum::I64(held)) => held,
            _ => 0,
        }
    }

    /// Add to a counter fact, never below `floor`. Returns what it now holds.
    pub fn add(&mut self, key: &str, delta: i64, floor: i64) -> i64 {
        let now = (self.count(key) + delta).max(floor);
        self.0.insert(key.to_owned(), Datum::I64(now));
        now
    }

    /// Every fact's name, in order.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_counter_starts_at_zero_and_has_a_floor() {
        let mut facts = Facts::new();
        assert_eq!(facts.count("iron"), 0);
        assert_eq!(facts.add("iron", 2, 0), 2);
        assert_eq!(facts.add("iron", -5, 0), 0);
        assert_eq!(facts.count("iron"), 0);
    }

    #[test]
    fn facts_of_other_kinds_read_as_themselves() {
        let facts = Facts::new().with("awake", true).with("warmth", 0.5);
        assert_eq!(facts.get("awake"), Some(Datum::Bool(true)));
        assert_eq!(facts.get("warmth"), Some(Datum::F64(0.5)));
        // A counter read of something that is not a counter is zero, not a panic.
        assert_eq!(facts.count("awake"), 0);
    }
}
