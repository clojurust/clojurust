# ClojuRust
A proof of concept version of Clojure in Rust.

<!-- TOC -->

- [ClojuRust](#clojurust)
    - [Current state](#current-state)
    - [Goals](#goals)
    - [Documentation](#documentation)
    - [Code projects](#code-projects)
    - [Licence](#licence)
        - [Persistent data structures licence](#persistent-data-structures-licence)
        - [Original Clojure licence](#original-clojure-licence)
        - [clojure.core original code](#clojurecore-original-code)

<!-- /TOC -->

## Current state
WIP in an analysis and test state, and so a working version is out of sight for now.

As I'm a newbie in Rust, I hit all the bads of borrowing, referencing, Arcs, automatic derefs, timeline problems... But it begin to enter in my fingers... :wink:

For now, in the creation of the clojure::rust modules, say the Rust host environment, as Clojure is an hosted language. Rust, as a bare-metal language, has no dynamic abilities per se, and is even opposed to such an approach as all is verified at compile time. So a lot of information should be statically stored for the library to function.

The idea, is not to have an optimized structure as first draft, but a working concepts framework, easy to refactor as experiments are tested.

For example the current model for Clojure objects is a boxed and synchronized structure. Returned values are boxed in a Result struct incorporating error values. So it's clearly the less efficient and the more ressource consumig way to do it, so optimization can only go in a more fast result... :grind:

Meanwhile, the first Core Clojure classes can be developed in parallel, for testing purposes, but a strong separation shall be preserved.

## Goals
* Define an internal object model.
* Translate Clojure java code to Rust.
* Test a way to have an interpreter of Clojure code.
* Test a way to have a VM based on a stack machine.
* Test a way to dynamically compile Clojure code in machine code.
* Test a way to create Rust code to compile Clojure code.

## Documentation
* [Clojurust crate](https://clojurust.github.io/clojurust.doc/clojurust/) Rust version of Clojure Java Core
* [Im crate](https://clojurust.github.io/clojurust.doc/im/) Immutable data structures for Rust by Bodil

## Code projects
* [ClojuRust Core](https://github.com/clojurust/clojurust) ClojuRust library
* [ClojuRust macros](https://github.com/clojurust/clojurust-macros) Macros to standardize ClojuRust


## Licence
This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. 

If a copy of the MPL was not distributed within this file, You can obtain one at http://mozilla.org/MPL/2.0/.


    Copyright (c) 2020 Ivan Pierre, kilroySoft, <Ivan Pierre, ivan@kilroysoft.ch>, under MPL 2.0.

### Persistent data structures licence

    Copyright 2017 Bodil Stokke, under MPL 2.0.

* Crate doc is here: https://crates.io/crates/im
* Code is on GitHub: https://github.com/bodil/im-rs

### Original Clojure licence
As clojure::lang::* is a translation of original code of Java Clojure core, reference is due to Rich Hickey:

    Copyright (c) Rich Hickey. All rights reserved. Under EPL 1.0.

* Original project can be found in the main repository: https://github.com/clojure/clojure

But the project is not in this stage for now.

As clojure::lang is on Eclipse Public License 1.0 (http://opensource.org/licenses/eclipse-1.0.php), there could be some problem of double licence problem.

As is, I put this advice of the original project licence:

    Clojure
    Copyright (c) Rich Hickey. All rights reserved.
    The use and distribution terms for this software are covered by the Eclipse Public License 1.0 (http://opensource.org/licenses/eclipse-1.0.php) which can be found in the file epl-v10.html at the root of this distribution.
    By using this software in any fashion, you are agreeing to be bound by the terms of this license.
    You must not remove this notice, or any other, from this software.

The goal to use this type of licence by Rich is to keep control of evolution of the Clojure project core, and garantee not have an overwhelming core with elements that could be added w/o problem as external libraries.

This project goal is not to make a fork of Clojure core, but to make a Rust version as close as possible to the original project. Code creation is quite different of the original. Differences are due to the limitations or possibilities caused by implementation context of the Rust ecosystem, else this would be a fork or more probably a completely different project made from scratch.

### clojure.core original code
The goal is to compile the originaly written `clojure.core` code, so normally the original code will remain in the original repository, and so final executable should envolve in parallel with the main project.

This will perhaps no be feasible, but only implementation problems should be cause of particular rewrites, and enhancement of the Rust core should be prioritized. For example the compilation in JVM bytecode will be of no usage, but the AST analysis will be valid.
