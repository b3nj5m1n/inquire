use crate::ui::{Key, KeyModifiers};
use chrono::{Datelike, Duration, NaiveDate};

/// An event guard which decides whether or not an event has occured based on the pressed key and a predicate
pub struct Guard<T> {
    key: Key,
    predicate: Option<Box<dyn Fn(&T) -> bool>>,
}

impl<T> Guard<T> {
    /// Create a new [guard][Guard]
    pub fn new(key: Key, predicate: Option<Box<dyn Fn(&T) -> bool>>) -> Self {
        Self { key, predicate }
    }
    /// Test if the event has occured
    pub fn test(&self, key: Key, caller: &T) -> bool {
        if key != self.key {
            return false;
        }
        if let Some(f) = &self.predicate {
            return f(caller);
        }
        true
    }
}

/// An event which has a callback that can be trigged by one of multiple [guards](Guard)
struct Event<T> {
    guards: Vec<Guard<T>>,
    callback: Box<dyn Fn(&mut T) -> ()>,
}

impl<T> Event<T> {
    /// Create a new [event][Event]
    pub fn new(guards: Vec<Guard<T>>, callback: Box<dyn Fn(&mut T) -> ()>) -> Self {
        Self { guards, callback }
    }
}

/// Manages a collection of [events](Event)
pub struct KeyHandler<T> {
    events: Vec<Event<T>>,
}

impl<T> KeyHandler<T> {
    /// Create a new [key handler](KeyHandler)
    pub fn new() -> Self {
        Self { events: vec![] }
    }
    /// Add an event which only has a single guard
    pub fn add_event_single_guard(&mut self, guard: Guard<T>, callback: Box<dyn Fn(&mut T) -> ()>) {
        self.add_event_multiple_guards(vec![guard], callback);
    }
    /// Add an event which has multiple guards
    pub fn add_event_multiple_guards(
        &mut self,
        guards: Vec<Guard<T>>,
        callback: Box<dyn Fn(&mut T) -> ()>,
    ) {
        self.events.push(Event::new(guards, callback));
    }
    /// Process a key and call callbacks for all triggered events
    pub fn on_change(key: Key, caller: &mut T, handler: &Self) {
        for (Event { guards, callback }) in handler.events.iter() {
            for guard in guards {
                if guard.test(key, caller) {
                    callback(caller);
                }
            }
        }
    }
}
