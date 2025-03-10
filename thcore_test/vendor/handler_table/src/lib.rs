#![no_std]
#![doc = include_str!("../README.md")]

use core::sync::atomic::{AtomicUsize, Ordering};

/// The type of an event handler.
///
/// Currently no arguments and return values are supported.
pub type Handler = fn();

/// A lock-free table of event handlers.
///
/// It internally uses an array of `AtomicUsize` to store the handlers.
pub struct HandlerTable<const N: usize> {
    handlers: [AtomicUsize; N],
}

impl<const N: usize> HandlerTable<N> {
    /// Creates a new handler table with all entries empty.
    pub const fn new() -> Self {
        Self {
            handlers: [const { AtomicUsize::new(0) }; N],
        }
    }

    /// Registers a handler for the given index.
    ///
    /// Returns `true` if the registration succeeds, `false` if the index is out
    /// of bounds or the handler is already registered.
    pub fn register_handler(&self, idx: usize, handler: Handler) -> bool {
        if idx >= N {
            return false;
        }
        self.handlers[idx]
            .compare_exchange(0, handler as usize, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    /// Unregisters the handler for the given index.
    ///
    /// Returns the existing handler if it is registered, `None` otherwise.
    pub fn unregister_handler(&self, idx: usize) -> Option<Handler> {
        if idx >= N {
            return None;
        }
        let handler = self.handlers[idx].swap(0, Ordering::Acquire);
        if handler != 0 {
            Some(unsafe { core::mem::transmute::<usize, fn()>(handler) })
        } else {
            None
        }
    }

    /// Handles the event with the given index.
    ///
    /// Returns `true` if the event is handled, `false` if no handler is
    /// registered for the given index.
    pub fn handle(&self, idx: usize) -> bool {
        if idx >= N {
            return false;
        }
        let handler = self.handlers[idx].load(Ordering::Acquire);
        if handler != 0 {
            let handler: Handler = unsafe { core::mem::transmute(handler) };
            handler();
            true
        } else {
            false
        }
    }
}

impl<const N: usize> Default for HandlerTable<N> {
    fn default() -> Self {
        Self::new()
    }
}
