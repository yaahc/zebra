// Copied from tower/src/semaphore.rs, commit:
// d4d1c67 hedge: use auto-resizing histograms (#484)
//
// When we upgrade to tower 0.4, we can use tokio's PollSemaphore, like tower's:
// ccfaffc buffer, limit: use `tokio-util`'s `PollSemaphore` (#556)

// Ignore lints on this copied code
#![allow(dead_code)]

pub(crate) use self::sync::OwnedSemaphorePermit as Permit;
use futures_core::ready;
use std::{
    fmt,
    future::Future,
    mem,
    pin::Pin,
    sync::{Arc, Weak},
    task::{Context, Poll},
};
use tokio::sync;

#[derive(Debug)]
pub(crate) struct Semaphore {
    semaphore: Arc<sync::Semaphore>,
    state: State,
}

#[derive(Debug)]
pub(crate) struct Close {
    semaphore: Weak<sync::Semaphore>,
    permits: usize,
}

enum State {
    Waiting(Pin<Box<dyn Future<Output = Permit> + Send + Sync + 'static>>),
    Ready(Permit),
    Empty,
}

impl Semaphore {
    pub(crate) fn new_with_close(permits: usize) -> (Self, Close) {
        let semaphore = Arc::new(sync::Semaphore::new(permits));
        let close = Close {
            semaphore: Arc::downgrade(&semaphore),
            permits,
        };
        let semaphore = Self {
            semaphore,
            state: State::Empty,
        };
        (semaphore, close)
    }

    pub(crate) fn new(permits: usize) -> Self {
        Self {
            semaphore: Arc::new(sync::Semaphore::new(permits)),
            state: State::Empty,
        }
    }

    pub(crate) fn poll_acquire(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        loop {
            self.state = match self.state {
                State::Ready(_) => return Poll::Ready(()),
                State::Waiting(ref mut fut) => {
                    let permit = ready!(Pin::new(fut).poll(cx));
                    State::Ready(permit)
                }
                State::Empty => State::Waiting(Box::pin(self.semaphore.clone().acquire_owned())),
            };
        }
    }

    pub(crate) fn take_permit(&mut self) -> Option<Permit> {
        if let State::Ready(permit) = mem::replace(&mut self.state, State::Empty) {
            return Some(permit);
        }
        None
    }
}

impl Clone for Semaphore {
    fn clone(&self) -> Self {
        Self {
            semaphore: self.semaphore.clone(),
            state: State::Empty,
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Waiting(_) => f
                .debug_tuple("State::Waiting")
                .field(&format_args!("..."))
                .finish(),
            State::Ready(ref r) => f.debug_tuple("State::Ready").field(&r).finish(),
            State::Empty => f.debug_tuple("State::Empty").finish(),
        }
    }
}

impl Close {
    /// Close the semaphore, waking any remaining tasks currently awaiting a permit.
    pub(crate) fn close(self) {
        // The maximum number of permits that a `tokio::sync::Semaphore`
        // can hold is usize::MAX >> 3. If we attempt to add more than that
        // number of permits, the semaphore will panic.
        // XXX(eliza): another shift is kinda janky but if we add (usize::MAX
        // > 3 - initial permits) the semaphore impl panics (I think due to a
        // bug in tokio?).
        // TODO(eliza): Tokio should _really_ just expose `Semaphore::close`
        // publicly so we don't have to do this nonsense...
        const MAX: usize = std::usize::MAX >> 4;
        if let Some(semaphore) = self.semaphore.upgrade() {
            // If we added `MAX - available_permits`, any tasks that are
            // currently holding permits could drop them, overflowing the max.
            semaphore.add_permits(MAX - self.permits);
        }
    }
}
