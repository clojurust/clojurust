# ClojuRust
A proof of concept version of Clojure in Rust.

## Goals
* Translate Clojure java code to Rust.
* Test a way to have an interpreter of Clojure code.
* Test a way to have a VM based on a FORTH system.
* Test a way to dynamically compile Clojure code in machine code.
* Test a way to create Rust code to compile Clojure code.

### Translate Clojure Java code to Rust
* There are already some partial implementations.
  * [ClojureRS](https://github.com/clojure-rs/ClojureRS).
  * [MAL (make a lisp) Rust version](https://github.com/kanaka/mal/tree/master/impls/rust).
  * [Ketos is a Lisp dialect compiled to bytecode and interpreted by pure Rust code](https://github.com/murarth/ketos).
* Some difficulties
  * We should use ARc to improve threading changes.
  * Implementation of Clojure protocols with class, traits, functions dispatch, and type coercion at the object trait level.
  * This should be implemented with macros for ease of programming, and code generation.
  * Implementation of RT calls.
  * Base Object class doesn't exist create a trait? generic struct?
  * For type coertion T -> S w/o v-table problem: 
    * [downcast-rs](https://crates.io/crates/downcast-rs)
    * [oso](https://www.osohq.com/post/rust-reflection-pt-1)
  * For Dynamic Casting for Trait.
    * [Dynamic Casting for Traits]/(http://idubrov.name/rust/2018/06/16/dynamic-casting-traits.html).
  * For all data structures, it's possible to use the excellent crate from _Bodil Stokke, Esq._
    * [Immutable Data Structures for Rust, Rc version](https://crates.io/crates/im-rc).
    * [Immutable Data Structures for Rust, Arc version](https://crates.io/crates/im).
### Test a way to have an interpreter of Clojure code
* Should be straitforward, but slow and clojure.core lib compilation is a problem.
* Can be a first test for dynamic compilation before a full implementation.
* Way to serialize?

### Test a way to have a VM based on a FORTH system
* Could be a solution, but slow.
* The JVM is a stack machine too. so... ;)
* Easy dynamic loading. but not for external libraries with Rust interface.
* Easy compilation.

### Test a way to dynamically compile Clojure code in machine code
* _Inline assemblers_, need Rust compilation  
  * [Rust asm! macro (inline llvm directives)](https://doc.rust-lang.org/beta/unstable-book/library-features/asm.html).
  * [Rust llvm_asm! macro](https://doc.rust-lang.org/unstable-book/library-features/llvm-asm.html).
  * [Rust global_asm! nacro](https://doc.rust-lang.org/unstable-book/library-features/global-asm.html).
* _Runtime assembler_, can generate code during execution.
    * [Registery assembler (only x86-64 long mode instructions, enable dynamic relocation)](https://crates.io/crates/assembler).
*  Some difficulties
    * Assemblers, need assembly code adaptation according to the harware.

### Test a way to create Rust code to compile Clojure code
* This would be the simplest solution.
* Compiler could carry on optimization.
* Compilation could be a first part test of correct code.
* Dynamic loading could be managed through FFI for both C and Rust.
  * Rust: 
    * [abi_stable_crates](https://github.com/rodrimati1992/abi_stable_crates).
    * [Rust FFI Guide](https://michael-f-bryan.github.io/rust-ffi-guide/overview.html).
    * [Plugins in Rust](https://github.com/Michael-F-Bryan/plugins_in_rust).
    * [Generates code for loading native libraries at runtime](https://github.com/Michael-F-Bryan/libloading-bindgen)
  * C: 
    * [libc](https://doc.rust-lang.org/nomicon/ffi.html).
* A loader, call, and callback management should be written.
* In case of compilation, static linking could be used.
* Managing raw types could be tricky.

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed withn this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
