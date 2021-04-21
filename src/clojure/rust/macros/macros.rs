//! # Macros to manage static `Clojure`'s data wrtiting.
//!
//! But other usefull macros will be inserted here to have macro
//! templates to generate objects. Goal is to be able to use then
//! to simplify wraping of Rust objects, and also be helpers to
//! generate Rust code in the compilation of `Clojure` code to Rust.

/// Read modules to use for `Object`s
#[macro_export]
macro_rules! use_init_obj(
    () => {};

    ($path:path ; $($tail:tt)*) => {
        use $path::*;
        use_init_obj!($($tail)*);
    };
);

/// Read initialisation block
///
/// This should be a block to avoid complicated syntax parsing
#[macro_export]
macro_rules! init_init_obj(
    () => {};

    ($t:block $($tail:tt)*) => {
        $t
        init_init_obj!($($tail)*);
    };
);

/// Initialisation block of `Object`s
///
/// Here is the whole runtime initialisation of `Object`s, `Classe`s,
/// `Protocol`s, `Functions`... the whole Runtime DB.
#[macro_export]
macro_rules! init_obj {
    ($name:ident $($tail:tt)*) => {
        /// Initialisation block of `Object`s
        ///
        /// Here is the whole runtime initialisation of `Object`s, `Classe`s,
        /// `Protocol`s, `Functions`... the whole Runtime DB.
        ///
        /// As we are going to mutate global state of the library
        /// at runtime this function is `unsafe`. But it's executed only once
        /// and doesnt need to be thread-safe as it run only on one thread ///// (for now).
        pub unsafe fn init() {
            // only execute one time
            if INIT {
                return;
            }

            INIT = true;

            println!("$name::init");
            init_init_obj!($($tail)*);
        }


        static mut INIT: bool = false;
    }
}

/// Use block for `Object`s
#[macro_export]
macro_rules! use_obj {
    ($($tail:tt)*) => {
        // use crate::*;
        // use crate::use_init_obj;
        // use crate::init_obj;

        use_init_obj!($($tail)*);
    }
}

#[macro_export]
macro_rules! new_obj (
    (*$exp:expr) => { Object::new(Some(Arc::new(*$exp))) };
    ($exp:expr) => { Object::new(Some(Arc::new($exp))) };
);
