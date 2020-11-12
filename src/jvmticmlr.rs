#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::jni::*;

pub const JVMTI_CMLR_MAJOR_VERSION_1: ::std::os::raw::c_uint = 1;
pub const JVMTI_CMLR_MINOR_VERSION_0: ::std::os::raw::c_uint = 0;
pub const JVMTI_CMLR_MAJOR_VERSION: ::std::os::raw::c_uint = 1;
pub const JVMTI_CMLR_MINOR_VERSION: ::std::os::raw::c_uint = 0;
pub const jvmtiCMLRKind_JVMTI_CMLR_DUMMY: jvmtiCMLRKind = 1;
pub const jvmtiCMLRKind_JVMTI_CMLR_INLINE_INFO: jvmtiCMLRKind = 2;
pub type jvmtiCMLRKind = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiCompiledMethodLoadRecordHeader {
    pub kind: jvmtiCMLRKind,
    pub majorinfoversion: jint,
    pub minorinfoversion: jint,
    pub next: *mut _jvmtiCompiledMethodLoadRecordHeader,
}
pub type jvmtiCompiledMethodLoadRecordHeader = _jvmtiCompiledMethodLoadRecordHeader;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PCStackInfo {
    pub pc: *mut ::std::os::raw::c_void,
    pub numstackframes: jint,
    pub methods: *mut jmethodID,
    pub bcis: *mut jint,
}
pub type PCStackInfo = _PCStackInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiCompiledMethodLoadInlineRecord {
    pub header: jvmtiCompiledMethodLoadRecordHeader,
    pub numpcs: jint,
    pub pcinfo: *mut PCStackInfo,
}
pub type jvmtiCompiledMethodLoadInlineRecord = _jvmtiCompiledMethodLoadInlineRecord;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _jvmtiCompiledMethodLoadDummyRecord {
    pub header: jvmtiCompiledMethodLoadRecordHeader,
    pub message: [::std::os::raw::c_char; 50usize],
}
pub type jvmtiCompiledMethodLoadDummyRecord = _jvmtiCompiledMethodLoadDummyRecord;
