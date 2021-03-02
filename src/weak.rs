use std::sync::atomic::{AtomicUsize, Ordering};
use std::{marker, mem};
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

pub struct Weak<F> {
	name: &'static str,
	addr: AtomicUsize,
	_marker: marker::PhantomData<F>,
}

impl<F> Weak<F> {
	pub const fn new(name: &'static str) -> Weak<F> {
		Weak {
			name,
			addr: AtomicUsize::new(1),
			_marker: marker::PhantomData,
		}
	}
	
	pub fn get(&self) -> Option<&F> {
		assert_eq!(mem::size_of::<F>(), mem::size_of::<usize>());
		unsafe {
			if self.addr.load(Ordering::SeqCst) == 1 {
				self.addr.store(fetch(self.name), Ordering::SeqCst);
			}
			if self.addr.load(Ordering::SeqCst) == 0 {
				None
			} else {
				mem::transmute::<&AtomicUsize, Option<&F>>(&self.addr)
			}
		}
	}
}

unsafe fn fetch(name: &str) -> usize {
	let name = match CString::new(name) {
		Ok(cstr) => cstr,
		Err(..) => return 0,
	};
	find_symbol(name.as_ptr())
}

#[cfg(unix)]
unsafe fn find_symbol(name: *const c_char) -> usize {
	extern "C" {
		pub fn dlsym(
			handle: *mut c_void,
			symbol: *const c_char,
		) -> *mut c_void;
	}
	pub const RTLD_DEFAULT: *mut c_void = 0i64 as *mut c_void;
	
	dlsym(RTLD_DEFAULT, name) as usize
}

#[cfg(windows)]
unsafe fn find_symbol(name: *const c_char) -> usize {
	use std::ptr::null;
	use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
	GetProcAddress(GetModuleHandleA(null()), name) as usize
}
