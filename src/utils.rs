#[macro_export]
macro_rules! cstr {
	($s:expr) => {
		std::ffi::CString::new($s).unwrap().into_raw()
	};
}

#[macro_export]
macro_rules! check_jni {
	($e:expr) => {{
		let result = $e;
		if result != rs_jvm_bindings::jni::JNI_OK as u32 {
			panic!("JNI expression failed (received {} from {})", result, stringify!($e));
		}
	}};
}
