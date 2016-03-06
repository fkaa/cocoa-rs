// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use base::{id, class, BOOL, SEL};
use foundation::{NSInteger, NSUInteger, NSTimeInterval,
                 NSPoint, NSSize, NSRect, NSRectEdge,
                 NSObjectPrototype, NSString, NSDictionary,
                 NSArray, NSSet, NSDate};

use libc;

use std::marker::PhantomData;

pub use core_graphics::base::CGFloat;
pub use core_graphics::geometry::CGPoint;

pub use self::NSApplicationActivationPolicy::*;
pub use self::NSApplicationActivationOptions::*;
pub use self::NSWindowMask::*;
pub use self::NSBackingStoreType::*;
pub use self::NSOpenGLPixelFormatAttribute::*;
pub use self::NSOpenGLPFAOpenGLProfiles::*;
pub use self::NSEventType::*;

pub type CGLContextObj = *mut libc::c_void;

pub type GLint = libc::int32_t;

#[link(name = "AppKit", kind = "framework")]
extern {
    pub static NSAppKitVersionNumber: f64;
}

pub const NSAppKitVersionNumber10_0: f64 = 577.0;
pub const NSAppKitVersionNumber10_1: f64 = 620.0;
pub const NSAppKitVersionNumber10_2: f64 = 663.0;
pub const NSAppKitVersionNumber10_2_3: f64 = 663.6;
pub const NSAppKitVersionNumber10_3: f64 = 743.0;
pub const NSAppKitVersionNumber10_3_2: f64 = 743.14;
pub const NSAppKitVersionNumber10_3_3: f64 = 743.2;
pub const NSAppKitVersionNumber10_3_5: f64 = 743.24;
pub const NSAppKitVersionNumber10_3_7: f64 = 743.33;
pub const NSAppKitVersionNumber10_3_9: f64 = 743.36;
pub const NSAppKitVersionNumber10_4: f64 = 824.0;
pub const NSAppKitVersionNumber10_4_1: f64 = 824.1;
pub const NSAppKitVersionNumber10_4_3: f64 = 824.23;
pub const NSAppKitVersionNumber10_4_4: f64 = 824.33;
pub const NSAppKitVersionNumber10_4_7: f64 = 824.41;
pub const NSAppKitVersionNumber10_5: f64 = 949.0;
pub const NSAppKitVersionNumber10_5_2: f64 = 949.27;
pub const NSAppKitVersionNumber10_5_3: f64 = 949.33;
pub const NSAppKitVersionNumber10_6: f64 = 1038.0;
pub const NSAppKitVersionNumber10_7: f64 = 1138.0;
pub const NSAppKitVersionNumber10_7_2: f64 = 1138.23;
pub const NSAppKitVersionNumber10_7_3: f64 = 1138.32;
pub const NSAppKitVersionNumber10_7_4: f64 = 1138.47;
pub const NSAppKitVersionNumber10_8: f64 = 1187.0;
pub const NSAppKitVersionNumber10_9: f64 = 1265.0;

#[repr(i64)]
pub enum NSApplicationActivationPolicy {
    NSApplicationActivationPolicyRegular = 0,
    NSApplicationActivationPolicyAccessory = 1,
    NSApplicationActivationPolicyERROR = -1
}

#[repr(u64)]
pub enum NSApplicationActivationOptions {
    NSApplicationActivateAllWindows = 1 << 0,
    NSApplicationActivateIgnoringOtherApps = 1 << 1
}

#[repr(u64)]
pub enum NSApplicationTerminateReply {
    NSTerminateCancel = 0,
    NSTerminateNow = 1,
    NSTerminateLater = 2,
}

#[repr(u64)]
pub enum NSWindowMask {
    NSBorderlessWindowMask      = 0,
    NSTitledWindowMask          = 1 << 0,
    NSClosableWindowMask        = 1 << 1,
    NSMiniaturizableWindowMask  = 1 << 2,
    NSResizableWindowMask       = 1 << 3,

    NSTexturedBackgroundWindowMask  = 1 << 8,

    NSUnifiedTitleAndToolbarWindowMask  = 1 << 12,

    NSFullScreenWindowMask      = 1 << 14,

    NSFullSizeContentViewWindowMask = 1 << 15
}

#[repr(u64)]
pub enum NSWindowTitleVisibility {
    NSWindowTitleVisible = 0,
    NSWindowTitleHidden = 1
}

#[repr(u64)]
pub enum NSBackingStoreType {
    NSBackingStoreRetained      = 0,
    NSBackingStoreNonretained   = 1,
    NSBackingStoreBuffered      = 2
}

bitflags! {
    flags NSWindowOrderingMode: NSInteger {
        const NSWindowAbove =  1,
        const NSWindowBelow = -1,
        const NSWindowOut   =  0,
    }
}

bitflags! {
    flags NSAlignmentOptions: libc::c_ulonglong {
        const NSAlignMinXInward         = 1 << 0,
        const NSAlignMinYInward         = 1 << 1,
        const NSAlignMaxXInward         = 1 << 2,
        const NSAlignMaxYInward         = 1 << 3,
        const NSAlignWidthInward        = 1 << 4,
        const NSAlignHeightInward       = 1 << 5,
        const NSAlignMinXOutward        = 1 << 8,
        const NSAlignMinYOutward        = 1 << 9,
        const NSAlignMaxXOutward        = 1 << 10,
        const NSAlignMaxYOutward        = 1 << 11,
        const NSAlignWidthOutward       = 1 << 12,
        const NSAlignHeightOutward      = 1 << 13,
        const NSAlignMinXNearest        = 1 << 16,
        const NSAlignMinYNearest        = 1 << 17,
        const NSAlignMaxXNearest        = 1 << 18,
        const NSAlignMaxYNearest        = 1 << 19,
        const NSAlignWidthNearest       = 1 << 20,
        const NSAlignHeightNearest      = 1 << 21,
        const NSAlignRectFlipped        = 1 << 63,
        const NSAlignAllEdgesInward     = NSAlignMinXInward.bits
                                        | NSAlignMaxXInward.bits
                                        | NSAlignMinYInward.bits
                                        | NSAlignMaxYInward.bits,
        const NSAlignAllEdgesOutward    = NSAlignMinXOutward.bits
                                        | NSAlignMaxXOutward.bits
                                        | NSAlignMinYOutward.bits
                                        | NSAlignMaxYOutward.bits,
        const NSAlignAllEdgesNearest    = NSAlignMinXNearest.bits
                                        | NSAlignMaxXNearest.bits
                                        | NSAlignMinYNearest.bits
                                        | NSAlignMaxYNearest.bits,
    }
}

