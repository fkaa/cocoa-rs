// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{id, class, BOOL, nil};
use libc;
use objc;

use std::marker::PhantomData;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;

const UTF8_ENCODING: usize = 4;

#[repr(C)]
pub struct NSPoint {
    pub x: f64,
    pub y: f64,
}

impl NSPoint {
    #[inline]
    pub fn new(x: f64, y: f64) -> NSPoint {
        NSPoint {
            x: x,
            y: y,
        }
    }
}

unsafe impl objc::Encode for NSPoint {
    fn encode() -> objc::Encoding {
        let encoding = format!("{{CGPoint={}{}}}",
                               f64::encode().as_str(),
                               f64::encode().as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

#[repr(C)]
pub struct NSSize {
    pub width: f64,
    pub height: f64,
}

impl NSSize {
    #[inline]
    pub fn new(width: f64, height: f64) -> NSSize {
        NSSize {
            width: width,
            height: height,
        }
    }
}

unsafe impl objc::Encode for NSSize {
    fn encode() -> objc::Encoding {
        let encoding = format!("{{CGSize={}{}}}",
                               f64::encode().as_str(),
                               f64::encode().as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

#[repr(C)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}

impl NSRect {
    #[inline]
    pub fn new(origin: NSPoint, size: NSSize) -> NSRect {
        NSRect {
            origin: origin,
            size: size
        }
    }
}

#[repr(C)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

impl NSRange {
    #[inline]
    pub fn new(location: NSUInteger, length: NSUInteger) -> NSRange {
        NSRange {
            location: location,
            length: length
        }
    }
}

unsafe impl objc::Encode for NSRect {
    fn encode() -> objc::Encoding {
        let encoding = format!("{{CGRect={}{}}}",
                               NSPoint::encode().as_str(),
                               NSSize::encode().as_str());
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

// Same as CGRectEdge
#[repr(u32)]
pub enum NSRectEdge {
    NSRectMinXEdge,
    NSRectMinYEdge,
    NSRectMaxXEdge,
    NSRectMaxYEdge,
}

#[link(name = "Foundation", kind = "framework")]
extern {
    pub static NSDefaultRunLoopMode: NSObject;
}

pub enum NSObjectPrototype {}
pub type NSObject = id<(NSObjectPrototype, ())>;

impl<T> id<(NSObjectPrototype, T)> {
    pub unsafe fn alloc() -> Self { id(msg_send![class("NSObject"), alloc], PhantomData) }
    pub unsafe fn init(&self) -> Self { id(msg_send![self.0, init], PhantomData) }
    
    pub unsafe fn retain(&self) { msg_send![self.0, retain] }
    pub unsafe fn release(&self) { msg_send![self.0, release] }
}

pub enum NSAutoreleasePoolPrototype {}
pub type NSAutoreleasePool = id<(NSAutoreleasePoolPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSAutoreleasePoolPrototype, T)> {
    pub unsafe fn alloc() -> Self { id(msg_send![class("NSAutoreleasePool"), alloc], PhantomData) }
    pub unsafe fn init(&self) -> Self { id(msg_send![self.0, init], PhantomData) }
    
    pub unsafe fn autorelease(&self) { msg_send![self.0, autorelease] }
    pub unsafe fn drain(&self) { msg_send![self.0, drain] }
}

#[macro_export]
macro_rules! autorelease {
    (@as_expr $e:expr) => { $e };

    ($($body:tt)*) => {{
        let _pool = NSAutoreleasePool::alloc().init();
        autorelease!(@as_expr {$($body)*});
        _pool.release();
    }};
}


pub enum NSProcessInfoPrototype {}
pub type NSProcessInfo = id<(NSProcessInfoPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSProcessInfoPrototype, T)> {
    pub unsafe fn processInfo() -> Self { id(msg_send![class("NSProcessInfo"), processInfo], PhantomData) }
    pub unsafe fn processName(self) -> NSString { id(msg_send![self.0, processName], PhantomData) }
}

pub enum NSValuePrototype {}
pub type NSValue = id<(NSValuePrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSValuePrototype, T)> {
    pub unsafe fn valueWithPoint(point: NSPoint) -> Self { 
        id(msg_send![class("NSValue"), valueWithPoint:point], PhantomData)
    }

    pub unsafe fn valueWithSize(size: NSSize) -> Self {
        id(msg_send![class("NSValue"), valueWithSize:size], PhantomData)
    }
}

pub struct NSArrayPrototype<T> { _phantom: PhantomData<T> }
pub type NSArray<T> = id<(NSArrayPrototype<T>, (NSObjectPrototype, ()))>;

impl<T, V> id<(NSArrayPrototype<V>, T)> {
    pub unsafe fn array() -> Self {
        id(msg_send![class("NSArray"), array], PhantomData)
    }

    pub unsafe fn arrayWithObjects(objects: &[V]) -> Self {
        id(msg_send![class("NSArray"), arrayWithObjects:objects.as_ptr()
                                       count:objects.len()], PhantomData)
    }

    pub unsafe fn arrayWithObject(object: V) -> Self {
        id(msg_send![class("NSArray"), arrayWithObject:object], PhantomData)
    }

    pub unsafe fn arrayByAddingObject(self, object: V) -> Self {
        id(msg_send![self.0, arrayByAddingObject:object], PhantomData)
    }

    pub unsafe fn arrayByAddingObjectsFromArray(self, objects: Self) -> Self {
        id(msg_send![self.0, arrayByAddingObjectsFromArray:objects.0], PhantomData)
    }
}

pub struct NSDictionaryPrototype<K, V> { _phantom: PhantomData<(K, V)> }
pub type NSDictionary<K, V> = id<(NSDictionaryPrototype<K, V>, (NSObjectPrototype, ()))>;

impl<K, V, T> id<(NSDictionaryPrototype<K, V>, T)> {
    pub unsafe fn dictionary() -> Self {
        id(msg_send![class("NSDictionary"), dictionary], PhantomData)
    }

    pub unsafe fn count(self) -> NSUInteger {
        msg_send![self.0, count]
    }

    /*pub unsafe fn objectForKey(self, key: K) -> V {
        msg_send![self.0, objectForKey:key]
    }*/
}

pub struct NSSetPrototype<T> { _phantom: PhantomData<T> }
pub type NSSet<T> = id<(NSSetPrototype<T>, (NSObjectPrototype, ()))>;

impl<T, R> id<(NSSetPrototype<T>, R)> {
    pub unsafe fn set() -> Self {
        id(msg_send![class("NSSet"), set], PhantomData)
    }
}

pub enum NSStringPrototype {}
pub type NSString = id<(NSStringPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSStringPrototype, T)> {
    pub unsafe fn alloc() -> Self {
        id(msg_send![class("NSString"), alloc], PhantomData)
    }

    pub unsafe fn init(self) -> Self {
        id(msg_send![self.0, init], PhantomData)
    }

    pub unsafe fn initWithUTF8String(self, bytes: *const libc::c_char) -> Self {
        id(msg_send![self.0, initWithUTF8String:bytes], PhantomData)
    }

    pub unsafe fn stringByAppendingString_(self, other: Self) -> Self {
        id(msg_send![self.0, stringByAppendingString:other.0], PhantomData)
    }

    pub unsafe fn UTF8String(self) -> *const libc::c_char {
        msg_send![self.0, UTF8String]
    }

    pub unsafe fn length(self) -> NSUInteger {
        msg_send![self.0, length]
    }
}

pub enum NSDatePrototype {}
pub type NSDate = id<(NSDatePrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSDatePrototype, T)> {
    pub unsafe fn distantPast() -> Self {
        id(msg_send![class("NSDate"), distantPast], PhantomData)
    }

    pub unsafe fn distantFuture() -> Self {
        id(msg_send![class("NSDate"), distantFuture], PhantomData)
    }
}

pub type NSTimeInterval = libc::c_double;

