use std::ffi::CString;

#[macro_export]
macro_rules! cstr {
	($s:expr) => {
		CString::new($s).unwrap().into_raw()
	};
}

#[macro_export]
macro_rules! check_jni {
	($e:expr) => {
		let result = $e;
		if result != JNI_OK as i32 {
			panic!("JNI expression failed (received {} from {})", result, stringify!($e));
		}
	};
}