#[repr(u64)]
pub enum NSOpenGLPixelFormatAttribute {
    NSOpenGLPFAAllRenderers             = 1,
    NSOpenGLPFATripleBuffer             = 3,
    NSOpenGLPFADoubleBuffer             = 5,
    NSOpenGLPFAStereo                   = 6,
    NSOpenGLPFAAuxBuffers               = 7,
    NSOpenGLPFAColorSize                = 8,
    NSOpenGLPFAAlphaSize                = 11,
    NSOpenGLPFADepthSize                = 12,
    NSOpenGLPFAStencilSize              = 13,
    NSOpenGLPFAAccumSize                = 14,
    NSOpenGLPFAMinimumPolicy            = 51,
    NSOpenGLPFAMaximumPolicy            = 52,
    NSOpenGLPFAOffScreen                = 53,
    NSOpenGLPFAFullScreen               = 54,
    NSOpenGLPFASampleBuffers            = 55,
    NSOpenGLPFASamples                  = 56,
    NSOpenGLPFAAuxDepthStencil          = 57,
    NSOpenGLPFAColorFloat               = 58,
    NSOpenGLPFAMultisample              = 59,
    NSOpenGLPFASupersample              = 60,
    NSOpenGLPFASampleAlpha              = 61,
    NSOpenGLPFARendererID               = 70,
    NSOpenGLPFASingleRenderer           = 71,
    NSOpenGLPFANoRecovery               = 72,
    NSOpenGLPFAAccelerated              = 73,
    NSOpenGLPFAClosestPolicy            = 74,
    NSOpenGLPFARobust                   = 75,
    NSOpenGLPFABackingStore             = 76,
    NSOpenGLPFAMPSafe                   = 78,
    NSOpenGLPFAWindow                   = 80,
    NSOpenGLPFAMultiScreen              = 81,
    NSOpenGLPFACompliant                = 83,
    NSOpenGLPFAScreenMask               = 84,
    NSOpenGLPFAPixelBuffer              = 90,
    NSOpenGLPFARemotePixelBuffer        = 91,
    NSOpenGLPFAAllowOfflineRenderers    = 96,
    NSOpenGLPFAAcceleratedCompute       = 97,
    NSOpenGLPFAOpenGLProfile            = 99,
    NSOpenGLPFAVirtualScreenCount       = 128,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
pub enum NSOpenGLPFAOpenGLProfiles {
    NSOpenGLProfileVersionLegacy = 0x1000,
    NSOpenGLProfileVersion3_2Core = 0x3200,
    NSOpenGLProfileVersion4_1Core = 0x4100,
}

#[repr(u64)]
pub enum NSOpenGLContextParameter {
    NSOpenGLCPSwapInterval          = 222,
    NSOpenGLCPSurfaceOrder          = 235,
    NSOpenGLCPSurfaceOpacity        = 236,
    NSOpenGLCPSurfaceBackingSize    = 304,
    NSOpenGLCPReclaimResources      = 308,
    NSOpenGLCPCurrentRendererID     = 309,
    NSOpenGLCPGPUVertexProcessing   = 310,
    NSOpenGLCPGPUFragmentProcessing = 311,
    NSOpenGLCPHasDrawable           = 314,
    NSOpenGLCPMPSwapsInFlight       = 315,
}

#[repr(u64)]
#[derive(Copy, Clone, PartialEq)]
pub enum NSWindowButton {
    NSWindowCloseButton            = 0,
    NSWindowMiniaturizeButton      = 1,
    NSWindowZoomButton             = 2,
    NSWindowToolbarButton          = 3,
    NSWindowDocumentIconButton     = 4,
    NSWindowDocumentVersionsButton = 6,
    NSWindowFullScreenButton       = 7,
}

pub static NSMainMenuWindowLevel: libc::int32_t = 24;

pub enum NSApplicationPrototype {}
pub type NSApplication = id<(NSApplicationPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSApplicationPrototype, T)> {
    pub unsafe fn sharedApplication() -> Self {
        id(msg_send![class("NSApplication"), sharedApplication], PhantomData)
    }

    pub unsafe fn setActivationPolicy_(&self, policy: NSApplicationActivationPolicy) -> BOOL {
        msg_send![self.0, setActivationPolicy:policy as NSInteger]
    }

    pub unsafe fn setMainMenu_(&self, menu: NSMenu) {
        msg_send![self.0, setMainMenu:menu]
    }

    pub unsafe fn setServicesMenu_(&self, menu: NSMenu) {
        msg_send![self.0, setServicesMenu:menu]
    }

    pub unsafe fn activateIgnoringOtherApps_(&self, ignore: BOOL) {
        msg_send![self.0, activateIgnoringOtherApps:ignore]
    }

    pub unsafe fn run(&self) {
        msg_send![self.0, run]
    }

    pub unsafe fn finishLaunching(&self) {
        msg_send![self.0, finishLaunching]
    }

    pub unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(&self,
                                                              mask: NSUInteger,
                                                              expiration: NSDate,
                                                              in_mode: NSString,
                                                              dequeue: BOOL) -> NSEvent {
        id(msg_send![self.0, nextEventMatchingMask:mask
                                         untilDate:expiration
                                            inMode:in_mode
                                           dequeue:dequeue], PhantomData)
    }

    pub unsafe fn sendEvent_(&self, an_event: NSEvent) {
        msg_send![self.0, sendEvent:an_event]
    }

    pub unsafe fn postEvent_atStart_(&self, anEvent: NSEvent, flag: BOOL) {
        msg_send![self.0, postEvent:anEvent atStart:flag]
    }

    pub unsafe fn stop_<A>(&self, sender: id<A>) {
        msg_send![self.0, stop:sender]
    }
}

pub enum NSRunningApplicationPrototype {}
pub type NSRunningApplication = id<(NSRunningApplicationPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSRunningApplicationPrototype, T)> {
    pub unsafe fn currentApplication() -> Self {
        id(msg_send![class("NSRunningApplication"), currentApplication], PhantomData)
    }

    pub unsafe fn activateWithOptions_(&self, options: NSApplicationActivationOptions) -> BOOL {
        msg_send![self.0, activateWithOptions:options as NSUInteger]
    }
}

pub enum NSMenuPrototype {}
pub type NSMenu = id<(NSMenuPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSMenuPrototype, T)> {
    pub unsafe fn new() -> Self {
        id(msg_send![class("NSMenu"), new], PhantomData)
    }

    pub unsafe fn setAutoenablesItems(&self, state: BOOL) {
        msg_send![self.0, setAutoenablesItems: state]
    }

    pub unsafe fn addItem_(&self, menu_item: NSMenuItem) {
        msg_send![self.0, addItem:menu_item]
    }

    pub unsafe fn addItemWithTitle_action_keyEquivalent(&self, title: NSString, action: SEL, key: NSString) -> NSMenuItem {
        msg_send![self.0, addItemWithTitle:title action:action keyEquivalent:key]
    }
}

pub enum NSMenuItemPrototype {}
pub type NSMenuItem = id<(NSMenuItemPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSMenuItemPrototype, T)> {
    pub unsafe fn alloc() -> Self {
        id(msg_send![class("NSMenuItem"), alloc], PhantomData)
    }

    pub unsafe fn new() -> Self {
        id(msg_send![class("NSMenuItem"), new], PhantomData)
    }

    pub unsafe fn separatorItem() -> Self {
        id(msg_send![class("NSMenuItem"), separatorItem], PhantomData)
    }

    pub unsafe fn initWithTitle_action_keyEquivalent_(&self, title: NSString, action: SEL, key: NSString) -> Self {
        id(msg_send![self.0, initWithTitle:title action:action keyEquivalent:key], PhantomData)
    }

    pub unsafe fn setKeyEquivalentModifierMask_(&self, mask: NSEventModifierFlags) {
        msg_send![self.0, setKeyEquivalentModifierMask:mask]
    }

    pub unsafe fn setSubmenu_(&self, submenu: NSMenu) {
        msg_send![self.0, setSubmenu:submenu]
    }

}

pub type NSWindowDepth = libc::c_int;

bitflags! {
    flags NSWindowCollectionBehavior: NSUInteger {
        const NSWindowCollectionBehaviorDefault = 0,
        const NSWindowCollectionBehaviorCanJoinAllSpaces = 1 << 0,
        const NSWindowCollectionBehaviorMoveToActiveSpace = 1 << 1,

        const NSWindowCollectionBehaviorManaged = 1 << 2,
        const NSWindowCollectionBehaviorTransient = 1 << 3,
        const NSWindowCollectionBehaviorStationary = 1 << 4,

        const NSWindowCollectionBehaviorParticipatesInCycle = 1 << 5,
        const NSWindowCollectionBehaviorIgnoresCycle = 1 << 6,

        const NSWindowCollectionBehaviorFullScreenPrimary = 1 << 7,
        const NSWindowCollectionBehaviorFullScreenAuxiliary = 1 << 8,
    }
}

bitflags! {
    flags NSWindowOcclusionState: NSUInteger {
        const NSWindowOcclusionStateVisible = 1 << 1
    }
}

pub enum NSWindowPrototype {}
pub type NSWindow = id<(NSWindowPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSWindowPrototype, T)> {
    pub unsafe fn alloc() -> Self {
        id(msg_send![class("NSWindow"), alloc], PhantomData)
    }

    pub unsafe fn initWithContentRect_styleMask_backing_defer_(&self,
                                                           rect: NSRect,
                                                           style: NSUInteger,
                                                           backing: NSBackingStoreType,
                                                           defer: BOOL) -> Self {
        id(msg_send![self.0, initWithContentRect:rect
                                       styleMask:style
                                         backing:backing as NSUInteger
                                           defer:defer], PhantomData)
    }

    pub unsafe fn initWithContentRect_styleMask_backing_defer_screen_(&self,
                                                                  rect: NSRect,
                                                                  style: NSUInteger,
                                                                  backing: NSBackingStoreType,
                                                                  defer: BOOL,
                                                                  screen: NSScreen) -> Self {
        id(msg_send![self.0, initWithContentRect:rect
                                       styleMask:style
                                         backing:backing as NSUInteger
                                           defer:defer
                                          screen:screen], PhantomData)
    }

    // Configuring Windows

    pub unsafe fn styleMask(&self) -> NSUInteger {
        msg_send![self.0, styleMask]
    }

    pub unsafe fn setStyleMask_(&self, styleMask: NSUInteger) {
        msg_send![self.0, setStyleMask:styleMask]
    }

    pub unsafe fn toggleFullScreen_<A>(&self, sender: id<A>) {
        msg_send![self.0, toggleFullScreen:sender]
    }

    pub unsafe fn worksWhenModal(&self) -> BOOL {
        msg_send![self.0, worksWhenModal]
    }

    pub unsafe fn alphaValue(&self) -> CGFloat {
        msg_send![self.0, alphaValue]
    }

    pub unsafe fn setAlphaValue_(&self, windowAlpha: CGFloat) {
        msg_send![self.0, setAlphaValue:windowAlpha]
    }

    pub unsafe fn backgroundColor(&self) -> NSColor {
        msg_send![self.0, backgroundColor]
    }

    pub unsafe fn setBackgroundColor_(&self, color: NSColor) {
        msg_send![self.0, setBackgroundColor:color]
    }

    pub unsafe fn colorSpace(&self) -> NSColor {
        msg_send![self.0, colorSpace]
    }

    pub unsafe fn setColorSpace_(&self, colorSpace: NSColorSpace) {
        msg_send![self.0, setColorSpace:colorSpace]
    }

    pub unsafe fn contentView(&self) -> NSView {
        msg_send![self.0, contentView]
    }

    pub unsafe fn setContentView_(&self, view: NSView) {
        msg_send![self.0, setContentView:view]
    }

    pub unsafe fn canHide(&self) -> BOOL {
        msg_send![self.0, canHide]
    }

    pub unsafe fn setCanHide_(&self, canHide: BOOL) {
        msg_send![self.0, setCanHide:canHide]
    }

    pub unsafe fn hidesOnDeactivate(&self) -> BOOL {
        msg_send![self.0, hidesOnDeactivate]
    }

    pub unsafe fn setHidesOnDeactivate_(&self, hideOnDeactivate: BOOL) {
        msg_send![self.0, setHidesOnDeactivate:hideOnDeactivate]
    }

    pub unsafe fn collectionBehavior(&self) -> NSWindowCollectionBehavior {
        msg_send![self.0, collectionBehavior]
    }

    pub unsafe fn setCollectionBehavior_(&self, collectionBehavior: NSWindowCollectionBehavior) {
        msg_send![self.0, setCollectionBehavior:collectionBehavior]
    }

    pub unsafe fn setOpaque_(&self, opaque: BOOL) {
        msg_send![self.0, setOpaque:opaque]
    }

    pub unsafe fn hasShadow(&self) -> BOOL {
        msg_send![self.0, hasShadow]
    }

    pub unsafe fn setHasShadow_(&self, hasShadow: BOOL) {
        msg_send![self.0, setHasShadow:hasShadow]
    }

    pub unsafe fn invalidateShadow(&self) {
        msg_send![self.0, invalidateShadow]
    }

    pub unsafe fn autorecalculatesContentBorderThicknessForEdge_(&self, edge: NSRectEdge) -> BOOL {
        msg_send![self.0, autorecalculatesContentBorderThicknessForEdge:edge]
    }

    pub unsafe fn setAutorecalculatesContentBorderThickness_forEdge_(&self,
                                                                 autorecalculateContentBorderThickness: BOOL,
                                                                 edge: NSRectEdge) -> BOOL {
        msg_send![self.0, setAutorecalculatesContentBorderThickness:
                          autorecalculateContentBorderThickness forEdge:edge]
    }

    pub unsafe fn contentBorderThicknessForEdge_(&self, edge: NSRectEdge) -> CGFloat {
        msg_send![self.0, contentBorderThicknessForEdge:edge]
    }

    pub unsafe fn setContentBorderThickness_forEdge_(&self, borderThickness: CGFloat, edge: NSRectEdge) {
        msg_send![self.0, setContentBorderThickness:borderThickness forEdge:edge]
    }

    pub unsafe fn delegate<A>(&self) -> id<A> {
        id(msg_send![self.0, delegate], PhantomData)
    }

    pub unsafe fn setDelegate_<A>(&self, delegate: id<A>) {
        msg_send![self.0, setDelegate:delegate]
    }

    pub unsafe fn preventsApplicationTerminationWhenModal(&self) -> BOOL {
        msg_send![self.0, preventsApplicationTerminationWhenModal]
    }

    pub unsafe fn setPreventsApplicationTerminationWhenModal_(&self, flag: BOOL) {
        msg_send![self.0, setPreventsApplicationTerminationWhenModal:flag]
    }

    // TODO: Accessing Window Information

    // Getting Layout Information

    pub unsafe fn contentRectForFrameRect_styleMask_(&self, windowFrame: NSRect, windowStyle: NSUInteger) -> NSRect {
        msg_send![self.0, contentRectForFrameRect:windowFrame styleMask:windowStyle]
    }

    pub unsafe fn frameRectForContentRect_styleMask_(&self, windowContentRect: NSRect, windowStyle: NSUInteger) -> NSRect {
        msg_send![self.0, frameRectForContentRect:windowContentRect styleMask:windowStyle]
    }

    pub unsafe fn minFrameWidthWithTitle_styleMask_(&self, windowTitle: NSString, windowStyle: NSUInteger) -> CGFloat {
        msg_send![self.0, minFrameWidthWithTitle:windowTitle styleMask:windowStyle]
    }

    pub unsafe fn contentRectForFrameRect_(&self, windowFrame: NSRect) -> NSRect {
        msg_send![self.0, contentRectForFrameRect:windowFrame]
    }

    pub unsafe fn frameRectForContentRect_(&self, windowContent: NSRect) -> NSRect {
        msg_send![self.0, frameRectForContentRect:windowContent]
    }

    // Managing Windows

    pub unsafe fn drawers(&self) -> NSDrawer {
        msg_send![self.0, drawers]
    }

    pub unsafe fn windowController<A>(&self) -> id<A> {
        id(msg_send![self.0, windowController], PhantomData)
    }

    pub unsafe fn setWindowController_<A>(&self, windowController: id<A>) {
        msg_send![self.0, setWindowController:windowController]
    }

    // TODO: Managing Sheets

    // Sizing Windows

    pub unsafe fn frame(&self) -> NSRect {
        msg_send![self.0, frame]
    }

    pub unsafe fn setFrameOrigin_(&self, point: NSPoint) {
        msg_send![self.0, setFrameOrigin:point]
    }

    pub unsafe fn setFrameTopLeftPoint_(&self, point: NSPoint) {
        msg_send![self.0, setFrameTopLeftPoint:point]
    }

    pub unsafe fn constrainFrameRect_toScreen_(&self, frameRect: NSRect, screen: NSScreen) {
        msg_send![self.0, constrainFrameRect:frameRect toScreen:screen]
    }

    pub unsafe fn cascadeTopLeftFromPoint_(&self, topLeft: NSPoint) -> NSPoint {
        msg_send![self.0, cascadeTopLeftFromPoint:topLeft]
    }

    pub unsafe fn setFrame_display_(&self, windowFrame: NSRect, display: BOOL) {
        msg_send![self.0, setFrame:windowFrame display:display]
    }

    pub unsafe fn setFrame_displayViews_(&self, windowFrame: NSRect, display: BOOL) {
        msg_send![self.0, setFrame:windowFrame displayViews:display]
    }

    pub unsafe fn aspectRatio(&self) -> NSSize {
        msg_send![self.0, aspectRatio]
    }

    pub unsafe fn setAspectRatio_(&self, aspectRatio: NSSize) {
        msg_send![self.0, setAspectRatio:aspectRatio]
    }

    pub unsafe fn minSize(&self) -> NSSize {
        msg_send![self.0, minSize]
    }

    pub unsafe fn setMinSize_(&self, minSize: NSSize) {
        msg_send![self.0, setMinSize:minSize]
    }

    pub unsafe fn maxSize(&self) -> NSSize {
        msg_send![self.0, maxSize]
    }

    pub unsafe fn setMaxSize_(&self, maxSize: NSSize) {
        msg_send![self.0, setMaxSize:maxSize]
    }

    pub unsafe fn performZoom_<A>(&self, sender: id<A>) {
        msg_send![self.0, performZoom:sender]
    }

    pub unsafe fn zoom_<A>(&self, sender: id<A>) {
        msg_send![self.0, zoom:sender]
    }

    pub unsafe fn resizeFlags(&self) -> NSInteger {
        msg_send![self.0, resizeFlags]
    }

    pub unsafe fn showsResizeIndicator(&self) -> BOOL {
        msg_send![self.0, showsResizeIndicator]
    }

    pub unsafe fn setShowsResizeIndicator_(&self, showsResizeIndicator: BOOL) {
        msg_send![self.0, setShowsResizeIndicator:showsResizeIndicator]
    }

    pub unsafe fn resizeIncrements(&self) -> NSSize {
        msg_send![self.0, resizeIncrements]
    }

    pub unsafe fn setResizeIncrements_(&self, resizeIncrements: NSSize) {
        msg_send![self.0, setResizeIncrements:resizeIncrements]
    }

    pub unsafe fn preservesContentDuringLiveResize(&self) -> BOOL {
        msg_send![self.0, preservesContentDuringLiveResize]
    }

    pub unsafe fn setPreservesContentDuringLiveResize_(&self, preservesContentDuringLiveResize: BOOL) {
        msg_send![self.0, setPreservesContentDuringLiveResize:preservesContentDuringLiveResize]
    }

    pub unsafe fn inLiveResize(&self) -> BOOL {
        msg_send![self.0, inLiveResize]
    }

    // Sizing Content

    pub unsafe fn contentAspectRatio(&self) -> NSSize {
        msg_send![self.0, contentAspectRatio]
    }

    pub unsafe fn setContentAspectRatio_(&self, contentAspectRatio: NSSize) {
        msg_send![self.0, setContentAspectRatio:contentAspectRatio]
    }

    pub unsafe fn contentMinSize(&self) -> NSSize {
        msg_send![self.0, contentMinSize]
    }

    pub unsafe fn setContentMinSize_(&self, contentMinSize: NSSize) {
        msg_send![self.0, setContentMinSize:contentMinSize]
    }

    pub unsafe fn contentSize(&self) -> NSSize {
        msg_send![self.0, contentSize]
    }

    pub unsafe fn setContentSize_(&self, contentSize: NSSize) {
        msg_send![self.0, setContentSize:contentSize]
    }

    pub unsafe fn contentMaxSize(&self) -> NSSize {
        msg_send![self.0, contentMaxSize]
    }

    pub unsafe fn setContentMaxSize_(&self, contentMaxSize: NSSize) {
        msg_send![self.0, setContentMaxSize:contentMaxSize]
    }

    pub unsafe fn contentResizeIncrements(&self) -> NSSize {
        msg_send![self.0, contentResizeIncrements]
    }

    pub unsafe fn setContentResizeIncrements_(&self, contentResizeIncrements: NSSize) {
        msg_send![self.0, setContentResizeIncrements:contentResizeIncrements]
    }

    // Managing Window Visibility and Occlusion State

    pub unsafe fn isVisible(&self) -> BOOL {
        msg_send![self.0, isVisible]
    }

    pub unsafe fn occlusionState(&self) -> NSWindowOcclusionState {
        msg_send![self.0, occlusionState]
    }

    // Managing Window Layers

    pub unsafe fn orderOut_<A>(&self, sender: id<A>) {
        msg_send![self.0, orderOut:sender]
    }

    pub unsafe fn orderBack_<A>(&self, sender: id<A>) {
        msg_send![self.0, orderBack:sender]
    }

    pub unsafe fn orderFront_<A>(&self, sender: id<A>) {
        msg_send![self.0, orderFront:sender]
    }

    pub unsafe fn orderFrontRegardless(&self) {
        msg_send![self.0, orderFrontRegardless]
    }

    pub unsafe fn orderFrontWindow_relativeTo_(&self, ordering_mode: NSWindowOrderingMode, other_window_number: NSInteger) {
        msg_send![self.0, orderWindow:ordering_mode relativeTo:other_window_number]
    }

    pub unsafe fn level(&self) -> NSInteger {
        msg_send![self.0, level]
    }

    pub unsafe fn setLevel_(&self, level: NSInteger) {
        msg_send![self.0, setLevel:level]
    }

    // Managing Key Status

    pub unsafe fn canBecomeKeyWindow(&self) -> BOOL {
        msg_send![self.0, canBecomeKeyWindow]
    }

    pub unsafe fn makeKeyWindow(&self) {
        msg_send![self.0, makeKeyWindow]
    }

    pub unsafe fn makeKeyAndOrderFront_<A>(&self, sender: id<A>) {
        msg_send![self.0, makeKeyAndOrderFront:sender]
    }

    // Managing Main Status

    pub unsafe fn canBecomeMainWindow(&self) -> BOOL {
        msg_send![self.0, canBecomeMainWindow]
    }

    pub unsafe fn makeMainWindow(&self) {
        msg_send![self.0, makeMainWindow]
    }

    // TODO: Managing Toolbars
    // TODO: Managing Attached Windows
    // TODO: Managing Window Buffers
    // TODO: Managing Default Buttons
    // TODO: Managing Field Editors
    // TODO: Managing the Window Menu
    // TODO: Managing Cursor Rectangles

    // Managing Title Bars

    pub unsafe fn standardWindowButton_(&self, windowButtonKind: NSWindowButton) -> NSWindowButton {
        msg_send![self.0, standardWindowButton:windowButtonKind]
    }

    // TODO: Managing Tooltips
    // TODO: Handling Events

    // Managing Responders

    pub unsafe fn initialFirstResponder(&self) -> NSView {
        msg_send![self.0, initialFirstResponder]
    }

    pub unsafe fn firstResponder<A>(&self) -> id<A> {
        id(msg_send![self.0, firstResponder], PhantomData)
    }

    pub unsafe fn setInitialFirstResponder_(&self, responder: NSView) {
        msg_send![self.0, setInitialFirstResponder:responder]
    }

    pub unsafe fn makeFirstResponder_<A>(&self, responder: id<A>) -> BOOL {
        msg_send![self.0, makeFirstResponder:responder]
    }

    // TODO: Managing the Key View Loop

    // Handling Keyboard Events

    pub unsafe fn keyDown_(&self, event: NSEvent) {
        msg_send![self.0, keyDown:event]
    }

    // Handling Mouse Events

    pub unsafe fn acceptsMouseMovedEvents(&self) -> BOOL {
        msg_send![self.0, acceptsMouseMovedEvents]
    }

    pub unsafe fn ignoresMouseEvents(&self) -> BOOL {
        msg_send![self.0, ignoresMouseEvents]
    }

    pub unsafe fn setIgnoresMouseEvents_(&self, ignoreMouseEvents: BOOL) {
        msg_send![self.0, setIgnoresMouseEvents:ignoreMouseEvents]
    }

    pub unsafe fn mouseLocationOutsideOfEventStream(&self) -> NSPoint {
        msg_send![self.0, mouseLocationOutsideOfEventStream]
    }

    pub unsafe fn setAcceptsMouseMovedEvents_(&self, acceptMouseMovedEvents: BOOL) {
        msg_send![self.0, setAcceptsMouseMovedEvents:acceptMouseMovedEvents]
    }

    pub unsafe fn windowNumberAtPoint_belowWindowWithWindowNumber_(&self,
                                                               point: NSPoint,
                                                               windowNumber: NSInteger) -> NSInteger {
        msg_send![self.0, windowNumberAtPoint:point belowWindowWithWindowNumber:windowNumber]
    }

    // Converting Coordinates

    pub unsafe fn backingScaleFactor(&self) -> CGFloat {
        msg_send![self.0, backingScaleFactor]
    }

    pub unsafe fn backingAlignedRect_options_(&self, rect: NSRect, options: NSAlignmentOptions) -> NSRect {
        msg_send![self.0, backingAlignedRect:rect options:options]
    }

    pub unsafe fn convertRectFromBacking_(&self, rect: NSRect) -> NSRect {
        msg_send![self.0, convertRectFromBacking:rect]
    }

    pub unsafe fn convertRectToBacking_(&self, rect: NSRect) -> NSRect {
        msg_send![self.0, convertRectToBacking:rect]
    }

    pub unsafe fn convertRectToScreen_(&self, rect: NSRect) -> NSRect {
        msg_send![self.0, convertRectToScreen:rect]
    }

    pub unsafe fn convertRectFromScreen_(&self, rect: NSRect) -> NSRect {
        msg_send![self.0, convertRectFromScreen:rect]
    }

    // Accessing Edited Status

    pub unsafe fn setDocumentEdited_(&self, documentEdited: BOOL) {
        msg_send![self.0, setDocumentEdited:documentEdited]
    }

    // Managing Titles

    pub unsafe fn title(&self) -> NSString {
        msg_send![self.0, title]
    }

    pub unsafe fn setTitle_(&self, title: NSString) {
        msg_send![self.0, setTitle:title]
    }

    pub unsafe fn setTitleWithRepresentedFilename_(&self, filePath: NSString) {
        msg_send![self.0, setTitleWithRepresentedFilename:filePath]
    }

    pub unsafe fn setTitleVisibility_(&self, visibility: NSWindowTitleVisibility) {
        msg_send![self.0, setTitleVisibility:visibility]
    }

    pub unsafe fn setTitlebarAppearsTransparent_(&self, transparent: BOOL) {
        msg_send![self.0, setTitlebarAppearsTransparent:transparent]
    }

    pub unsafe fn representedFilename(&self) -> NSString {
        msg_send![self.0, representedFilename]
    }

    pub unsafe fn setRepresentedFilename_(&self, filePath: NSString) {
        msg_send![self.0, setRepresentedFilename:filePath]
    }

    pub unsafe fn representedURL(&self) -> NSURL {
        msg_send![self.0, representedURL]
    }

    pub unsafe fn setRepresentedURL_(&self, representedURL: NSURL) {
        msg_send![self.0, setRepresentedURL:representedURL]
    }

    // Accessing Screen Information

    pub unsafe fn screen(&self) -> NSScreen {
        msg_send![self.0, screen]
    }

    pub unsafe fn deepestScreen(&self) -> NSScreen {
        msg_send![self.0, deepestScreen]
    }

    pub unsafe fn displaysWhenScreenProfileChanges(&self) -> BOOL {
        msg_send![self.0, displaysWhenScreenProfileChanges]
    }

    pub unsafe fn setDisplaysWhenScreenProfileChanges_(&self, displaysWhenScreenProfileChanges: BOOL) {
        msg_send![self.0, setDisplaysWhenScreenProfileChanges:displaysWhenScreenProfileChanges]
    }

    // Moving Windows

    pub unsafe fn setMovableByWindowBackground_(&self, movableByWindowBackground: BOOL) {
        msg_send![self.0, setMovableByWindowBackground:movableByWindowBackground]
    }

    pub unsafe fn setMovable_(&self, movable: BOOL) {
        msg_send![self.0, setMovable:movable]
    }

    pub unsafe fn center(&self) {
        msg_send![self.0, center]
    }

    // Closing Windows

    pub unsafe fn performClose_<A>(&self, sender: id<A>) {
        msg_send![self.0, performClose:sender]
    }

    pub unsafe fn close(&self) {
        msg_send![self.0, close]
    }

    pub unsafe fn setReleasedWhenClosed_(&self, releasedWhenClosed: BOOL) {
        msg_send![self.0, setReleasedWhenClosed:releasedWhenClosed]
    }

    // Minimizing Windows

    pub unsafe fn performMiniaturize_<A>(&self, sender: id<A>) {
        msg_send![self.0, performMiniaturize:sender]
    }

    pub unsafe fn miniaturize_<A>(&self, sender: id<A>) {
        msg_send![self.0, miniaturize:sender]
    }

    pub unsafe fn deminiaturize_<A>(&self, sender: id<A>) {
        msg_send![self.0, deminiaturize:sender]
    }

    pub unsafe fn miniwindowImage(&self) -> NSImage {
        msg_send![self.0, miniwindowImage]
    }

    pub unsafe fn setMiniwindowImage_(&self, miniwindowImage: NSImage) {
        msg_send![self.0, setMiniwindowImage:miniwindowImage]
    }

    pub unsafe fn miniwindowTitle(&self) -> NSString {
        msg_send![self.0, miniwindowTitle]
    }

    pub unsafe fn setMiniwindowTitle_(&self, miniwindowTitle: NSString) {
        msg_send![self.0, setMiniwindowTitle:miniwindowTitle]
    }

    // TODO: Getting the Dock Tile
    // TODO: Printing Windows
    // TODO: Providing Services
    // TODO: Working with Carbon
    // TODO: Triggering Constraint-Based Layout
    // TODO: Debugging Constraint-Based Layout
    // TODO: Constraint-Based Layouts
}

pub enum NSViewPrototype {}
pub type NSView = id<(NSViewPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSViewPrototype, T)> {
    pub unsafe fn alloc() -> Self {
        id(msg_send![class("NSView"), alloc], PhantomData)
    }
    pub unsafe fn init(&self) -> Self {
        id(msg_send![self.0, init], PhantomData)
    }

    pub unsafe fn initWithFrame_(&self, frameRect: NSRect) -> Self {
        id(msg_send![self.0, initWithFrame:frameRect], PhantomData)
    }

    pub unsafe fn bounds(&self) -> NSRect {
        msg_send![self.0, bounds]
    }

    pub unsafe fn frame(&self) -> NSRect {
        msg_send![self.0, frame]
    }

    pub unsafe fn display_(&self) {
        msg_send![self.0, display]
    }

    pub unsafe fn setWantsBestResolutionOpenGLSurface_(&self, flag: BOOL) {
        msg_send![self.0, setWantsBestResolutionOpenGLSurface:flag]
    }

    pub unsafe fn convertPoint_fromView_(&self, point: NSPoint, view: NSView) -> NSPoint {
        msg_send![self.0, convertPoint:point fromView:view]
    }

    pub unsafe fn addSubview_(&self, view: NSView) {
        msg_send![self.0, addSubview:view]
    }

    pub unsafe fn superview(&self) -> NSView {
        msg_send![self.0, superview]
    }

    pub unsafe fn removeFromSuperview(&self) {
        msg_send![self.0, removeFromSuperview]
    }

    pub unsafe fn wantsLayer(&self) -> BOOL {
        msg_send![self.0, wantsLayer]
    }

    pub unsafe fn setWantsLayer(&self, wantsLayer: BOOL) {
        msg_send![self.0, setWantsLayer:wantsLayer]
    }

    pub unsafe fn layer<A>(&self) -> id<A> {
        id(msg_send![self.0, layer], PhantomData)
    }

    pub unsafe fn setLayer<A>(&self, layer: id<A>) {
        msg_send![self.0, setLayer:layer]
    }
}


pub enum NSOpenGLViewPrototype {}
pub type NSOpenGLView = id<(NSOpenGLViewPrototype, (NSViewPrototype, (NSObjectPrototype, ())))>;

impl<T> id<(NSOpenGLViewPrototype, T)> {
    pub unsafe fn alloc() -> Self {
        id(msg_send![class("NSOpenGLView"), alloc], PhantomData)
    }

