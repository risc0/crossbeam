//! Tools for concurrent programming.
//!
//! ## Atomics
//!
//! * [`AtomicCell`], a thread-safe mutable memory location.
//! * [`AtomicConsume`], for reading from primitive atomic types with "consume" ordering.
//!
//! ## Data structures
//!
//! * [`deque`], work-stealing deques for building task schedulers.
//! * [`ArrayQueue`], a bounded MPMC queue that allocates a fixed-capacity buffer on construction.
//! * [`SegQueue`], an unbounded MPMC queue that allocates small buffers, segments, on demand.
//!
//! ## Memory management
//!
//! * [`epoch`], an epoch-based garbage collector.
//!
//! ## Thread synchronization
//!
//! * [`channel`], multi-producer multi-consumer channels for message passing.
//! * [`Parker`], a thread parking primitive.
//! * [`ShardedLock`], a sharded reader-writer lock with fast concurrent reads.
//! * [`WaitGroup`], for synchronizing the beginning or end of some computation.
//!
//! ## Utilities
//!
//! * [`Backoff`], for exponential backoff in spin loops.
//! * [`CachePadded`], for padding and aligning a value to the length of a cache line.
//! * [`scope`], for spawning threads that borrow local variables from the stack.
//!
//! [`AtomicCell`]: atomic::AtomicCell
//! [`AtomicConsume`]: atomic::AtomicConsume
//! [`ArrayQueue`]: queue::ArrayQueue
//! [`SegQueue`]: queue::SegQueue
//! [`Parker`]: sync::Parker
//! [`ShardedLock`]: sync::ShardedLock
//! [`WaitGroup`]: sync::WaitGroup
//! [`Backoff`]: utils::Backoff
//! [`CachePadded`]: utils::CachePadded

#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms),
        allow(dead_code, unused_assignments, unused_variables)
    )
))]
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(cfg_target_has_atomic))]

#[cfg_attr(feature = "nightly", cfg(target_has_atomic = "ptr"))]
pub use crossbeam_utils::atomic;

pub mod utils {
    //! Miscellaneous utilities.
    //!
    //! * [`Backoff`], for exponential backoff in spin loops.
    //! * [`CachePadded`], for padding and aligning a value to the length of a cache line.

    pub use crossbeam_utils::Backoff;
    pub use crossbeam_utils::CachePadded;
}

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "alloc")] {
        #[doc(inline)]
        pub use crossbeam_epoch as epoch;

        #[doc(inline)]
        pub use crossbeam_queue as queue;
    }
}

cfg_if! {
    if #[cfg(feature = "std")] {
        #[doc(inline)]
        pub use crossbeam_deque as deque;

        #[doc(inline)]
        pub use crossbeam_channel as channel;
        pub use crossbeam_channel::select;

        pub use crossbeam_utils::sync;

        #[cfg(not(loom_crossbeam))]
        pub use crossbeam_utils::thread;
        #[cfg(not(loom_crossbeam))]
        pub use crossbeam_utils::thread::scope;
    }
}
