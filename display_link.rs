// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Display links, useful for vertical synchronization.

#![allow(non_snake_case)]

use CVReturn;
use kCVReturnSuccess;
use time::CVTimeStamp;

use core_foundation::base::{CFTypeID, CFTypeRef, TCFType};
use libc::c_void;
use std::mem;
use std::ptr;

#[repr(C)]
struct __CVDisplayLink;

/// Matches the definition of `CGLContextObj` in `rust-opengles`.
pub type CGLContextObj = *mut c_void;

/// Matches the definition of `CGLPixelFormatObj` in `rust-opengles`.
pub type CGLPixelFormatObj = *mut c_void;

pub type CVDisplayLinkRef = *const __CVDisplayLink;

pub struct CVDisplayLink {
    obj: CVDisplayLinkRef,
}

impl Drop for CVDisplayLink {
    fn drop(&mut self) {
        unsafe {
            CVDisplayLinkRelease(self.obj)
        }
    }
}

impl Clone for CVDisplayLink {
    #[inline]
    fn clone(&self) -> CVDisplayLink {
        unsafe {
            TCFType::wrap_under_get_rule(self.obj)
        }
    }
}

impl TCFType<CVDisplayLinkRef> for CVDisplayLink {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CVDisplayLinkRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CVDisplayLinkRef) -> CVDisplayLink {
        assert!(reference != ptr::null());
        TCFType::wrap_under_create_rule(CVDisplayLinkRetain(reference))
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    #[inline]
    unsafe fn wrap_under_create_rule(reference: CVDisplayLinkRef) -> CVDisplayLink {
        assert!(reference != ptr::null());
        CVDisplayLink {
            obj: reference,
        }
    }

    #[inline]
    fn type_id(_: Option<CVDisplayLink>) -> CFTypeID {
        unsafe {
            CVDisplayLinkGetTypeID()
        }
    }
}

impl CVDisplayLink {
    pub fn create_with_active_cg_displays() -> CVDisplayLink {
        let mut reference = ptr::null();
        unsafe {
            CVDisplayLinkCreateWithActiveCGDisplays(&mut reference);
            TCFType::wrap_under_create_rule(reference)
        }
    }

    pub fn set_current_cg_display_from_opengl_context(&self,
                                                      cgl_context: CGLContextObj,
                                                      cgl_pixel_format: CGLPixelFormatObj)
                                                      -> CVReturn {
        unsafe {
            CVDisplayLinkSetCurrentCGDisplayFromOpenGLContext(self.obj,
                                                              cgl_context,
                                                              cgl_pixel_format)
        }
    }

    pub fn set_output_callback(&self, callback: Box<CVDisplayLinkRustOutputCallback>) -> CVReturn {
        unsafe {
            CVDisplayLinkSetOutputCallback(self.obj,
                                           rust_output_callback,
                                           mem::transmute::<_,*mut c_void>(box callback))
        }
    }

    pub fn start(&self) -> CVReturn {
        unsafe {
            CVDisplayLinkStart(self.obj)
        }
    }

    pub fn stop(&self) -> CVReturn {
        unsafe {
            CVDisplayLinkStop(self.obj)
        }
    }
}

pub trait CVDisplayLinkRustOutputCallback {
    fn call(&mut self, display_link: CVDisplayLink, now: &CVTimeStamp, output_time: &CVTimeStamp);
}

type CVDisplayLinkOutputCallback = extern "C" fn(displayLink: CVDisplayLinkRef,
                                                 inNow: &CVTimeStamp,
                                                 inOutputTime: &CVTimeStamp,
                                                 flagsIn: CVOptionFlags,
                                                 flagsOut: &mut CVOptionFlags,
                                                 displayLinkContext: *mut c_void)
                                                 -> CVReturn;

extern "C" fn rust_output_callback(displayLink: CVDisplayLinkRef,
                                   inNow: &CVTimeStamp,
                                   inOutputTime: &CVTimeStamp,
                                   _: CVOptionFlags,
                                   _: &mut CVOptionFlags,
                                   displayLinkContext: *mut c_void)
                                   -> CVReturn {
    unsafe {
        let mut user_procedure =
            mem::transmute::<_,Box<Box<CVDisplayLinkRustOutputCallback>>>(displayLinkContext);
        let display_link = TCFType::wrap_under_get_rule(displayLink);
        (*user_procedure).call(display_link, inNow, inOutputTime);
        kCVReturnSuccess
    }
}

pub type CVOptionFlags = u64;

#[link(name="CoreVideo", kind="framework")]
extern "C" {
    fn CVDisplayLinkGetTypeID() -> CFTypeID;
    fn CVDisplayLinkRetain(displayLink: CVDisplayLinkRef) -> CVDisplayLinkRef;
    fn CVDisplayLinkRelease(displayLink: CVDisplayLinkRef);
    fn CVDisplayLinkCreateWithActiveCGDisplays(displayLinkOut: *mut CVDisplayLinkRef);
    fn CVDisplayLinkSetCurrentCGDisplayFromOpenGLContext(displayLink: CVDisplayLinkRef,
                                                         cglContext: CGLContextObj,
                                                         cglPixelFormat: CGLPixelFormatObj)
                                                         -> CVReturn;
    fn CVDisplayLinkSetOutputCallback(displayLink: CVDisplayLinkRef,
                                      callback: CVDisplayLinkOutputCallback,
                                      userInfo: *mut c_void)
                                      -> CVReturn;
    fn CVDisplayLinkStart(displayLink: CVDisplayLinkRef) -> CVReturn;
    fn CVDisplayLinkStop(displayLink: CVDisplayLinkRef) -> CVReturn;
}