    pub unsafe fn initWithFrame_pixelFormat_(&self,  frameRect: NSRect, format: NSOpenGLPixelFormat) -> Self {
        id(msg_send![self.0, initWithFrame:frameRect pixelFormat:format], PhantomData)
    }

    pub unsafe fn display_(&self) {
        msg_send![self.0, display]
    }

    pub unsafe fn setOpenGLContext_(&self, context: NSOpenGLContext) {
        msg_send![self.0, setOpenGLContext:context]
    }

    pub unsafe fn setPixelFormat_(&self, pixelformat: NSOpenGLPixelFormat) {
        msg_send![self.0, setPixelFormat:pixelformat]
    }

}

pub enum NSOpenGLPixelFormatPrototype {}
pub type NSOpenGLPixelFormat = id<(NSOpenGLPixelFormatPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSOpenGLPixelFormatPrototype, T)> {
    pub unsafe fn alloc() -> Self {
        id(msg_send![class("NSOpenGLPixelFormat"), alloc], PhantomData)
    }

    pub unsafe fn initWithAttributes_(&self, attributes: &[u32]) -> Self {
        id(msg_send![self.0, initWithAttributes:attributes], PhantomData)
    }

    // Managing the Pixel Format

    pub unsafe fn getValues_forAttribute_forVirtualScreen_(&self, val: *mut GLint, attrib: NSOpenGLPixelFormatAttribute, screen: GLint) {
        msg_send![self.0, getValues:val forAttribute:attrib forVirtualScreen:screen]
    }

    pub unsafe fn numberOfVirtualScreens(&self) -> GLint {
        msg_send![self.0, numberOfVirtualScreens]
    }
}

pub enum NSOpenGLContextPrototype {}
pub type NSOpenGLContext = id<(NSOpenGLContextPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSOpenGLContextPrototype, T)> {
    pub unsafe fn alloc() -> Self {
        id(msg_send![class("NSOpenGLPixelFormat"), alloc], PhantomData)
    }

    pub unsafe fn initWithFormat_shareContext_(&self, format: NSOpenGLPixelFormat, shareContext: NSOpenGLContext) -> Self {
        id(msg_send![self.0, initWithFormat:format shareContext:shareContext], PhantomData)
    }

    pub unsafe fn initWithCGLContextObj_(&self, context: CGLContextObj) -> Self {
        id(msg_send![self.0, initWithCGLContextObj:context], PhantomData)
    }

    // Managing the Current Context

    pub unsafe fn clearCurrentContext() {
        msg_send![class("NSOpenGLContext"), clearCurrentContext]
    }

    pub unsafe fn currentContext() -> Self {
        id(msg_send![class("NSOpenGLContext"), currentContext], PhantomData)
    }

    pub unsafe fn makeCurrentContext(&self) {
        msg_send![self.0, makeCurrentContext]
    }

    // Drawable Object Management

    pub unsafe fn setView_(&self, view: NSView) {
        msg_send![self.0, setView:view]
    }

    pub unsafe fn view(&self) -> NSView {
        msg_send![self.0, view]
    }

    pub unsafe fn clearDrawable(&self) {
        msg_send![self.0, clearDrawable]
    }

    pub unsafe fn update(&self) {
        msg_send![self.0, update]
    }

    // Flushing the Drawing Buffer

    pub unsafe fn flushBuffer(&self) {
        msg_send![self.0, flushBuffer]
    }

    // Context Parameter Handling

    pub unsafe fn setValues_forParameter_(&self, vals: *const GLint, param: NSOpenGLContextParameter) {
        msg_send![self.0, setValues:vals forParameter:param]
    }

    pub unsafe fn getValues_forParameter_(&self, vals: *mut GLint, param: NSOpenGLContextParameter) {
        msg_send![self.0, getValues:vals forParameter:param]
    }

    // Working with Virtual Screens

    pub unsafe fn setCurrentVirtualScreen_(&self, screen: GLint) {
        msg_send![self.0, setCurrentVirtualScreen:screen]
    }

    pub unsafe fn currentVirtualScreen(&self) -> GLint {
        msg_send![self.0, currentVirtualScreen]
    }

    // Getting the CGL Context Object

    pub unsafe fn CGLContextObj(&self) -> CGLContextObj {
        msg_send![self.0, CGLContextObj]
    }
}

bitflags! {
    flags NSEventSwipeTrackingOptions: NSUInteger {
        const NSEventSwipeTrackingLockDirection         = 0x1 << 0,
        const NSEventSwipeTrackingClampGestureAmount    = 0x1 << 1,
    }
}

#[repr(i64)] // NSInteger
pub enum NSEventGestureAxis {
    NSEventGestureAxisNone = 0,
    NSEventGestureAxisHorizontal,
    NSEventGestureAxisVertical,
}

bitflags! {
    flags NSEventPhase: NSUInteger {
       const NSEventPhaseNone        = 0,
       const NSEventPhaseBegan       = 0x1 << 0,
       const NSEventPhaseStationary  = 0x1 << 1,
       const NSEventPhaseChanged     = 0x1 << 2,
       const NSEventPhaseEnded       = 0x1 << 3,
       const NSEventPhaseCancelled   = 0x1 << 4,
       const NSEventPhaseMayBegin    = 0x1 << 5,
    }
}

bitflags! {
    flags NSTouchPhase: NSUInteger {
        const NSTouchPhaseBegan         = 1 << 0,
        const NSTouchPhaseMoved         = 1 << 1,
        const NSTouchPhaseStationary    = 1 << 2,
        const NSTouchPhaseEnded         = 1 << 3,
        const NSTouchPhaseCancelled     = 1 << 4,
        const NSTouchPhaseTouching      = NSTouchPhaseBegan.bits
                                        | NSTouchPhaseMoved.bits
                                        | NSTouchPhaseStationary.bits,
        const NSTouchPhaseAny           = !0, // NSUIntegerMax
    }
}

#[derive(Debug)]
#[repr(u64)] // NSUInteger
pub enum NSEventType {
    NSLeftMouseDown         = 1,
    NSLeftMouseUp           = 2,
    NSRightMouseDown        = 3,
    NSRightMouseUp          = 4,
    NSMouseMoved            = 5,
    NSLeftMouseDragged      = 6,
    NSRightMouseDragged     = 7,
    NSMouseEntered          = 8,
    NSMouseExited           = 9,
    NSKeyDown               = 10,
    NSKeyUp                 = 11,
    NSFlagsChanged          = 12,
    NSAppKitDefined         = 13,
    NSSystemDefined         = 14,
    NSApplicationDefined    = 15,
    NSPeriodic              = 16,
    NSCursorUpdate          = 17,
    NSScrollWheel           = 22,
    NSTabletPoint           = 23,
    NSTabletProximity       = 24,
    NSOtherMouseDown        = 25,
    NSOtherMouseUp          = 26,
    NSOtherMouseDragged     = 27,
    NSEventTypeGesture      = 29,
    NSEventTypeMagnify      = 30,
    NSEventTypeSwipe        = 31,
    NSEventTypeRotate       = 18,
    NSEventTypeBeginGesture = 19,
    NSEventTypeEndGesture   = 20,
    NSEventTypePressure     = 34,
}

bitflags! {
    flags NSEventMask: libc::c_ulonglong {
        const NSLeftMouseDownMask         = 1 << NSLeftMouseDown as libc::c_ulonglong,
        const NSLeftMouseUpMask           = 1 << NSLeftMouseUp as libc::c_ulonglong,
        const NSRightMouseDownMask        = 1 << NSRightMouseDown as libc::c_ulonglong,
        const NSRightMouseUpMask          = 1 << NSRightMouseUp as libc::c_ulonglong,
        const NSMouseMovedMask            = 1 << NSMouseMoved as libc::c_ulonglong,
        const NSLeftMouseDraggedMask      = 1 << NSLeftMouseDragged as libc::c_ulonglong,
        const NSRightMouseDraggedMask     = 1 << NSRightMouseDragged as libc::c_ulonglong,
        const NSMouseEnteredMask          = 1 << NSMouseEntered as libc::c_ulonglong,
        const NSMouseExitedMask           = 1 << NSMouseExited as libc::c_ulonglong,
        const NSKeyDownMask               = 1 << NSKeyDown as libc::c_ulonglong,
        const NSKeyUpMask                 = 1 << NSKeyUp as libc::c_ulonglong,
        const NSFlagsChangedMask          = 1 << NSFlagsChanged as libc::c_ulonglong,
        const NSAppKitDefinedMask         = 1 << NSAppKitDefined as libc::c_ulonglong,
        const NSSystemDefinedMask         = 1 << NSSystemDefined as libc::c_ulonglong,
        const NSApplicationDefinedMask    = 1 << NSApplicationDefined as libc::c_ulonglong,
        const NSPeriodicMask              = 1 << NSPeriodic as libc::c_ulonglong,
        const NSCursorUpdateMask          = 1 << NSCursorUpdate as libc::c_ulonglong,
        const NSScrollWheelMask           = 1 << NSScrollWheel as libc::c_ulonglong,
        const NSTabletPointMask           = 1 << NSTabletPoint as libc::c_ulonglong,
        const NSTabletProximityMask       = 1 << NSTabletProximity as libc::c_ulonglong,
        const NSOtherMouseDownMask        = 1 << NSOtherMouseDown as libc::c_ulonglong,
        const NSOtherMouseUpMask          = 1 << NSOtherMouseUp as libc::c_ulonglong,
        const NSOtherMouseDraggedMask     = 1 << NSOtherMouseDragged as libc::c_ulonglong,
        const NSEventMaskGesture          = 1 << NSEventTypeGesture as libc::c_ulonglong,
        const NSEventMaskSwipe            = 1 << NSEventTypeSwipe as libc::c_ulonglong,
        const NSEventMaskRotate           = 1 << NSEventTypeRotate as libc::c_ulonglong,
        const NSEventMaskBeginGesture     = 1 << NSEventTypeBeginGesture as libc::c_ulonglong,
        const NSEventMaskEndGesture       = 1 << NSEventTypeEndGesture as libc::c_ulonglong,
        const NSEventMaskPressure         = 1 << NSEventTypePressure as libc::c_ulonglong,
        const NSAnyEventMask              = 0xffffffff,
    }
}

impl NSEventMask {
    pub fn from_type(ty: NSEventType) -> NSEventMask {
        NSEventMask { bits: 1 << ty as libc::c_ulonglong }
    }
}

bitflags! {
    flags NSEventModifierFlags: NSUInteger {
        const NSAlphaShiftKeyMask                     = 1 << 16,
        const NSShiftKeyMask                          = 1 << 17,
        const NSControlKeyMask                        = 1 << 18,
        const NSAlternateKeyMask                      = 1 << 19,
        const NSCommandKeyMask                        = 1 << 20,
        const NSNumericPadKeyMask                     = 1 << 21,
        const NSHelpKeyMask                           = 1 << 22,
        const NSFunctionKeyMask                       = 1 << 23,
        const NSDeviceIndependentModifierFlagsMask    = 0xffff0000,
    }
}

// Not sure of the type here
pub enum NSPointingDeviceType {
    // TODO: Not sure what these values are
    // NSUnknownPointingDevice = NX_TABLET_POINTER_UNKNOWN,
    // NSPenPointingDevice     = NX_TABLET_POINTER_PEN,
    // NSCursorPointingDevice  = NX_TABLET_POINTER_CURSOR,
    // NSEraserPointingDevice  = NX_TABLET_POINTER_ERASER,
}

// Not sure of the type here
pub enum NSEventButtonMask {
    // TODO: Not sure what these values are
    // NSPenTipMask =       NX_TABLET_BUTTON_PENTIPMASK,
    // NSPenLowerSideMask = NX_TABLET_BUTTON_PENLOWERSIDEMASK,
    // NSPenUpperSideMask = NX_TABLET_BUTTON_PENUPPERSIDEMASK,
}

#[repr(i16)]
pub enum NSEventSubtype {
    // TODO: Not sure what these values are
    // NSMouseEventSubtype           = NX_SUBTYPE_DEFAULT,
    // NSTabletPointEventSubtype     = NX_SUBTYPE_TABLET_POINT,
    // NSTabletProximityEventSubtype = NX_SUBTYPE_TABLET_PROXIMITY
    // NSTouchEventSubtype           = NX_SUBTYPE_MOUSE_TOUCH,
    NSWindowExposedEventType = 0,
    NSApplicationActivatedEventType = 1,
    NSApplicationDeactivatedEventType = 2,
    NSWindowMovedEventType = 4,
    NSScreenChangedEventType = 8,
    NSAWTEventType = 16,
}

pub const NSUpArrowFunctionKey: libc::c_ushort = 0xF700;
pub const NSDownArrowFunctionKey: libc::c_ushort = 0xF701;
pub const NSLeftArrowFunctionKey: libc::c_ushort = 0xF702;
pub const NSRightArrowFunctionKey: libc::c_ushort = 0xF703;
pub const NSF1FunctionKey: libc::c_ushort = 0xF704;
pub const NSF2FunctionKey: libc::c_ushort = 0xF705;
pub const NSF3FunctionKey: libc::c_ushort = 0xF706;
pub const NSF4FunctionKey: libc::c_ushort = 0xF707;
pub const NSF5FunctionKey: libc::c_ushort = 0xF708;
pub const NSF6FunctionKey: libc::c_ushort = 0xF709;
pub const NSF7FunctionKey: libc::c_ushort = 0xF70A;
pub const NSF8FunctionKey: libc::c_ushort = 0xF70B;
pub const NSF9FunctionKey: libc::c_ushort = 0xF70C;
pub const NSF10FunctionKey: libc::c_ushort = 0xF70D;
pub const NSF11FunctionKey: libc::c_ushort = 0xF70E;
pub const NSF12FunctionKey: libc::c_ushort = 0xF70F;
pub const NSF13FunctionKey: libc::c_ushort = 0xF710;
pub const NSF14FunctionKey: libc::c_ushort = 0xF711;
pub const NSF15FunctionKey: libc::c_ushort = 0xF712;
pub const NSF16FunctionKey: libc::c_ushort = 0xF713;
pub const NSF17FunctionKey: libc::c_ushort = 0xF714;
pub const NSF18FunctionKey: libc::c_ushort = 0xF715;
pub const NSF19FunctionKey: libc::c_ushort = 0xF716;
pub const NSF20FunctionKey: libc::c_ushort = 0xF717;
pub const NSF21FunctionKey: libc::c_ushort = 0xF718;
pub const NSF22FunctionKey: libc::c_ushort = 0xF719;
pub const NSF23FunctionKey: libc::c_ushort = 0xF71A;
pub const NSF24FunctionKey: libc::c_ushort = 0xF71B;
pub const NSF25FunctionKey: libc::c_ushort = 0xF71C;
pub const NSF26FunctionKey: libc::c_ushort = 0xF71D;
pub const NSF27FunctionKey: libc::c_ushort = 0xF71E;
pub const NSF28FunctionKey: libc::c_ushort = 0xF71F;
pub const NSF29FunctionKey: libc::c_ushort = 0xF720;
pub const NSF30FunctionKey: libc::c_ushort = 0xF721;
pub const NSF31FunctionKey: libc::c_ushort = 0xF722;
pub const NSF32FunctionKey: libc::c_ushort = 0xF723;
pub const NSF33FunctionKey: libc::c_ushort = 0xF724;
pub const NSF34FunctionKey: libc::c_ushort = 0xF725;
pub const NSF35FunctionKey: libc::c_ushort = 0xF726;
pub const NSInsertFunctionKey: libc::c_ushort = 0xF727;
pub const NSDeleteFunctionKey: libc::c_ushort = 0xF728;
pub const NSHomeFunctionKey: libc::c_ushort = 0xF729;
pub const NSBeginFunctionKey: libc::c_ushort = 0xF72A;
pub const NSEndFunctionKey: libc::c_ushort = 0xF72B;
pub const NSPageUpFunctionKey: libc::c_ushort = 0xF72C;
pub const NSPageDownFunctionKey: libc::c_ushort = 0xF72D;
pub const NSPrintScreenFunctionKey: libc::c_ushort = 0xF72E;
pub const NSScrollLockFunctionKey: libc::c_ushort = 0xF72F;
pub const NSPauseFunctionKey: libc::c_ushort = 0xF730;
pub const NSSysReqFunctionKey: libc::c_ushort = 0xF731;
pub const NSBreakFunctionKey: libc::c_ushort = 0xF732;
pub const NSResetFunctionKey: libc::c_ushort = 0xF733;
pub const NSStopFunctionKey: libc::c_ushort = 0xF734;
pub const NSMenuFunctionKey: libc::c_ushort = 0xF735;
pub const NSUserFunctionKey: libc::c_ushort = 0xF736;
pub const NSSystemFunctionKey: libc::c_ushort = 0xF737;
pub const NSPrintFunctionKey: libc::c_ushort = 0xF738;
pub const NSClearLineFunctionKey: libc::c_ushort = 0xF739;
pub const NSClearDisplayFunctionKey: libc::c_ushort = 0xF73A;
pub const NSInsertLineFunctionKey: libc::c_ushort = 0xF73B;
pub const NSDeleteLineFunctionKey: libc::c_ushort = 0xF73C;
pub const NSInsertCharFunctionKey: libc::c_ushort = 0xF73D;
pub const NSDeleteCharFunctionKey: libc::c_ushort = 0xF73E;
pub const NSPrevFunctionKey: libc::c_ushort = 0xF73F;
pub const NSNextFunctionKey: libc::c_ushort = 0xF740;
pub const NSSelectFunctionKey: libc::c_ushort = 0xF741;
pub const NSExecuteFunctionKey: libc::c_ushort = 0xF742;
pub const NSUndoFunctionKey: libc::c_ushort = 0xF743;
pub const NSRedoFunctionKey: libc::c_ushort = 0xF744;
pub const NSFindFunctionKey: libc::c_ushort = 0xF745;
pub const NSHelpFunctionKey: libc::c_ushort = 0xF746;
pub const NSModeSwitchFunctionKey: libc::c_ushort = 0xF747;

pub enum NSEventPrototype {}
pub type NSEvent = id<(NSEventPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSEventPrototype, T)> {
    // Creating Events

    pub unsafe fn keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode_(
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: NSGraphicsContext,
        characters: NSString,
        unmodCharacters: NSString,
        repeatKey: BOOL,
        code: libc::c_ushort) -> Self
    {
        id(msg_send![class("NSEvent"), keyEventWithType:eventType
                                              location:location
                                         modifierFlags:modifierFlags
                                             timestamp:timestamp
                                          windowNumber:windowNumber
                                               context:context
                                            characters:characters
                           charactersIgnoringModifiers:unmodCharacters
                                             isARepeat:repeatKey
                                               keyCode:code], PhantomData)
    }

    pub unsafe fn mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure_(
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: NSGraphicsContext,
        eventNumber: NSInteger,
        clickCount: NSInteger,
        pressure: libc::c_float) -> Self
    {
        id(msg_send![class("NSEvent"), mouseEventWithType:eventType
                                                location:location
                                           modifierFlags:modifierFlags
                                               timestamp:timestamp
                                            windowNumber:windowNumber
                                                 context:context
                                             eventNumber:eventNumber
                                              clickCount:clickCount
                                                pressure:pressure], PhantomData)
    }

    pub unsafe fn enterExitEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_trackingNumber_userData_(
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: NSGraphicsContext,
        eventNumber: NSInteger,
        trackingNumber: NSInteger,
        userData: *mut libc::c_void) -> Self
    {
        id(msg_send![class("NSEvent"), enterExitEventWithType:eventType
                                                    location:location
                                               modifierFlags:modifierFlags
                                                   timestamp:timestamp
                                                windowNumber:windowNumber
                                                     context:context
                                                 eventNumber:eventNumber
                                              trackingNumber:trackingNumber
                                                    userData:userData], PhantomData)
    }

    pub unsafe fn otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: NSGraphicsContext,
        subtype: NSEventSubtype,
        data1: NSInteger,
        data2: NSInteger) -> Self
    {
        id(msg_send![class("NSEvent"), otherEventWithType:eventType
                                                 location:location
                                            modifierFlags:modifierFlags
                                                timestamp:timestamp
                                             windowNumber:windowNumber
                                                  context:context
                                                  subtype:subtype
                                                    data1:data1
                                                    data2:data2], PhantomData)
    }

