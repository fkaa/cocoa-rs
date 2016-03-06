#[macro_use(autorelease)]
extern crate cocoa;

use cocoa::base::{selector, nil, NO};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize,
						NSAutoreleasePool, NSProcessInfo, NSString};

use std::ffi::{CString, CStr};

fn main() {
    unsafe {
        let name = NSProcessInfo::processInfo().processName();

        println!("Process name: {:?}", CStr::from_ptr(name.UTF8String()));

        autorelease! {
            let string = NSString::alloc().initWithUTF8String(CString::new("Jääääres").unwrap().as_ptr());
    
            println!("Process name: {:?}", CStr::from_ptr(string.UTF8String()));
        }
    }
}
