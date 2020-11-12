#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::bitfield::__BindgenBitfieldUnit;
use crate::jni::*;

pub const JDWPTRANSPORT_VERSION_1_0: ::std::os::raw::c_uint = 65536;
pub const JDWPTRANSPORT_VERSION_1_1: ::std::os::raw::c_uint = 65537;
pub type jdwpTransportEnv = *const jdwpTransportNativeInterface_;
pub const jdwpTransportError_JDWPTRANSPORT_ERROR_NONE: jdwpTransportError = 0;
pub const jdwpTransportError_JDWPTRANSPORT_ERROR_ILLEGAL_ARGUMENT: jdwpTransportError = 103;
pub const jdwpTransportError_JDWPTRANSPORT_ERROR_OUT_OF_MEMORY: jdwpTransportError = 110;
pub const jdwpTransportError_JDWPTRANSPORT_ERROR_INTERNAL: jdwpTransportError = 113;
pub const jdwpTransportError_JDWPTRANSPORT_ERROR_ILLEGAL_STATE: jdwpTransportError = 201;
pub const jdwpTransportError_JDWPTRANSPORT_ERROR_IO_ERROR: jdwpTransportError = 202;
pub const jdwpTransportError_JDWPTRANSPORT_ERROR_TIMEOUT: jdwpTransportError = 203;
pub const jdwpTransportError_JDWPTRANSPORT_ERROR_MSG_NOT_AVAILABLE: jdwpTransportError = 204;
pub type jdwpTransportError = ::std::os::raw::c_uint;
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct JDWPTransportCapabilities {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
    pub __bindgen_padding_0: u16,
}
impl JDWPTransportCapabilities {
    #[inline]
    pub fn can_timeout_attach(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_can_timeout_attach(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn can_timeout_accept(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_can_timeout_accept(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn can_timeout_handshake(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_can_timeout_handshake(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved3(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved3(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved4(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved4(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved5(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved5(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved6(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved6(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved7(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved7(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved8(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved8(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved9(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved9(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved10(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved10(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved11(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved11(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved12(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved12(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved13(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved13(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved14(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved14(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved15(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_reserved15(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        can_timeout_attach: ::std::os::raw::c_uint,
        can_timeout_accept: ::std::os::raw::c_uint,
        can_timeout_handshake: ::std::os::raw::c_uint,
        reserved3: ::std::os::raw::c_uint,
        reserved4: ::std::os::raw::c_uint,
        reserved5: ::std::os::raw::c_uint,
        reserved6: ::std::os::raw::c_uint,
        reserved7: ::std::os::raw::c_uint,
        reserved8: ::std::os::raw::c_uint,
        reserved9: ::std::os::raw::c_uint,
        reserved10: ::std::os::raw::c_uint,
        reserved11: ::std::os::raw::c_uint,
        reserved12: ::std::os::raw::c_uint,
        reserved13: ::std::os::raw::c_uint,
        reserved14: ::std::os::raw::c_uint,
        reserved15: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let can_timeout_attach: u32 = unsafe { ::std::mem::transmute(can_timeout_attach) };
            can_timeout_attach as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let can_timeout_accept: u32 = unsafe { ::std::mem::transmute(can_timeout_accept) };
            can_timeout_accept as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let can_timeout_handshake: u32 =
                unsafe { ::std::mem::transmute(can_timeout_handshake) };
            can_timeout_handshake as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let reserved3: u32 = unsafe { ::std::mem::transmute(reserved3) };
            reserved3 as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let reserved4: u32 = unsafe { ::std::mem::transmute(reserved4) };
            reserved4 as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let reserved5: u32 = unsafe { ::std::mem::transmute(reserved5) };
            reserved5 as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let reserved6: u32 = unsafe { ::std::mem::transmute(reserved6) };
            reserved6 as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let reserved7: u32 = unsafe { ::std::mem::transmute(reserved7) };
            reserved7 as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let reserved8: u32 = unsafe { ::std::mem::transmute(reserved8) };
            reserved8 as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let reserved9: u32 = unsafe { ::std::mem::transmute(reserved9) };
            reserved9 as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let reserved10: u32 = unsafe { ::std::mem::transmute(reserved10) };
            reserved10 as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let reserved11: u32 = unsafe { ::std::mem::transmute(reserved11) };
            reserved11 as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let reserved12: u32 = unsafe { ::std::mem::transmute(reserved12) };
            reserved12 as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let reserved13: u32 = unsafe { ::std::mem::transmute(reserved13) };
            reserved13 as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let reserved14: u32 = unsafe { ::std::mem::transmute(reserved14) };
            reserved14 as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let reserved15: u32 = unsafe { ::std::mem::transmute(reserved15) };
            reserved15 as u64
        });
        __bindgen_bitfield_unit
    }
}
pub const JDWPTRANSPORT_FLAGS_NONE: ::std::os::raw::c_uint = 0;
pub const JDWPTRANSPORT_FLAGS_REPLY: ::std::os::raw::c_uint = 128;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jdwpCmdPacket {
    pub len: jint,
    pub id: jint,
    pub flags: jbyte,
    pub cmdSet: jbyte,
    pub cmd: jbyte,
    pub data: *mut jbyte,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jdwpReplyPacket {
    pub len: jint,
    pub id: jint,
    pub flags: jbyte,
    pub errorCode: jshort,
    pub data: *mut jbyte,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jdwpPacket {
    pub type_: jdwpPacket__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union jdwpPacket__bindgen_ty_1 {
    pub cmd: jdwpCmdPacket,
    pub reply: jdwpReplyPacket,
    _bindgen_union_align: [u64; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jdwpTransportCallback {
    pub alloc:
        ::std::option::Option<unsafe extern "C" fn(numBytes: jint) -> *mut ::std::os::raw::c_void>,
    pub free: ::std::option::Option<unsafe extern "C" fn(buffer: *mut ::std::os::raw::c_void)>,
}
pub type jdwpTransport_OnLoad_t = ::std::option::Option<
    unsafe extern "C" fn(
        jvm: *mut JavaVM,
        callback: *mut jdwpTransportCallback,
        version: jint,
        env: *mut *mut jdwpTransportEnv,
    ) -> jint,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jdwpTransportConfiguration {
    pub allowed_peers: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jdwpTransportNativeInterface_ {
    pub reserved1: *mut ::std::os::raw::c_void,
    pub GetCapabilities: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut jdwpTransportEnv,
            capabilities_ptr: *mut JDWPTransportCapabilities,
        ) -> jdwpTransportError,
    >,
    pub Attach: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut jdwpTransportEnv,
            address: *const ::std::os::raw::c_char,
            attach_timeout: jlong,
            handshake_timeout: jlong,
        ) -> jdwpTransportError,
    >,
    pub StartListening: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut jdwpTransportEnv,
            address: *const ::std::os::raw::c_char,
            actual_address: *mut *mut ::std::os::raw::c_char,
        ) -> jdwpTransportError,
    >,
    pub StopListening: ::std::option::Option<
        unsafe extern "C" fn(env: *mut jdwpTransportEnv) -> jdwpTransportError,
    >,
    pub Accept: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut jdwpTransportEnv,
            accept_timeout: jlong,
            handshake_timeout: jlong,
        ) -> jdwpTransportError,
    >,
    pub IsOpen: ::std::option::Option<unsafe extern "C" fn(env: *mut jdwpTransportEnv) -> jboolean>,
    pub Close: ::std::option::Option<
        unsafe extern "C" fn(env: *mut jdwpTransportEnv) -> jdwpTransportError,
    >,
    pub ReadPacket: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut jdwpTransportEnv,
            pkt: *mut jdwpPacket,
        ) -> jdwpTransportError,
    >,
    pub WritePacket: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut jdwpTransportEnv,
            pkt: *const jdwpPacket,
        ) -> jdwpTransportError,
    >,
    pub GetLastError: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut jdwpTransportEnv,
            error: *mut *mut ::std::os::raw::c_char,
        ) -> jdwpTransportError,
    >,
    pub SetTransportConfiguration: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut jdwpTransportEnv,
            config: *mut jdwpTransportConfiguration,
        ) -> jdwpTransportError,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jdwpTransportEnv {
    pub functions: *const jdwpTransportNativeInterface_,
}
