# ClojuRust
A proof of concept version of Clojure in Rust.



##  Opiniated way to proceed, a wishfull thinking
* Begining with documentation changes before testing 
or implementing code, so to be up-to-date.
* integrating TTD into modules, easing the link between tests and 
code.

## Goals
* Define an internal object model.
* Translate Clojure java code to Rust.
* Test a way to have an interpreter of Clojure code.
* Test a way to have a VM based on a FORTH system.
* Test a way to dynamically compile Clojure code in machine code.
* Test a way to create Rust code to compile Clojure code.

## Documentation
The whole documentation is located in the [wiki](https://github.com/ivanpierre/clojurust/wiki)

## Copyright
Copyright (c) 2020 Ivan Pierre, kilroySoft, <Ivan Pierre, ivan@kilroysoft.ch>

As clojure::lang::* is a translation of original code of Java Clojure
core, reference is due to Rich Hickey:

Copyright (c) Rich Hickey. All rights reserved.

Original project can be found in the main repository:

https://github.com/clojure/clojure

## Licence
This Source Code Form is subject to the terms of the 
Mozilla Public License, v. 2.0. 

If a copy of the MPL was not distributed within this
file, You can obtain one at http://mozilla.org/MPL/2.0/.

### Original Clojure licence
As clojure::lang is on Eclipse Public License 1.0 (http://opensource.org/licenses/eclipse-1.0.php), there could be some problem of double
licence problem.

As is, I put this advice of the original project licence:
*   Clojure
*   Copyright (c) Rich Hickey. All rights reserved.
*   The use and distribution terms for this software are covered by the
*   Eclipse Public License 1.0 (http://opensource.org/licenses/eclipse-1.0.php)
*   which can be found in the file epl-v10.html at the root of this distribution.
*   By using this software in any fashion, you are agreeing to be bound by
* 	 the terms of this license.
*   You must not remove this notice, or any other, from this software.

The goal to use this type of licence by Rich is to keep control of evolution of the Clojure project core, and garantee not to overwhelm core with elements that could be added w/o problem as external libraries.

This project goal is not to make a fork of Clojure core, but to make 
a Rust version as close as possible to the original project. Code creation is quite different of the original. Differences are due to the limitations or possibilities caused by implementation context of the Rust ecosystem, else this would be a fork or more probably a completely different project made from scratch.

### clojure.core original code
The goal is to compile the Clojure original written Clojure code, so normally the original code will remain in the original repository, and so final executable should envolve in parallel with the main project.

This will perhaps no be feasible, but only implementation problems should be particular rewrites, and enhancement of the Rust core
should be prioritized.