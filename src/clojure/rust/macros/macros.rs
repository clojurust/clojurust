//! # Macros to manage static `Clojure`'s data wrtiting.
//!
//! But other usefull macros will be inserted here to have macro
//! templates to generate objects. Goal is to be able to use then
//! to simplify wraping of Rust objects, and also be helpers to
//! generate Rust code in the compilation of `Clojure` code to Rust.

#[macro_export]
macro_rules! new_obj (
    (*$exp:expr) => { Object::new(Some(Arc::new(*$exp))) };
    ($exp:expr) => { Object::new(Some(Arc::new($exp))) };
);