    pub unsafe fn eventWithEventRef_(eventRef: *const libc::c_void) -> Self {
        id(msg_send![class("NSEvent"), eventWithEventRef:eventRef], PhantomData)
    }

    pub unsafe fn eventWithCGEvent_(cgEvent: *mut libc::c_void /* CGEventRef */) -> Self {
        id(msg_send![class("NSEvent"), eventWithCGEvent:cgEvent], PhantomData)
    }

    // Getting General Event Information

    pub unsafe fn context(&self) -> NSGraphicsContext {
        msg_send![self.0, context]
    }

    pub unsafe fn locationInWindow(&self) -> NSPoint {
        msg_send![self.0, locationInWindow]
    }

    pub unsafe fn modifierFlags(&self) -> NSEventModifierFlags {
        msg_send![self.0, modifierFlags]
    }

    pub unsafe fn timestamp(&self) -> NSTimeInterval {
        msg_send![self.0, timestamp]
    }
    // NOTE: renamed from `- type` due to Rust keyword collision

    pub unsafe fn eventType(&self) -> NSEventType {
        msg_send![self.0, type]
    }

    pub unsafe fn window(&self) -> NSWindow {
        msg_send![self.0, window]
    }

    pub unsafe fn windowNumber(&self) -> NSInteger {
        msg_send![self.0, windowNumber]
    }

