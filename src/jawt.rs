#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::jni::*;

pub const JAWT_LOCK_ERROR: u32 = 1;
pub const JAWT_LOCK_CLIP_CHANGED: u32 = 2;
pub const JAWT_LOCK_BOUNDS_CHANGED: u32 = 4;
pub const JAWT_LOCK_SURFACE_CHANGED: u32 = 8;
pub const JAWT_VERSION_1_3: u32 = 65539;
pub const JAWT_VERSION_1_4: u32 = 65540;
pub const JAWT_VERSION_1_7: u32 = 65543;
pub const JAWT_VERSION_9: u32 = 589824;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jawt_Rectangle {
    pub x: jint,
    pub y: jint,
    pub width: jint,
    pub height: jint,
}
pub type JAWT_Rectangle = jawt_Rectangle;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jawt_DrawingSurfaceInfo {
    pub platformInfo: *mut ::std::os::raw::c_void,
    pub ds: *mut jawt_DrawingSurface,
    pub bounds: JAWT_Rectangle,
    pub clipSize: jint,
    pub clip: *mut JAWT_Rectangle,
}
pub type JAWT_DrawingSurfaceInfo = jawt_DrawingSurfaceInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jawt_DrawingSurface {
    pub env: *mut JNIEnv,
    pub target: jobject,
    pub Lock: ::std::option::Option<unsafe extern "C" fn(ds: *mut jawt_DrawingSurface) -> jint>,
    pub GetDrawingSurfaceInfo: ::std::option::Option<
        unsafe extern "C" fn(ds: *mut jawt_DrawingSurface) -> *mut JAWT_DrawingSurfaceInfo,
    >,
    pub FreeDrawingSurfaceInfo:
        ::std::option::Option<unsafe extern "C" fn(dsi: *mut JAWT_DrawingSurfaceInfo)>,
    pub Unlock: ::std::option::Option<unsafe extern "C" fn(ds: *mut jawt_DrawingSurface)>,
}
pub type JAWT_DrawingSurface = jawt_DrawingSurface;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jawt {
    pub version: jint,
    pub GetDrawingSurface: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, target: jobject) -> *mut JAWT_DrawingSurface,
    >,
    pub FreeDrawingSurface:
        ::std::option::Option<unsafe extern "C" fn(ds: *mut JAWT_DrawingSurface)>,
    pub Lock: ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv)>,
    pub Unlock: ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv)>,
    pub GetComponent: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            platformInfo: *mut ::std::os::raw::c_void,
        ) -> jobject,
    >,
    #[doc = " Since 9"]
    #[doc = " Creates a java.awt.Frame placed in a native container. Container is"]
    #[doc = " referenced by the native platform handle. For example on Windows this"]
    #[doc = " corresponds to an HWND. For other platforms, see the appropriate"]
    #[doc = " machine-dependent header file for a description. The reference returned"]
    #[doc = " by this function is a local reference that is only valid in this"]
    #[doc = " environment. This function returns a NULL reference if no frame could be"]
    #[doc = " created with matching platform information."]
    pub CreateEmbeddedFrame: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            platformInfo: *mut ::std::os::raw::c_void,
        ) -> jobject,
    >,
    #[doc = " Since 9"]
    #[doc = " Moves and resizes the embedded frame. The new location of the top-left"]
    #[doc = " corner is specified by x and y parameters relative to the native parent"]
    #[doc = " component. The new size is specified by width and height."]
    #[doc = ""]
    #[doc = " The embedded frame should be created by CreateEmbeddedFrame() method, or"]
    #[doc = " this function will not have any effect."]
    #[doc = ""]
    #[doc = " java.awt.Component.setLocation() and java.awt.Component.setBounds() for"]
    #[doc = " EmbeddedFrame really don't move it within the native parent. These"]
    #[doc = " methods always locate the embedded frame at (0, 0) for backward"]
    #[doc = " compatibility. To allow moving embedded frames this method was"]
    #[doc = " introduced, and it works just the same way as setLocation() and"]
    #[doc = " setBounds() for usual, non-embedded components."]
    #[doc = ""]
    #[doc = " Using usual get/setLocation() and get/setBounds() together with this new"]
    #[doc = " method is not recommended."]
    pub SetBounds: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            embeddedFrame: jobject,
            x: jint,
            y: jint,
            w: jint,
            h: jint,
        ),
    >,
    #[doc = " Since 9"]
    #[doc = " Synthesize a native message to activate or deactivate an EmbeddedFrame"]
    #[doc = " window depending on the value of parameter doActivate, if \"true\""]
    #[doc = " activates the window; otherwise, deactivates the window."]
    #[doc = ""]
    #[doc = " The embedded frame should be created by CreateEmbeddedFrame() method, or"]
    #[doc = " this function will not have any effect."]
    pub SynthesizeWindowActivation: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, embeddedFrame: jobject, doActivate: jboolean),
    >,
}
pub type JAWT = jawt;
extern "C" {
    pub fn JAWT_GetAWT(env: *mut JNIEnv, awt: *mut JAWT) -> jboolean;
}
