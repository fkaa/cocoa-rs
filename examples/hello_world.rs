#[macro_use(autorelease)]
extern crate cocoa;

use cocoa::base::{selector, nil, NO};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize,
						NSAutoreleasePool, NSProcessInfo, NSString};
use cocoa::appkit::{NSApplication, NSApplicationActivationPolicyRegular,
					NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
					NSMenu, NSMenuItem, NSRunningApplication,
					NSApplicationActivateIgnoringOtherApps};

use std::ffi::CString;

fn main() {
	unsafe {
        autorelease! {
            let app = NSApplication::sharedApplication();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

            let menubar = NSMenu::new();
            let menuitem = {
                let item = NSMenuItem::new();

                let appmenu = {
                    let menu = NSMenu::new();

                    let quit_prefix = NSString::alloc().initWithUTF8String(CString::new("Quit ").unwrap().as_ptr());
                    let quit_title = quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo().processName());
                    let quit_action = selector("terminate:");
                    let quit_key = NSString::alloc().initWithUTF8String(CString::new("q").unwrap().as_ptr());

                    let quit_item = NSMenuItem::alloc().initWithTitle_action_keyEquivalent_(
                        quit_title,
                        quit_action,
                        quit_key
                    );

                    menu.addItem_(quit_item);
                    menu
                };

                item.setSubmenu_(appmenu);
                item
            };

            menubar.addItem_(menuitem);
            app.setMainMenu_(menubar);

            let window = NSWindow::alloc().initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
                NSTitledWindowMask as NSUInteger,
                NSBackingStoreBuffered,
                NO
            );
            window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
            window.center();

            let title = NSString::alloc().initWithUTF8String(CString::new("Hello World!").unwrap().as_ptr());        
            window.setTitle_(title);
            window.makeKeyAndOrderFront_(nil);

            let current_app = NSRunningApplication::currentApplication();
            current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

            app.run();
        }
	}
}