    pub unsafe fn eventRef(&self) -> *const libc::c_void {
        msg_send![self.0, eventRef]
    }

    pub unsafe fn CGEvent(&self) -> *mut libc::c_void /* CGEventRef */ {
        msg_send![self.0, CGEvent]
    }

    // Getting Key Event Information

    // NOTE: renamed from `+ modifierFlags` due to conflict with `- modifierFlags`

    pub unsafe fn currentModifierFlags() -> NSEventModifierFlags {
        msg_send![class("NSEvent"), currentModifierFlags]
    }

    pub unsafe fn keyRepeatDelay() -> NSTimeInterval {
        msg_send![class("NSEvent"), keyRepeatDelay]
    }

    pub unsafe fn keyRepeatInterval() -> NSTimeInterval {
        msg_send![class("NSEvent"), keyRepeatInterval]
    }

    pub unsafe fn characters(&self) -> NSString {
        msg_send![self.0, characters]
    }

    pub unsafe fn charactersIgnoringModifiers(&self) -> NSString {
        msg_send![self.0, charactersIgnoringModifiers]
    }

    pub unsafe fn keyCode(&self) -> libc::c_ushort {
        msg_send![self.0, keyCode]
    }

    // Getting Mouse Event Information

    pub unsafe fn pressedMouseButtons() -> NSUInteger {
        msg_send![class("NSEvent"), pressedMouseButtons]
    }

