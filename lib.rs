// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Rust bindings to the Core Video framework on Mac OS X.

#![crate_name="core_video"]
#![crate_type="rlib"]

#[cfg(target_os="macos")]
extern crate core_foundation;
#[cfg(target_os="macos")]
extern crate libc;

#[cfg(target_os="macos")]
pub mod display_link;
#[cfg(target_os="macos")]
pub mod time;

pub type CVReturn = i32;

pub static kCVReturnSuccess: CVReturn = 0;
pub static kCVReturnFirst: CVReturn = -6660;
pub static kCVReturnError: CVReturn = kCVReturnFirst;
pub static kCVReturnLast: CVReturn = -6699;

