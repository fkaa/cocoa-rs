// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use objc::runtime;

use std::marker::PhantomData;
use std::ops::Deref;
use std::mem;

pub use objc::runtime::{BOOL, NO, YES};

pub type Class = *mut runtime::Class;
pub type SEL = runtime::Sel;

#[allow(non_camel_case_types)]
pub struct id<T=()>(pub *mut runtime::Object, pub PhantomData<T>);

impl<T, R> Deref for id<(T, R)> {
    type Target = id<R>;
    fn deref(&self) -> &id<R> { unsafe { mem::transmute(self) } }
}

#[allow(non_upper_case_globals)]
pub const nil: id = id(0 as *mut runtime::Object, PhantomData);
#[allow(non_upper_case_globals)]
pub const Nil: Class = 0 as Class;

/// A convenience method to convert the name of a class to the class object itself.
#[inline]
pub fn class(name: &str) -> Class {
    unsafe {
        mem::transmute(runtime::Class::get(name))
    }
}

/// A convenience method to convert the name of a selector to the selector object.
#[inline]
pub fn selector(name: &str) -> SEL {
    runtime::Sel::register(name)
}