    pub unsafe fn doubleClickInterval() -> NSTimeInterval {
        msg_send![class("NSEvent"), doubleClickInterval]
    }

    pub unsafe fn mouseLocation() -> NSPoint {
        msg_send![class("NSEvent"), mouseLocation]
    }

    pub unsafe fn buttonNumber(&self) -> NSInteger {
        msg_send![self.0, buttonNumber]
    }

    pub unsafe fn clickCount(&self) -> NSInteger {
        msg_send![self.0, clickCount]
    }

    pub unsafe fn pressure(&self) -> libc::c_float {
        msg_send![self.0, pressure]
    }

    pub unsafe fn stage(&self) -> NSInteger{
        msg_send![self.0, stage]
    }

    pub unsafe fn setMouseCoalescingEnabled_(flag: BOOL) {
        msg_send![class("NSEvent"), setMouseCoalescingEnabled:flag]
    }

    pub unsafe fn isMouseCoalescingEnabled() -> BOOL {
        msg_send![class("NSEvent"), isMouseCoalescingEnabled]
    }

    // Getting Mouse-Tracking Event Information

    pub unsafe fn eventNumber(&self) -> NSInteger {
        msg_send![self.0, eventNumber]
    }

    pub unsafe fn trackingNumber(&self) -> NSInteger {
        msg_send![self.0, trackingNumber]
    }

