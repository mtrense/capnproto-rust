/*
 * Copyright (c) 2013, David Renshaw (dwrenshaw@gmail.com)
 *
 * See the LICENSE file in the capnproto-rust root directory.
 */

#[feature(globs)];
#[feature(macro_rules)];

#[crate_id="capnp"];
#[crate_type = "lib"];


pub mod common;
pub mod endian;
pub mod mask;
pub mod blob;
pub mod layout;
pub mod pointer_helpers;
pub mod any;
pub mod arena;
pub mod message;
pub mod io;
pub mod serialize;
pub mod serialize_packed;
pub mod list;

#[cfg(test)]
pub mod layout_test;
#[cfg(test)]
pub mod serialize_packed_test;