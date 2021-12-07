#![ warn( missing_docs ) ]

//!
//! Tools for writing and runnint tests.
//!
//! # Sample
//! ``` rust
//! use wtest_basic::*;
//!
//! //
//!
//! fn _pass1()
//! {
//!   assert_eq!( true, true );
//! }
//!
//! //
//!
//! fn _pass2()
//! {
//!   assert_eq!( 1, 1 );
//! }
//!
//! //
//!
//! test_suite!
//! {
//!   pass1,
//!   pass2,
//! }
//!
//! ```

pub use wtest_basic as basic;
pub use basic::*;