    pub unsafe fn trackingArea(&self) -> NSTrackingArea {
        msg_send![self.0, trackingArea]
    }

    pub unsafe fn userData(&self) -> *const libc::c_void {
        msg_send![self.0, userData]
    }

    // Getting Custom Event Information

    pub unsafe fn data1(&self) -> NSInteger {
        msg_send![self.0, data1]
    }

    pub unsafe fn data2(&self) -> NSInteger {
        msg_send![self.0, data2]
    }

    pub unsafe fn subtype(&self) -> NSEventSubtype {
        msg_send![self.0, subtype]
    }

    // Getting Scroll Wheel Event Information

    pub unsafe fn deltaX(&self) -> CGFloat {
        msg_send![self.0, deltaX]
    }

    pub unsafe fn deltaY(&self) -> CGFloat {
        msg_send![self.0, deltaY]
    }

    pub unsafe fn deltaZ(&self) -> CGFloat {
        msg_send![self.0, deltaZ]
    }

    // Getting Tablet Proximity Information

    pub unsafe fn capabilityMask(&self) -> NSUInteger {
        msg_send![self.0, capabilityMask]
    }

    pub unsafe fn deviceID(&self) -> NSUInteger {
        msg_send![self.0, deviceID]
    }

    pub unsafe fn pointingDeviceID(&self) -> NSUInteger {
        msg_send![self.0, pointingDeviceID]
    }

    pub unsafe fn pointingDeviceSerialNumber(&self) -> NSUInteger {
        msg_send![self.0, pointingDeviceSerialNumber]
    }

    pub unsafe fn pointingDeviceType(&self) -> NSPointingDeviceType {
        msg_send![self.0, pointingDeviceType]
    }

    pub unsafe fn systemTabletID(&self) -> NSUInteger {
        msg_send![self.0, systemTabletID]
    }

    pub unsafe fn tabletID(&self) -> NSUInteger {
        msg_send![self.0, tabletID]
    }

    pub unsafe fn uniqueID(&self) -> libc::c_ulonglong {
        msg_send![self.0, uniqueID]
    }

    pub unsafe fn vendorID(&self) -> NSUInteger {
        msg_send![self.0, vendorID]
    }

    pub unsafe fn vendorPointingDeviceType(&self) -> NSUInteger {
        msg_send![self.0, vendorPointingDeviceType]
    }

    // Getting Tablet Pointing Information

    pub unsafe fn absoluteX(&self) -> NSInteger {
        msg_send![self.0, absoluteX]
    }

    pub unsafe fn absoluteY(&self) -> NSInteger {
        msg_send![self.0, absoluteY]
    }

    pub unsafe fn absoluteZ(&self) -> NSInteger {
        msg_send![self.0, absoluteZ]
    }

    pub unsafe fn buttonMask(&self) -> NSEventButtonMask {
        msg_send![self.0, buttonMask]
    }

    pub unsafe fn rotation(&self) -> libc::c_float {
        msg_send![self.0, rotation]
    }

    pub unsafe fn tangentialPressure(&self) -> libc::c_float {
        msg_send![self.0, tangentialPressure]
    }

    pub unsafe fn tilt(&self) -> NSPoint {
        msg_send![self.0, tilt]
    }

    pub unsafe fn vendorDefined<A>(&self) -> id<A> {
        id(msg_send![self.0, vendorDefined], PhantomData)
    }

