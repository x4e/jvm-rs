#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
	pub gp_offset: ::std::os::raw::c_uint,
	pub fp_offset: ::std::os::raw::c_uint,
	pub overflow_arg_area: *mut ::std::os::raw::c_void,
	pub reg_save_area: *mut ::std::os::raw::c_void,
}
