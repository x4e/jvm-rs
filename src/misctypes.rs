#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub type size_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
	_unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
	_unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
	pub _flags: ::std::os::raw::c_int,
	pub _IO_read_ptr: *mut ::std::os::raw::c_char,
	pub _IO_read_end: *mut ::std::os::raw::c_char,
	pub _IO_read_base: *mut ::std::os::raw::c_char,
	pub _IO_write_base: *mut ::std::os::raw::c_char,
	pub _IO_write_ptr: *mut ::std::os::raw::c_char,
	pub _IO_write_end: *mut ::std::os::raw::c_char,
	pub _IO_buf_base: *mut ::std::os::raw::c_char,
	pub _IO_buf_end: *mut ::std::os::raw::c_char,
	pub _IO_save_base: *mut ::std::os::raw::c_char,
	pub _IO_backup_base: *mut ::std::os::raw::c_char,
	pub _IO_save_end: *mut ::std::os::raw::c_char,
	pub _markers: *mut _IO_marker,
	pub _chain: *mut _IO_FILE,
	pub _fileno: ::std::os::raw::c_int,
	pub _flags2: ::std::os::raw::c_int,
	pub _old_offset: __off_t,
	pub _cur_column: ::std::os::raw::c_ushort,
	pub _vtable_offset: ::std::os::raw::c_schar,
	pub _shortbuf: [::std::os::raw::c_char; 1usize],
	pub _lock: *mut _IO_lock_t,
	pub _offset: __off64_t,
	pub _codecvt: *mut _IO_codecvt,
	pub _wide_data: *mut _IO_wide_data,
	pub _freeres_list: *mut _IO_FILE,
	pub _freeres_buf: *mut ::std::os::raw::c_void,
	pub __pad5: size_t,
	pub _mode: ::std::os::raw::c_int,
	pub _unused2: [::std::os::raw::c_char; 20usize],
}
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