    // Requesting and Stopping Periodic Events

    pub unsafe fn startPeriodicEventsAfterDelay_withPeriod_(delaySeconds: NSTimeInterval, periodSeconds: NSTimeInterval) {
        msg_send![class("NSEvent"), startPeriodicEventsAfterDelay:delaySeconds withPeriod:periodSeconds]
    }

    pub unsafe fn stopPeriodicEvents() {
        msg_send![class("NSEvent"), stopPeriodicEvents]
    }

    // Getting Touch and Gesture Information

    pub unsafe fn magnification(&self) -> CGFloat {
        msg_send![self.0, magnification]
    }

    pub unsafe fn touchesMatchingPhase_inView_(&self, phase: NSTouchPhase, view: NSView) -> NSSet<NSTouch> {
        msg_send![self.0, touchesMatchingPhase:phase inView:view]
    }

    pub unsafe fn isSwipeTrackingFromScrollEventsEnabled() -> BOOL {
        msg_send![class("NSEvent"), isSwipeTrackingFromScrollEventsEnabled]
    }

    // Monitoring Application Events

    // TODO: addGlobalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)
    // TODO: addLocalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)

    pub unsafe fn removeMonitor_<A>(eventMonitor: id<A>) {
        msg_send![class("NSEvent"), removeMonitor:eventMonitor]
    }

    // Scroll Wheel and Flick Events

    pub unsafe fn hasPreciseScrollingDeltas(&self) -> BOOL {
        msg_send![self.0, hasPreciseScrollingDeltas]
    }

    pub unsafe fn scrollingDeltaX(&self) -> CGFloat {
        msg_send![self.0, scrollingDeltaX]
    }

    pub unsafe fn scrollingDeltaY(&self) -> CGFloat {
        msg_send![self.0, scrollingDeltaY]
    }

    pub unsafe fn momentumPhase(&self) -> NSEventPhase {
        msg_send![self.0, momentumPhase]
    }

    pub unsafe fn phase(&self) -> NSEventPhase {
        msg_send![self.0, phase]
    }

    // TODO: trackSwipeEventWithOptions_dampenAmountThresholdMin_max_usingHandler_ (unsure how to bind to blocks)
}

pub enum NSScreenPrototype {}
pub type NSScreen = id<(NSScreenPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSScreenPrototype, T)> {
    // Getting NSScreen Objects

    pub unsafe fn mainScreen() -> Self {
        id(msg_send![class("NSScreen"), mainScreen], PhantomData)
    }

    pub unsafe fn deepestScreen() -> Self {
        id(msg_send![class("NSScreen"), deepestScreen], PhantomData)
    }

    pub unsafe fn screens() -> NSArray<Self> {
        id(msg_send![class("NSScreen"), screens], PhantomData)
    }

    // Getting Screen Information

    pub unsafe fn depth(&self) -> NSWindowDepth {
        msg_send![self.0, depth]
    }

    pub unsafe fn frame(&self) -> NSRect {
        msg_send![self.0, frame]
    }

    pub unsafe fn supportedWindowDepths(&self) -> *const NSWindowDepth {
        msg_send![self.0, supportedWindowDepths]
    }

    pub unsafe fn deviceDescription(&self) -> NSDictionary<NSString, id> {
        msg_send![self.0, deviceDescription]
    }

    pub unsafe fn visibleFrame(&self) -> NSRect {
        msg_send![self.0, visibleFrame]
    }

    pub unsafe fn colorSpace(&self) -> NSColorSpace {
        msg_send![self.0, colorSpace]
    }

    pub unsafe fn screensHaveSeparateSpaces() -> BOOL {
        msg_send![class("NSScreen"), screensHaveSeparateSpaces]
    }

    // Screen Backing Coordinate Conversion

    pub unsafe fn backingAlignedRect_options_(&self, aRect: NSRect, options: NSAlignmentOptions) -> NSRect {
        msg_send![self.0, backingAlignedRect:aRect options:options]
    }

    pub unsafe fn backingScaleFactor(&self) -> CGFloat {
        msg_send![self.0, backingScaleFactor]
    }

    pub unsafe fn convertRectFromBacking_(&self, aRect: NSRect) -> NSRect {
        msg_send![self.0, convertRectFromBacking:aRect]
    }

    pub unsafe fn convertRectToBacking_(&self, aRect: NSRect) -> NSRect {
        msg_send![self.0, convertRectToBacking:aRect]
    }
}

pub enum NSButtonPrototype {}
pub type NSButton = id<(NSButtonPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSButtonPrototype, T)> {
    pub unsafe fn setImage_(&self, img: NSImage) {
        msg_send![self.0, setImage:img]
    }
}

pub enum NSImagePrototype {}
pub type NSImage = id<(NSImagePrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSImagePrototype, T)> {
    pub unsafe fn alloc() -> Self {
        id(msg_send![class("NSImage"), alloc], PhantomData)
    }

    pub unsafe fn initByReferencingFile_(&self, file_name: NSString) -> Self {
        id(msg_send![self.0, initByReferencingFile:file_name], PhantomData)
    }

    pub unsafe fn initWithContentsOfFile_(&self, file_name: NSString) -> Self {
        id(msg_send![self.0, initWithContentsOfFile:file_name], PhantomData)
    }

    pub unsafe fn name(&self) -> NSString {
        msg_send![self.0, name]
    }

    pub unsafe fn setName_(&self, name: NSString) -> BOOL {
        msg_send![self.0, setName:name]
    }
}

pub const NSVariableStatusItemLength: CGFloat = -1.0;
pub const NSSquareStatusItemLength: CGFloat = -2.0;

pub enum NSStatusItemPrototype {}
pub type NSStatusItem = id<(NSStatusItemPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSStatusItemPrototype, T)> {
    pub unsafe fn statusBar(&self) -> NSStatusBar {
        msg_send![self.0, statusBar]
    }

    pub unsafe fn button(&self) -> NSStatusBarButton {
        msg_send![self.0, button]
    }

    pub unsafe fn menu(&self) -> NSMenu {
        msg_send![self.0, menu]
    }

    pub unsafe fn setMenu_(&self, menu: NSMenu) {
        msg_send![self.0, setMenu:menu]
    }

    pub unsafe fn length(&self) -> CGFloat {
        msg_send![self.0, length]
    }

    pub unsafe fn setLength_(&self, length: CGFloat) {
        msg_send![self.0, setLength: length]
    }
}

pub enum NSStatusBarPrototype {}
pub type NSStatusBar = id<(NSStatusBarPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSStatusBarPrototype, T)> {
    pub unsafe fn systemStatusBar() -> Self {
        id(msg_send![class("NSStatusBar"), systemStatusBar], PhantomData)
    }

    pub unsafe fn statusItemWithLength_(&self, length: CGFloat) -> NSStatusItem {
        msg_send![self.0, statusItemWithLength:length]
    }

    pub unsafe fn removeStatusItem_(&self, item: NSStatusItem) {
        msg_send![self.0, removeStatusItem:item]
    }

    pub unsafe fn isVertical(&self) -> BOOL {
        msg_send![self.0, isVertical]
    }
}

pub enum NSColorPrototype {}
pub type NSColor = id<(NSColorPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSColorPrototype, T)> {
    pub unsafe fn colorWithCalibratedRed_green_blue_alpha(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> Self {
        id(msg_send![class("NSColor"), colorWithCalibratedRed:red
                                                        green:green
                                                         blue:blue
                                                        alpha:alpha], PhantomData)
    }
}

pub enum NSColorSpacePrototype {}
pub type NSColorSpace = id<(NSColorSpacePrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSColorSpacePrototype, T)> {
    pub unsafe fn deviceRGBColorSpace() -> Self {
        id(msg_send![class("NSColorSpace"), deviceRGBColorSpace], PhantomData)
    }

    pub unsafe fn genericRGBColorSpace() -> Self {
        id(msg_send![class("NSColorSpace"), genericRGBColorSpace], PhantomData)
    }

    pub unsafe fn sRGBColorSpace() -> Self {
        id(msg_send![class("NSColorSpace"), sRGBColorSpace], PhantomData)
    }
}

pub enum NSGraphicsContextPrototype {}
pub type NSGraphicsContext = id<(NSGraphicsContextPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSGraphicsContextPrototype, T)> {
    pub unsafe fn currentContext() -> Self {
        id(msg_send![class("NSGraphicsContext"), currentContext], PhantomData)
    }
}

pub enum NSDrawerPrototype {}
pub type NSDrawer = id<(NSDrawerPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSDrawerPrototype, T)> {
    pub unsafe fn initWithContentSize_preferredEdge(&self, contentSize: NSSize, edge: NSRectEdge) -> Self {
        id(msg_send![self.0, initWithContentSize:contentSize
                                 preferredEdge:edge], PhantomData)
    }
}

pub enum NSStatusBarButtonPrototype {}
pub type NSStatusBarButton = id<(NSStatusBarButtonPrototype, (NSViewPrototype, (NSObjectPrototype, ())))>;

impl<T> id<(NSStatusBarButtonPrototype, T)> {
}

pub enum NSURLPrototype {}
pub type NSURL = id<(NSURLPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSURLPrototype, T)> {
    pub unsafe fn URLWithString(URLString: NSString) -> Self {
        id(msg_send![class("NSURL"), URLWithString:URLString], PhantomData)
    }
}

pub enum NSTrackingAreaPrototype {}
pub type NSTrackingArea = id<(NSTrackingAreaPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSTrackingAreaPrototype, T)> {
}

pub enum NSTouchPrototype {}
pub type NSTouch = id<(NSTouchPrototype, (NSObjectPrototype, ()))>;

impl<T> id<(NSTouchPrototype, T)> {
}

