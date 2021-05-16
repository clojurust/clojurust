# ClojuRust
A proof of concept version of Clojure in Rust.

<!-- TOC -->

- [ClojuRust](#clojurust)
    - [Current state](#current-state)
    - [Processes](#processes)
        - [Getting original code structure](#getting-original-code-structure)
        - [Create or update the code with the data](#create-or-update-the-code-with-the-data)
        - [Macros to add Rust missing code](#macros-to-add-rust-missing-code)
        - [Implement Rust code](#implement-rust-code)
    - [Goals](#goals)
    - [Documentation](#documentation)
    - [Code projects](#code-projects)
    - [Licence](#licence)
        - [Persistent data structures licence](#persistent-data-structures-licence)
        - [Original Clojure licence](#original-clojure-licence)
        - [clojure.core original code](#clojurecore-original-code)

<!-- /TOC -->

## Current state
WIP in an analysis and test state, and so a working version is out of sight for now. Days have only 48 hours, be patient.

As I'm a newbie in Rust, I hit all the bad of borrowing, referencing, Arcs, automatic deref, time-line problems... But it begin to enter in my fingers... :wink:

For now, in the creation of the clojure::rust modules, say the Rust host environment, as Clojure is an hosted language. Rust, as a bare-metal language, has no dynamic abilities per se, and is even opposed to such an approach as all is verified at compile time. So a lot of information should be statically stored for the library to function.

To create the skeleton of the Clojure core library, the info shall be imported from original Clojure jar infos automatically, so syncing will be done with ease.

The idea, is not to have an optimized structure as first draft, but a working concepts framework, easy to refactor as experiments are tested.

For example the current model for Clojure objects is a boxed and synchronized structure. Returned values are boxed in a Result enum incorporating error values. So it's clearly the less efficient and the more ressource consuming way to do it, so optimization can only go in a more fast result... :grind:

Meanwhile, the first Core Clojure classes can be developed in parallel, for testing purposes, but a strong separation shall be preserved.

## Processes
The idea is to mainly automatize the process in order to grab all data and to create standard implementation w/o errors.

### Getting original code structure
With the intend to grab all the skeleton of the original Java library, ease creation of Rust code skeleton, as Rust doesn't play with reflection well, this will get the Objects, Abstract, Interfaces, defined in the core of the library, and get the same info for the Java standard library to be implemented in Rust.

This will be written in Clojure as it has access to reflection and, well... it's a better language for this sort of work.

### Create or update the code with the data
This will create or modify module and files hierarchy according to original data. Each Interface, and Abstract will create a new .rs file with a trait, each Object will create a TObject struct. The process will create code implementation place in the trait and struct definition with macros to generate all necessary code to match Java behavior. A new version of original code can modify the generated code, but not regenerate it. I shall only modify, if deletion of element is done, there should be done by commenting out the intended place.

In order to differentiate Generated code and Rust code, the names will be camel-cased... even is this is bad Rust writing. 

### Macros to add Rust missing code
We can access functions in Rust from trait object, but not automagically in the implemented structs. The same way derived trait doesn't permit to use function from their parents. Macros will add code to the files from simple struct and trait definitions, with code to implement them with struct functions call. Placeholders are already generated, so there should be only one place to add code to. This will also create generation of Functions, Classes, Interfaces, Protocols data to permit ClojuRust runtime to work.

Some existing crates can already make a part of the work, so these should be used.

### Implement Rust code
This is the long and painful work that will be proceeded manually... pfff...

## Goals
* [X] Define an internal object model. (clojurust::clojure::rust)
* [ ] Grab code structure from original JVM Clojure. (java-analysis) 
* [ ] Generate skeleton from Clojure core library programmatically.
* [ ] Manage macros to generate code to complete functions of the library. (clojurust-macro)
* [ ] Implement Clojure Java code into Rust implementation. (clojurust)
* [ ] Test a way to have an interpreter of Clojure code (eval).
* [ ] Test a way to have a VM based on a stack machine.
* [ ] Test a way to dynamically compile Clojure code in machine code.
* [ ] Test a way to create Rust code to compile Clojure code.

## Documentation
* [ClojuRust crate](https://clojurust.github.io/clojurust.doc/clojurust/) Rust version of Clojure Java Core
* [Im crate](https://clojurust.github.io/clojurust.doc/im/) Immutable data structures for Rust by Bodil

## Code projects
* [ClojuRust Core](https://github.com/clojurust/clojurust) ClojuRust library
* [ClojuRust macros](https://github.com/clojurust/clojurust-macros) Macros to standardize ClojuRust


## Licence
This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. 

If a copy of the MPL was not distributed within this file, You can obtain one at http://mozilla.org/MPL/2.0/.

    Copyright (c) 2020 Ivan Pierre, kilroySoft, <Ivan Pierre, ivan@kilroysoft.ch>, under MPL 2.0.

### Persistent data structures licence
This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. 

If a copy of the MPL was not distributed within this file, You can obtain one at http://mozilla.org/MPL/2.0/.

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

The goal to use this type of licence by Rich is to keep control of evolution of the Clojure project core, and guarantee not have an overwhelming core with elements that could be added w/o problem as external libraries.

This project goal is not to make a fork of Clojure core, but to make a Rust version as close as possible to the original project. Code creation is quite different of the original. Differences are due to the limitations or possibilities caused by implementation context of the Rust ecosystem, else this would be a fork or more probably a completely different project made from scratch.

### clojure.core original code
The goal is to compile the originally written `clojure.core` code, so normally the original code will remain in the original repository, and so final executable should involve in parallel with the main project.

This will perhaps no be feasible, but only implementation problems should be cause of particular rewrites, and enhancement of the Rust core should be prioritized. For example the compilation in JVM byte code will be of no usage, but the AST analysis will be valid.
