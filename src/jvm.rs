#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// to replace extern funs with weak:
// find: extern "C" \{[\n\r\t ]*pub fn ([A-z_0-9]+)([()A-z0-9_,\n\r\t\->*: ]+);[\n\r\t ]*\}
// replace: pub static $1: Weak<fn$2> = Weak::new("$1");

use crate::jni::*;
use crate::weak::Weak;
use crate::varargs::__va_list_tag;

pub type size_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;

pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
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

pub static JVM_GetInterfaceVersion: Weak<fn() -> jint> = Weak::new("JVM_GetInterfaceVersion");
#[doc = "PART 1: Functions for Native Libraries"]
pub static JVM_IHashCode: Weak<fn(env: *mut JNIEnv, obj: jobject) -> jint> = Weak::new("JVM_IHashCode");
pub static JVM_MonitorWait: Weak<fn(env: *mut JNIEnv, obj: jobject, ms: jlong)> = Weak::new("JVM_MonitorWait");
pub static JVM_MonitorNotify: Weak<fn(env: *mut JNIEnv, obj: jobject)> = Weak::new("JVM_MonitorNotify");
pub static JVM_MonitorNotifyAll: Weak<fn(env: *mut JNIEnv, obj: jobject)> = Weak::new("JVM_MonitorNotifyAll");
pub static JVM_Clone: Weak<fn(env: *mut JNIEnv, obj: jobject) -> jobject> = Weak::new("JVM_Clone");
pub static JVM_InternString: Weak<fn(env: *mut JNIEnv, str_: jstring) -> jstring> = Weak::new("JVM_InternString");
pub static JVM_CurrentTimeMillis: Weak<fn(env: *mut JNIEnv, ignored: jclass) -> jlong> = Weak::new("JVM_CurrentTimeMillis");
pub static JVM_NanoTime: Weak<fn(env: *mut JNIEnv, ignored: jclass) -> jlong> = Weak::new("JVM_NanoTime");
pub static JVM_GetNanoTimeAdjustment: Weak<fn(
		env: *mut JNIEnv,
		ignored: jclass,
		offset_secs: jlong,
	) -> jlong> = Weak::new("JVM_GetNanoTimeAdjustment");
pub static JVM_ArrayCopy: Weak<fn(
		env: *mut JNIEnv,
		ignored: jclass,
		src: jobject,
		src_pos: jint,
		dst: jobject,
		dst_pos: jint,
		length: jint,
	)> = Weak::new("JVM_ArrayCopy");
pub static JVM_GetProperties: Weak<fn(env: *mut JNIEnv) -> jobjectArray> = Weak::new("JVM_GetProperties");
pub static JVM_BeforeHalt: Weak<fn()> = Weak::new("JVM_BeforeHalt");
pub static JVM_Halt: Weak<fn(code: jint)> = Weak::new("JVM_Halt");
pub static JVM_GC: Weak<fn()> = Weak::new("JVM_GC");
pub static JVM_MaxObjectInspectionAge: Weak<fn() -> jlong> = Weak::new("JVM_MaxObjectInspectionAge");
pub static JVM_TotalMemory: Weak<fn() -> jlong> = Weak::new("JVM_TotalMemory");
pub static JVM_FreeMemory: Weak<fn() -> jlong> = Weak::new("JVM_FreeMemory");
pub static JVM_MaxMemory: Weak<fn() -> jlong> = Weak::new("JVM_MaxMemory");
pub static JVM_ActiveProcessorCount: Weak<fn() -> jint> = Weak::new("JVM_ActiveProcessorCount");
pub static JVM_LoadLibrary: Weak<fn(name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void> = Weak::new("JVM_LoadLibrary");
pub static JVM_UnloadLibrary: Weak<fn(handle: *mut ::std::os::raw::c_void)> = Weak::new("JVM_UnloadLibrary");
pub static JVM_FindLibraryEntry: Weak<fn(
		handle: *mut ::std::os::raw::c_void,
		name: *const ::std::os::raw::c_char,
	) -> *mut ::std::os::raw::c_void> = Weak::new("JVM_FindLibraryEntry");
pub static JVM_IsSupportedJNIVersion: Weak<fn(version: jint) -> jboolean> = Weak::new("JVM_IsSupportedJNIVersion");
pub static JVM_GetVmArguments: Weak<fn(env: *mut JNIEnv) -> jobjectArray> = Weak::new("JVM_GetVmArguments");
pub static JVM_InitializeFromArchive: Weak<fn(env: *mut JNIEnv, cls: jclass)> = Weak::new("JVM_InitializeFromArchive");
pub static JVM_RegisterLambdaProxyClassForArchiving: Weak<fn(
		env: *mut JNIEnv,
		caller: jclass,
		invokedName: jstring,
		invokedType: jobject,
		methodType: jobject,
		implMethodMember: jobject,
		instantiatedMethodType: jobject,
		lambdaProxyClass: jclass,
	)> = Weak::new("JVM_RegisterLambdaProxyClassForArchiving");
pub static JVM_LookupLambdaProxyClassFromArchive: Weak<fn(
		env: *mut JNIEnv,
		caller: jclass,
		invokedName: jstring,
		invokedType: jobject,
		methodType: jobject,
		implMethodMember: jobject,
		instantiatedMethodType: jobject,
		initialize: jboolean,
	) -> jclass> = Weak::new("JVM_LookupLambdaProxyClassFromArchive");
pub static JVM_IsCDSDumpingEnabled: Weak<fn(env: *mut JNIEnv) -> jboolean> = Weak::new("JVM_IsCDSDumpingEnabled");
pub static JVM_IsCDSSharingEnabled: Weak<fn(env: *mut JNIEnv) -> jboolean> = Weak::new("JVM_IsCDSSharingEnabled");
pub static JVM_GetRandomSeedForCDSDump: Weak<fn() -> jlong> = Weak::new("JVM_GetRandomSeedForCDSDump");
pub static JVM_FillInStackTrace: Weak<fn(env: *mut JNIEnv, throwable: jobject)> = Weak::new("JVM_FillInStackTrace");
pub static JVM_InitStackTraceElementArray: Weak<fn(
		env: *mut JNIEnv,
		elements: jobjectArray,
		throwable: jobject,
	)> = Weak::new("JVM_InitStackTraceElementArray");
pub static JVM_InitStackTraceElement: Weak<fn(env: *mut JNIEnv, element: jobject, stackFrameInfo: jobject)> = Weak::new("JVM_InitStackTraceElement");
pub static JVM_GetExtendedNPEMessage: Weak<fn(env: *mut JNIEnv, throwable: jthrowable) -> jstring> = Weak::new("JVM_GetExtendedNPEMessage");
pub const JVM_STACKWALK_FILL_CLASS_REFS_ONLY: ::std::os::raw::c_uint = 2;
pub const JVM_STACKWALK_GET_CALLER_CLASS: ::std::os::raw::c_uint = 4;
pub const JVM_STACKWALK_SHOW_HIDDEN_FRAMES: ::std::os::raw::c_uint = 32;
pub const JVM_STACKWALK_FILL_LIVE_STACK_FRAMES: ::std::os::raw::c_uint = 256;
pub static JVM_CallStackWalk: Weak<fn(
		env: *mut JNIEnv,
		stackStream: jobject,
		mode: jlong,
		skip_frames: jint,
		frame_count: jint,
		start_index: jint,
		frames: jobjectArray,
	) -> jobject> = Weak::new("JVM_CallStackWalk");
pub static JVM_MoreStackWalk: Weak<fn(
		env: *mut JNIEnv,
		stackStream: jobject,
		mode: jlong,
		anchor: jlong,
		frame_count: jint,
		start_index: jint,
		frames: jobjectArray,
	) -> jint> = Weak::new("JVM_MoreStackWalk");
pub static JVM_StartThread: Weak<fn(env: *mut JNIEnv, thread: jobject)> = Weak::new("JVM_StartThread");
pub static JVM_StopThread: Weak<fn(env: *mut JNIEnv, thread: jobject, exception: jobject)> = Weak::new("JVM_StopThread");
pub static JVM_IsThreadAlive: Weak<fn(env: *mut JNIEnv, thread: jobject) -> jboolean> = Weak::new("JVM_IsThreadAlive");
pub static JVM_SuspendThread: Weak<fn(env: *mut JNIEnv, thread: jobject)> = Weak::new("JVM_SuspendThread");
pub static JVM_ResumeThread: Weak<fn(env: *mut JNIEnv, thread: jobject)> = Weak::new("JVM_ResumeThread");
pub static JVM_SetThreadPriority: Weak<fn(env: *mut JNIEnv, thread: jobject, prio: jint)> = Weak::new("JVM_SetThreadPriority");
pub static JVM_Yield: Weak<fn(env: *mut JNIEnv, threadClass: jclass)> = Weak::new("JVM_Yield");
pub static JVM_Sleep: Weak<fn(env: *mut JNIEnv, threadClass: jclass, millis: jlong)> = Weak::new("JVM_Sleep");
pub static JVM_CurrentThread: Weak<fn(env: *mut JNIEnv, threadClass: jclass) -> jobject> = Weak::new("JVM_CurrentThread");
pub static JVM_Interrupt: Weak<fn(env: *mut JNIEnv, thread: jobject)> = Weak::new("JVM_Interrupt");
pub static JVM_HoldsLock: Weak<fn(env: *mut JNIEnv, threadClass: jclass, obj: jobject) -> jboolean> = Weak::new("JVM_HoldsLock");
pub static JVM_DumpAllStacks: Weak<fn(env: *mut JNIEnv, unused: jclass)> = Weak::new("JVM_DumpAllStacks");
pub static JVM_GetAllThreads: Weak<fn(env: *mut JNIEnv, dummy: jclass) -> jobjectArray> = Weak::new("JVM_GetAllThreads");
pub static JVM_SetNativeThreadName: Weak<fn(env: *mut JNIEnv, jthread: jobject, name: jstring)> = Weak::new("JVM_SetNativeThreadName");
pub static JVM_DumpThreads: Weak<fn(
		env: *mut JNIEnv,
		threadClass: jclass,
		threads: jobjectArray,
	) -> jobjectArray> = Weak::new("JVM_DumpThreads");
pub static JVM_GetClassContext: Weak<fn(env: *mut JNIEnv) -> jobjectArray> = Weak::new("JVM_GetClassContext");
pub static JVM_GetSystemPackage: Weak<fn(env: *mut JNIEnv, name: jstring) -> jstring> = Weak::new("JVM_GetSystemPackage");
pub static JVM_GetSystemPackages: Weak<fn(env: *mut JNIEnv) -> jobjectArray> = Weak::new("JVM_GetSystemPackages");
pub static JVM_GetAndClearReferencePendingList: Weak<fn(env: *mut JNIEnv) -> jobject> = Weak::new("JVM_GetAndClearReferencePendingList");
pub static JVM_HasReferencePendingList: Weak<fn(env: *mut JNIEnv) -> jboolean> = Weak::new("JVM_HasReferencePendingList");
pub static JVM_WaitForReferencePendingList: Weak<fn(env: *mut JNIEnv)> = Weak::new("JVM_WaitForReferencePendingList");
pub static JVM_LatestUserDefinedLoader: Weak<fn(env: *mut JNIEnv) -> jobject> = Weak::new("JVM_LatestUserDefinedLoader");
pub static JVM_GetArrayLength: Weak<fn(env: *mut JNIEnv, arr: jobject) -> jint> = Weak::new("JVM_GetArrayLength");
pub static JVM_GetArrayElement: Weak<fn(env: *mut JNIEnv, arr: jobject, index: jint) -> jobject> = Weak::new("JVM_GetArrayElement");
pub static JVM_GetPrimitiveArrayElement: Weak<fn(
		env: *mut JNIEnv,
		arr: jobject,
		index: jint,
		wCode: jint,
	) -> jvalue> = Weak::new("JVM_GetPrimitiveArrayElement");
pub static JVM_SetArrayElement: Weak<fn(env: *mut JNIEnv, arr: jobject, index: jint, val: jobject)> = Weak::new("JVM_SetArrayElement");
pub static JVM_SetPrimitiveArrayElement: Weak<fn(
		env: *mut JNIEnv,
		arr: jobject,
		index: jint,
		v: jvalue,
		vCode: ::std::os::raw::c_uchar,
	)> = Weak::new("JVM_SetPrimitiveArrayElement");
pub static JVM_NewArray: Weak<fn(env: *mut JNIEnv, eltClass: jclass, length: jint) -> jobject> = Weak::new("JVM_NewArray");
pub static JVM_NewMultiArray: Weak<fn(env: *mut JNIEnv, eltClass: jclass, dim: jintArray) -> jobject> = Weak::new("JVM_NewMultiArray");
pub static JVM_GetCallerClass: Weak<fn(env: *mut JNIEnv) -> jclass> = Weak::new("JVM_GetCallerClass");
pub static JVM_FindPrimitiveClass: Weak<fn(env: *mut JNIEnv, utf: *const ::std::os::raw::c_char) -> jclass> = Weak::new("JVM_FindPrimitiveClass");
pub static JVM_FindClassFromBootLoader: Weak<fn(
		env: *mut JNIEnv,
		name: *const ::std::os::raw::c_char,
	) -> jclass> = Weak::new("JVM_FindClassFromBootLoader");
pub static JVM_FindClassFromCaller: Weak<fn(
		env: *mut JNIEnv,
		name: *const ::std::os::raw::c_char,
		init: jboolean,
		loader: jobject,
		caller: jclass,
	) -> jclass> = Weak::new("JVM_FindClassFromCaller");
pub static JVM_FindClassFromClass: Weak<fn(
		env: *mut JNIEnv,
		name: *const ::std::os::raw::c_char,
		init: jboolean,
		from: jclass,
	) -> jclass> = Weak::new("JVM_FindClassFromClass");
pub static JVM_FindLoadedClass: Weak<fn(env: *mut JNIEnv, loader: jobject, name: jstring) -> jclass> = Weak::new("JVM_FindLoadedClass");
pub static JVM_DefineClass: Weak<fn(
		env: *mut JNIEnv,
		name: *const ::std::os::raw::c_char,
		loader: jobject,
		buf: *const jbyte,
		len: jsize,
		pd: jobject,
	) -> jclass> = Weak::new("JVM_DefineClass");
pub static JVM_DefineClassWithSource: Weak<fn(
		env: *mut JNIEnv,
		name: *const ::std::os::raw::c_char,
		loader: jobject,
		buf: *const jbyte,
		len: jsize,
		pd: jobject,
		source: *const ::std::os::raw::c_char,
	) -> jclass> = Weak::new("JVM_DefineClassWithSource");
pub static JVM_LookupDefineClass: Weak<fn(
		env: *mut JNIEnv,
		lookup: jclass,
		name: *const ::std::os::raw::c_char,
		buf: *const jbyte,
		len: jsize,
		pd: jobject,
		init: jboolean,
		flags: ::std::os::raw::c_int,
		classData: jobject,
	) -> jclass> = Weak::new("JVM_LookupDefineClass");
pub static JVM_DefineModule: Weak<fn(
		env: *mut JNIEnv,
		module: jobject,
		is_open: jboolean,
		version: jstring,
		location: jstring,
		packages: jobjectArray,
	)> = Weak::new("JVM_DefineModule");
pub static JVM_SetBootLoaderUnnamedModule: Weak<fn(env: *mut JNIEnv, module: jobject)> = Weak::new("JVM_SetBootLoaderUnnamedModule");
pub static JVM_AddModuleExports: Weak<fn(
		env: *mut JNIEnv,
		from_module: jobject,
		package: jstring,
		to_module: jobject,
	)> = Weak::new("JVM_AddModuleExports");
pub static JVM_AddModuleExportsToAllUnnamed: Weak<fn(
		env: *mut JNIEnv,
		from_module: jobject,
		package: jstring,
	)> = Weak::new("JVM_AddModuleExportsToAllUnnamed");
pub static JVM_AddModuleExportsToAll: Weak<fn(env: *mut JNIEnv, from_module: jobject, package: jstring)> = Weak::new("JVM_AddModuleExportsToAll");
pub static JVM_AddReadsModule: Weak<fn(env: *mut JNIEnv, from_module: jobject, source_module: jobject)> = Weak::new("JVM_AddReadsModule");
pub static JVM_InitClassName: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jstring> = Weak::new("JVM_InitClassName");
pub static JVM_GetClassInterfaces: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jobjectArray> = Weak::new("JVM_GetClassInterfaces");
pub static JVM_IsInterface: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jboolean> = Weak::new("JVM_IsInterface");
pub static JVM_GetClassSigners: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jobjectArray> = Weak::new("JVM_GetClassSigners");
pub static JVM_SetClassSigners: Weak<fn(env: *mut JNIEnv, cls: jclass, signers: jobjectArray)> = Weak::new("JVM_SetClassSigners");
pub static JVM_GetProtectionDomain: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jobject> = Weak::new("JVM_GetProtectionDomain");
pub static JVM_IsArrayClass: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jboolean> = Weak::new("JVM_IsArrayClass");
pub static JVM_IsPrimitiveClass: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jboolean> = Weak::new("JVM_IsPrimitiveClass");
pub static JVM_IsHiddenClass: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jboolean> = Weak::new("JVM_IsHiddenClass");
pub static JVM_GetClassModifiers: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jint> = Weak::new("JVM_GetClassModifiers");
pub static JVM_GetDeclaredClasses: Weak<fn(env: *mut JNIEnv, ofClass: jclass) -> jobjectArray> = Weak::new("JVM_GetDeclaredClasses");
pub static JVM_GetDeclaringClass: Weak<fn(env: *mut JNIEnv, ofClass: jclass) -> jclass> = Weak::new("JVM_GetDeclaringClass");
pub static JVM_GetSimpleBinaryName: Weak<fn(env: *mut JNIEnv, ofClass: jclass) -> jstring> = Weak::new("JVM_GetSimpleBinaryName");
pub static JVM_GetClassSignature: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jstring> = Weak::new("JVM_GetClassSignature");
pub static JVM_GetClassAnnotations: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jbyteArray> = Weak::new("JVM_GetClassAnnotations");
pub static JVM_GetClassTypeAnnotations: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jbyteArray> = Weak::new("JVM_GetClassTypeAnnotations");
pub static JVM_GetFieldTypeAnnotations: Weak<fn(env: *mut JNIEnv, field: jobject) -> jbyteArray> = Weak::new("JVM_GetFieldTypeAnnotations");
pub static JVM_GetMethodTypeAnnotations: Weak<fn(env: *mut JNIEnv, method: jobject) -> jbyteArray> = Weak::new("JVM_GetMethodTypeAnnotations");
pub static JVM_GetClassDeclaredMethods: Weak<fn(
		env: *mut JNIEnv,
		ofClass: jclass,
		publicOnly: jboolean,
	) -> jobjectArray> = Weak::new("JVM_GetClassDeclaredMethods");
pub static JVM_GetClassDeclaredFields: Weak<fn(
		env: *mut JNIEnv,
		ofClass: jclass,
		publicOnly: jboolean,
	) -> jobjectArray> = Weak::new("JVM_GetClassDeclaredFields");
pub static JVM_GetClassDeclaredConstructors: Weak<fn(
		env: *mut JNIEnv,
		ofClass: jclass,
		publicOnly: jboolean,
	) -> jobjectArray> = Weak::new("JVM_GetClassDeclaredConstructors");
pub static JVM_GetClassAccessFlags: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jint> = Weak::new("JVM_GetClassAccessFlags");
pub static JVM_AreNestMates: Weak<fn(env: *mut JNIEnv, current: jclass, member: jclass) -> jboolean> = Weak::new("JVM_AreNestMates");
pub static JVM_GetNestHost: Weak<fn(env: *mut JNIEnv, current: jclass) -> jclass> = Weak::new("JVM_GetNestHost");
pub static JVM_GetNestMembers: Weak<fn(env: *mut JNIEnv, current: jclass) -> jobjectArray> = Weak::new("JVM_GetNestMembers");
pub static JVM_IsRecord: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jboolean> = Weak::new("JVM_IsRecord");
pub static JVM_GetRecordComponents: Weak<fn(env: *mut JNIEnv, ofClass: jclass) -> jobjectArray> = Weak::new("JVM_GetRecordComponents");
pub static JVM_GetPermittedSubclasses: Weak<fn(env: *mut JNIEnv, current: jclass) -> jobjectArray> = Weak::new("JVM_GetPermittedSubclasses");
pub static JVM_InvokeMethod: Weak<fn(
		env: *mut JNIEnv,
		method: jobject,
		obj: jobject,
		args0: jobjectArray,
	) -> jobject> = Weak::new("JVM_InvokeMethod");
pub static JVM_NewInstanceFromConstructor: Weak<fn(
		env: *mut JNIEnv,
		c: jobject,
		args0: jobjectArray,
	) -> jobject> = Weak::new("JVM_NewInstanceFromConstructor");
pub static JVM_GetClassConstantPool: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jobject> = Weak::new("JVM_GetClassConstantPool");
pub static JVM_ConstantPoolGetSize: Weak<fn(env: *mut JNIEnv, unused: jobject, jcpool: jobject) -> jint> = Weak::new("JVM_ConstantPoolGetSize");
pub static JVM_ConstantPoolGetClassAt: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jclass> = Weak::new("JVM_ConstantPoolGetClassAt");
pub static JVM_ConstantPoolGetClassAtIfLoaded: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jclass> = Weak::new("JVM_ConstantPoolGetClassAtIfLoaded");
pub static JVM_ConstantPoolGetClassRefIndexAt: Weak<fn(
		env: *mut JNIEnv,
		obj: jobject,
		unused: jobject,
		index: jint,
	) -> jint> = Weak::new("JVM_ConstantPoolGetClassRefIndexAt");
pub static JVM_ConstantPoolGetMethodAt: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jobject> = Weak::new("JVM_ConstantPoolGetMethodAt");
pub static JVM_ConstantPoolGetMethodAtIfLoaded: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jobject> = Weak::new("JVM_ConstantPoolGetMethodAtIfLoaded");
pub static JVM_ConstantPoolGetFieldAt: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jobject> = Weak::new("JVM_ConstantPoolGetFieldAt");
pub static JVM_ConstantPoolGetFieldAtIfLoaded: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jobject> = Weak::new("JVM_ConstantPoolGetFieldAtIfLoaded");
pub static JVM_ConstantPoolGetMemberRefInfoAt: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jobjectArray> = Weak::new("JVM_ConstantPoolGetMemberRefInfoAt");
pub static JVM_ConstantPoolGetNameAndTypeRefIndexAt: Weak<fn(
		env: *mut JNIEnv,
		obj: jobject,
		unused: jobject,
		index: jint,
	) -> jint> = Weak::new("JVM_ConstantPoolGetNameAndTypeRefIndexAt");
pub static JVM_ConstantPoolGetNameAndTypeRefInfoAt: Weak<fn(
		env: *mut JNIEnv,
		obj: jobject,
		unused: jobject,
		index: jint,
	) -> jobjectArray> = Weak::new("JVM_ConstantPoolGetNameAndTypeRefInfoAt");
pub static JVM_ConstantPoolGetIntAt: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jint> = Weak::new("JVM_ConstantPoolGetIntAt");
pub static JVM_ConstantPoolGetLongAt: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jlong> = Weak::new("JVM_ConstantPoolGetLongAt");
pub static JVM_ConstantPoolGetFloatAt: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jfloat> = Weak::new("JVM_ConstantPoolGetFloatAt");
pub static JVM_ConstantPoolGetDoubleAt: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jdouble> = Weak::new("JVM_ConstantPoolGetDoubleAt");
pub static JVM_ConstantPoolGetStringAt: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jstring> = Weak::new("JVM_ConstantPoolGetStringAt");
pub static JVM_ConstantPoolGetUTF8At: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jstring> = Weak::new("JVM_ConstantPoolGetUTF8At");
pub static JVM_ConstantPoolGetTagAt: Weak<fn(
		env: *mut JNIEnv,
		unused: jobject,
		jcpool: jobject,
		index: jint,
	) -> jbyte> = Weak::new("JVM_ConstantPoolGetTagAt");
pub static JVM_GetMethodParameters: Weak<fn(env: *mut JNIEnv, method: jobject) -> jobjectArray> = Weak::new("JVM_GetMethodParameters");
pub static JVM_GetInheritedAccessControlContext: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jobject> = Weak::new("JVM_GetInheritedAccessControlContext");
pub static JVM_GetStackAccessControlContext: Weak<fn(env: *mut JNIEnv, cls: jclass) -> jobject> = Weak::new("JVM_GetStackAccessControlContext");
pub static JVM_RegisterSignal: Weak<fn(
		sig: jint,
		handler: *mut ::std::os::raw::c_void,
	) -> *mut ::std::os::raw::c_void> = Weak::new("JVM_RegisterSignal");
pub static JVM_RaiseSignal: Weak<fn(sig: jint) -> jboolean> = Weak::new("JVM_RaiseSignal");
pub static JVM_FindSignal: Weak<fn(name: *const ::std::os::raw::c_char) -> jint> = Weak::new("JVM_FindSignal");
pub static JVM_DesiredAssertionStatus: Weak<fn(env: *mut JNIEnv, unused: jclass, cls: jclass) -> jboolean> = Weak::new("JVM_DesiredAssertionStatus");
pub static JVM_AssertionStatusDirectives: Weak<fn(env: *mut JNIEnv, unused: jclass) -> jobject> = Weak::new("JVM_AssertionStatusDirectives");
pub static JVM_SupportsCX8: Weak<fn() -> jboolean> = Weak::new("JVM_SupportsCX8");
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JVM_DTraceProbe {
	pub method: jmethodID,
	pub function: jstring,
	pub name: jstring,
	pub reserved: [*mut ::std::os::raw::c_void; 4usize],
}
#[doc = " Encapsulates the stability ratings for a DTrace provider field"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JVM_DTraceInterfaceAttributes {
	pub nameStability: jint,
	pub dataStability: jint,
	pub dependencyClass: jint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JVM_DTraceProvider {
	pub name: jstring,
	pub probes: *mut JVM_DTraceProbe,
	pub probe_count: jint,
	pub providerAttributes: JVM_DTraceInterfaceAttributes,
	pub moduleAttributes: JVM_DTraceInterfaceAttributes,
	pub functionAttributes: JVM_DTraceInterfaceAttributes,
	pub nameAttributes: JVM_DTraceInterfaceAttributes,
	pub argsAttributes: JVM_DTraceInterfaceAttributes,
	pub reserved: [*mut ::std::os::raw::c_void; 4usize],
}
pub static JVM_DTraceGetVersion: Weak<fn(env: *mut JNIEnv) -> jint> = Weak::new("JVM_DTraceGetVersion");
pub static JVM_DTraceActivate: Weak<fn(
		env: *mut JNIEnv,
		version: jint,
		module_name: jstring,
		providers_count: jint,
		providers: *mut JVM_DTraceProvider,
	) -> jlong> = Weak::new("JVM_DTraceActivate");
pub static JVM_DTraceIsProbeEnabled: Weak<fn(env: *mut JNIEnv, method: jmethodID) -> jboolean> = Weak::new("JVM_DTraceIsProbeEnabled");
pub static JVM_DTraceDispose: Weak<fn(env: *mut JNIEnv, activation_handle: jlong)> = Weak::new("JVM_DTraceDispose");
pub static JVM_DTraceIsSupported: Weak<fn(env: *mut JNIEnv) -> jboolean> = Weak::new("JVM_DTraceIsSupported");
extern "C" {
	#[doc = "PART 2: Support for the Verifier and Class File Format Checker"]
	pub fn JVM_GetClassNameUTF(env: *mut JNIEnv, cb: jclass) -> *const ::std::os::raw::c_char;
}
pub static JVM_GetClassCPTypes: Weak<fn(env: *mut JNIEnv, cb: jclass, types: *mut ::std::os::raw::c_uchar)> = Weak::new("JVM_GetClassCPTypes");
pub static JVM_GetClassCPEntriesCount: Weak<fn(env: *mut JNIEnv, cb: jclass) -> jint> = Weak::new("JVM_GetClassCPEntriesCount");
pub static JVM_GetClassFieldsCount: Weak<fn(env: *mut JNIEnv, cb: jclass) -> jint> = Weak::new("JVM_GetClassFieldsCount");
pub static JVM_GetClassMethodsCount: Weak<fn(env: *mut JNIEnv, cb: jclass) -> jint> = Weak::new("JVM_GetClassMethodsCount");
pub static JVM_GetMethodIxExceptionIndexes: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		method_index: jint,
		exceptions: *mut ::std::os::raw::c_ushort,
	)> = Weak::new("JVM_GetMethodIxExceptionIndexes");
pub static JVM_GetMethodIxExceptionsCount: Weak<fn(env: *mut JNIEnv, cb: jclass, method_index: jint)
	                                      -> jint> = Weak::new("JVM_GetMethodIxExceptionsCount");
pub static JVM_GetMethodIxByteCode: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		method_index: jint,
		code: *mut ::std::os::raw::c_uchar,
	)> = Weak::new("JVM_GetMethodIxByteCode");
pub static JVM_GetMethodIxByteCodeLength: Weak<fn(env: *mut JNIEnv, cb: jclass, method_index: jint) -> jint> = Weak::new("JVM_GetMethodIxByteCodeLength");
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JVM_ExceptionTableEntryType {
	pub start_pc: jint,
	pub end_pc: jint,
	pub handler_pc: jint,
	pub catchType: jint,
}
pub static JVM_GetMethodIxExceptionTableEntry: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		method_index: jint,
		entry_index: jint,
		entry: *mut JVM_ExceptionTableEntryType,
	)> = Weak::new("JVM_GetMethodIxExceptionTableEntry");
pub static JVM_GetMethodIxExceptionTableLength: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint> = Weak::new("JVM_GetMethodIxExceptionTableLength");
pub static JVM_GetFieldIxModifiers: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint> = Weak::new("JVM_GetFieldIxModifiers");
pub static JVM_GetMethodIxModifiers: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint> = Weak::new("JVM_GetMethodIxModifiers");
pub static JVM_GetMethodIxLocalsCount: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint> = Weak::new("JVM_GetMethodIxLocalsCount");
pub static JVM_GetMethodIxArgsSize: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint> = Weak::new("JVM_GetMethodIxArgsSize");
pub static JVM_GetMethodIxMaxStack: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jint> = Weak::new("JVM_GetMethodIxMaxStack");
pub static JVM_IsConstructorIx: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jboolean> = Weak::new("JVM_IsConstructorIx");
pub static JVM_IsVMGeneratedMethodIx: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
	) -> jboolean> = Weak::new("JVM_IsVMGeneratedMethodIx");
pub static JVM_GetMethodIxNameUTF: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char> = Weak::new("JVM_GetMethodIxNameUTF");
pub static JVM_GetMethodIxSignatureUTF: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char> = Weak::new("JVM_GetMethodIxSignatureUTF");
pub static JVM_GetCPFieldNameUTF: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char> = Weak::new("JVM_GetCPFieldNameUTF");
pub static JVM_GetCPMethodNameUTF: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char> = Weak::new("JVM_GetCPMethodNameUTF");
pub static JVM_GetCPMethodSignatureUTF: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char> = Weak::new("JVM_GetCPMethodSignatureUTF");
pub static JVM_GetCPFieldSignatureUTF: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char> = Weak::new("JVM_GetCPFieldSignatureUTF");
pub static JVM_GetCPClassNameUTF: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char> = Weak::new("JVM_GetCPClassNameUTF");
pub static JVM_GetCPFieldClassNameUTF: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char> = Weak::new("JVM_GetCPFieldClassNameUTF");
pub static JVM_GetCPMethodClassNameUTF: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: jint,
	) -> *const ::std::os::raw::c_char> = Weak::new("JVM_GetCPMethodClassNameUTF");
pub static JVM_GetCPFieldModifiers: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
		calledClass: jclass,
	) -> jint> = Weak::new("JVM_GetCPFieldModifiers");
pub static JVM_GetCPMethodModifiers: Weak<fn(
		env: *mut JNIEnv,
		cb: jclass,
		index: ::std::os::raw::c_int,
		calledClass: jclass,
	) -> jint> = Weak::new("JVM_GetCPMethodModifiers");
pub static JVM_ReleaseUTF: Weak<fn(utf: *const ::std::os::raw::c_char)> = Weak::new("JVM_ReleaseUTF");
pub static JVM_IsSameClassPackage: Weak<fn(env: *mut JNIEnv, class1: jclass, class2: jclass) -> jboolean> = Weak::new("JVM_IsSameClassPackage");
pub const JVM_ACC_PUBLIC: ::std::os::raw::c_uint = 1;
pub const JVM_ACC_PRIVATE: ::std::os::raw::c_uint = 2;
pub const JVM_ACC_PROTECTED: ::std::os::raw::c_uint = 4;
pub const JVM_ACC_STATIC: ::std::os::raw::c_uint = 8;
pub const JVM_ACC_FINAL: ::std::os::raw::c_uint = 16;
pub const JVM_ACC_SYNCHRONIZED: ::std::os::raw::c_uint = 32;
pub const JVM_ACC_SUPER: ::std::os::raw::c_uint = 32;
pub const JVM_ACC_VOLATILE: ::std::os::raw::c_uint = 64;
pub const JVM_ACC_BRIDGE: ::std::os::raw::c_uint = 64;
pub const JVM_ACC_TRANSIENT: ::std::os::raw::c_uint = 128;
pub const JVM_ACC_VARARGS: ::std::os::raw::c_uint = 128;
pub const JVM_ACC_NATIVE: ::std::os::raw::c_uint = 256;
pub const JVM_ACC_INTERFACE: ::std::os::raw::c_uint = 512;
pub const JVM_ACC_ABSTRACT: ::std::os::raw::c_uint = 1024;
pub const JVM_ACC_STRICT: ::std::os::raw::c_uint = 2048;
pub const JVM_ACC_SYNTHETIC: ::std::os::raw::c_uint = 4096;
pub const JVM_ACC_ANNOTATION: ::std::os::raw::c_uint = 8192;
pub const JVM_ACC_ENUM: ::std::os::raw::c_uint = 16384;
pub const JVM_ACC_MODULE: ::std::os::raw::c_uint = 32768;
pub const JVM_T_BOOLEAN: ::std::os::raw::c_uint = 4;
pub const JVM_T_CHAR: ::std::os::raw::c_uint = 5;
pub const JVM_T_FLOAT: ::std::os::raw::c_uint = 6;
pub const JVM_T_DOUBLE: ::std::os::raw::c_uint = 7;
pub const JVM_T_BYTE: ::std::os::raw::c_uint = 8;
pub const JVM_T_SHORT: ::std::os::raw::c_uint = 9;
pub const JVM_T_INT: ::std::os::raw::c_uint = 10;
pub const JVM_T_LONG: ::std::os::raw::c_uint = 11;
pub const JVM_CONSTANT_Utf8: ::std::os::raw::c_uint = 1;
pub const JVM_CONSTANT_Unicode: ::std::os::raw::c_uint = 2;
pub const JVM_CONSTANT_Integer: ::std::os::raw::c_uint = 3;
pub const JVM_CONSTANT_Float: ::std::os::raw::c_uint = 4;
pub const JVM_CONSTANT_Long: ::std::os::raw::c_uint = 5;
pub const JVM_CONSTANT_Double: ::std::os::raw::c_uint = 6;
pub const JVM_CONSTANT_Class: ::std::os::raw::c_uint = 7;
pub const JVM_CONSTANT_String: ::std::os::raw::c_uint = 8;
pub const JVM_CONSTANT_Fieldref: ::std::os::raw::c_uint = 9;
pub const JVM_CONSTANT_Methodref: ::std::os::raw::c_uint = 10;
pub const JVM_CONSTANT_InterfaceMethodref: ::std::os::raw::c_uint = 11;
pub const JVM_CONSTANT_NameAndType: ::std::os::raw::c_uint = 12;
pub const JVM_CONSTANT_MethodHandle: ::std::os::raw::c_uint = 15;
pub const JVM_CONSTANT_MethodType: ::std::os::raw::c_uint = 16;
pub const JVM_CONSTANT_Dynamic: ::std::os::raw::c_uint = 17;
pub const JVM_CONSTANT_InvokeDynamic: ::std::os::raw::c_uint = 18;
pub const JVM_CONSTANT_Module: ::std::os::raw::c_uint = 19;
pub const JVM_CONSTANT_Package: ::std::os::raw::c_uint = 20;
pub const JVM_CONSTANT_ExternalMax: ::std::os::raw::c_uint = 20;
pub const JVM_REF_getField: ::std::os::raw::c_uint = 1;
pub const JVM_REF_getStatic: ::std::os::raw::c_uint = 2;
pub const JVM_REF_putField: ::std::os::raw::c_uint = 3;
pub const JVM_REF_putStatic: ::std::os::raw::c_uint = 4;
pub const JVM_REF_invokeVirtual: ::std::os::raw::c_uint = 5;
pub const JVM_REF_invokeStatic: ::std::os::raw::c_uint = 6;
pub const JVM_REF_invokeSpecial: ::std::os::raw::c_uint = 7;
pub const JVM_REF_newInvokeSpecial: ::std::os::raw::c_uint = 8;
pub const JVM_REF_invokeInterface: ::std::os::raw::c_uint = 9;
pub const JVM_ITEM_Top: ::std::os::raw::c_uint = 0;
pub const JVM_ITEM_Integer: ::std::os::raw::c_uint = 1;
pub const JVM_ITEM_Float: ::std::os::raw::c_uint = 2;
pub const JVM_ITEM_Double: ::std::os::raw::c_uint = 3;
pub const JVM_ITEM_Long: ::std::os::raw::c_uint = 4;
pub const JVM_ITEM_Null: ::std::os::raw::c_uint = 5;
pub const JVM_ITEM_UninitializedThis: ::std::os::raw::c_uint = 6;
pub const JVM_ITEM_Object: ::std::os::raw::c_uint = 7;
pub const JVM_ITEM_Uninitialized: ::std::os::raw::c_uint = 8;
pub const JVM_SIGNATURE_SLASH: ::std::os::raw::c_uint = 47;
pub const JVM_SIGNATURE_DOT: ::std::os::raw::c_uint = 46;
pub const JVM_SIGNATURE_SPECIAL: ::std::os::raw::c_uint = 60;
pub const JVM_SIGNATURE_ENDSPECIAL: ::std::os::raw::c_uint = 62;
pub const JVM_SIGNATURE_ARRAY: ::std::os::raw::c_uint = 91;
pub const JVM_SIGNATURE_BYTE: ::std::os::raw::c_uint = 66;
pub const JVM_SIGNATURE_CHAR: ::std::os::raw::c_uint = 67;
pub const JVM_SIGNATURE_CLASS: ::std::os::raw::c_uint = 76;
pub const JVM_SIGNATURE_ENDCLASS: ::std::os::raw::c_uint = 59;
pub const JVM_SIGNATURE_ENUM: ::std::os::raw::c_uint = 69;
pub const JVM_SIGNATURE_FLOAT: ::std::os::raw::c_uint = 70;
pub const JVM_SIGNATURE_DOUBLE: ::std::os::raw::c_uint = 68;
pub const JVM_SIGNATURE_FUNC: ::std::os::raw::c_uint = 40;
pub const JVM_SIGNATURE_ENDFUNC: ::std::os::raw::c_uint = 41;
pub const JVM_SIGNATURE_INT: ::std::os::raw::c_uint = 73;
pub const JVM_SIGNATURE_LONG: ::std::os::raw::c_uint = 74;
pub const JVM_SIGNATURE_SHORT: ::std::os::raw::c_uint = 83;
pub const JVM_SIGNATURE_VOID: ::std::os::raw::c_uint = 86;
pub const JVM_SIGNATURE_BOOLEAN: ::std::os::raw::c_uint = 90;
pub const JVM_OPC_nop: ::std::os::raw::c_uint = 0;
pub const JVM_OPC_aconst_null: ::std::os::raw::c_uint = 1;
pub const JVM_OPC_iconst_m1: ::std::os::raw::c_uint = 2;
pub const JVM_OPC_iconst_0: ::std::os::raw::c_uint = 3;
pub const JVM_OPC_iconst_1: ::std::os::raw::c_uint = 4;
pub const JVM_OPC_iconst_2: ::std::os::raw::c_uint = 5;
pub const JVM_OPC_iconst_3: ::std::os::raw::c_uint = 6;
pub const JVM_OPC_iconst_4: ::std::os::raw::c_uint = 7;
pub const JVM_OPC_iconst_5: ::std::os::raw::c_uint = 8;
pub const JVM_OPC_lconst_0: ::std::os::raw::c_uint = 9;
pub const JVM_OPC_lconst_1: ::std::os::raw::c_uint = 10;
pub const JVM_OPC_fconst_0: ::std::os::raw::c_uint = 11;
pub const JVM_OPC_fconst_1: ::std::os::raw::c_uint = 12;
pub const JVM_OPC_fconst_2: ::std::os::raw::c_uint = 13;
pub const JVM_OPC_dconst_0: ::std::os::raw::c_uint = 14;
pub const JVM_OPC_dconst_1: ::std::os::raw::c_uint = 15;
pub const JVM_OPC_bipush: ::std::os::raw::c_uint = 16;
pub const JVM_OPC_sipush: ::std::os::raw::c_uint = 17;
pub const JVM_OPC_ldc: ::std::os::raw::c_uint = 18;
pub const JVM_OPC_ldc_w: ::std::os::raw::c_uint = 19;
pub const JVM_OPC_ldc2_w: ::std::os::raw::c_uint = 20;
pub const JVM_OPC_iload: ::std::os::raw::c_uint = 21;
pub const JVM_OPC_lload: ::std::os::raw::c_uint = 22;
pub const JVM_OPC_fload: ::std::os::raw::c_uint = 23;
pub const JVM_OPC_dload: ::std::os::raw::c_uint = 24;
pub const JVM_OPC_aload: ::std::os::raw::c_uint = 25;
pub const JVM_OPC_iload_0: ::std::os::raw::c_uint = 26;
pub const JVM_OPC_iload_1: ::std::os::raw::c_uint = 27;
pub const JVM_OPC_iload_2: ::std::os::raw::c_uint = 28;
pub const JVM_OPC_iload_3: ::std::os::raw::c_uint = 29;
pub const JVM_OPC_lload_0: ::std::os::raw::c_uint = 30;
pub const JVM_OPC_lload_1: ::std::os::raw::c_uint = 31;
pub const JVM_OPC_lload_2: ::std::os::raw::c_uint = 32;
pub const JVM_OPC_lload_3: ::std::os::raw::c_uint = 33;
pub const JVM_OPC_fload_0: ::std::os::raw::c_uint = 34;
pub const JVM_OPC_fload_1: ::std::os::raw::c_uint = 35;
pub const JVM_OPC_fload_2: ::std::os::raw::c_uint = 36;
pub const JVM_OPC_fload_3: ::std::os::raw::c_uint = 37;
pub const JVM_OPC_dload_0: ::std::os::raw::c_uint = 38;
pub const JVM_OPC_dload_1: ::std::os::raw::c_uint = 39;
pub const JVM_OPC_dload_2: ::std::os::raw::c_uint = 40;
pub const JVM_OPC_dload_3: ::std::os::raw::c_uint = 41;
pub const JVM_OPC_aload_0: ::std::os::raw::c_uint = 42;
pub const JVM_OPC_aload_1: ::std::os::raw::c_uint = 43;
pub const JVM_OPC_aload_2: ::std::os::raw::c_uint = 44;
pub const JVM_OPC_aload_3: ::std::os::raw::c_uint = 45;
pub const JVM_OPC_iaload: ::std::os::raw::c_uint = 46;
pub const JVM_OPC_laload: ::std::os::raw::c_uint = 47;
pub const JVM_OPC_faload: ::std::os::raw::c_uint = 48;
pub const JVM_OPC_daload: ::std::os::raw::c_uint = 49;
pub const JVM_OPC_aaload: ::std::os::raw::c_uint = 50;
pub const JVM_OPC_baload: ::std::os::raw::c_uint = 51;
pub const JVM_OPC_caload: ::std::os::raw::c_uint = 52;
pub const JVM_OPC_saload: ::std::os::raw::c_uint = 53;
pub const JVM_OPC_istore: ::std::os::raw::c_uint = 54;
pub const JVM_OPC_lstore: ::std::os::raw::c_uint = 55;
pub const JVM_OPC_fstore: ::std::os::raw::c_uint = 56;
pub const JVM_OPC_dstore: ::std::os::raw::c_uint = 57;
pub const JVM_OPC_astore: ::std::os::raw::c_uint = 58;
pub const JVM_OPC_istore_0: ::std::os::raw::c_uint = 59;
pub const JVM_OPC_istore_1: ::std::os::raw::c_uint = 60;
pub const JVM_OPC_istore_2: ::std::os::raw::c_uint = 61;
pub const JVM_OPC_istore_3: ::std::os::raw::c_uint = 62;
pub const JVM_OPC_lstore_0: ::std::os::raw::c_uint = 63;
pub const JVM_OPC_lstore_1: ::std::os::raw::c_uint = 64;
pub const JVM_OPC_lstore_2: ::std::os::raw::c_uint = 65;
pub const JVM_OPC_lstore_3: ::std::os::raw::c_uint = 66;
pub const JVM_OPC_fstore_0: ::std::os::raw::c_uint = 67;
pub const JVM_OPC_fstore_1: ::std::os::raw::c_uint = 68;
pub const JVM_OPC_fstore_2: ::std::os::raw::c_uint = 69;
pub const JVM_OPC_fstore_3: ::std::os::raw::c_uint = 70;
pub const JVM_OPC_dstore_0: ::std::os::raw::c_uint = 71;
pub const JVM_OPC_dstore_1: ::std::os::raw::c_uint = 72;
pub const JVM_OPC_dstore_2: ::std::os::raw::c_uint = 73;
pub const JVM_OPC_dstore_3: ::std::os::raw::c_uint = 74;
pub const JVM_OPC_astore_0: ::std::os::raw::c_uint = 75;
pub const JVM_OPC_astore_1: ::std::os::raw::c_uint = 76;
pub const JVM_OPC_astore_2: ::std::os::raw::c_uint = 77;
pub const JVM_OPC_astore_3: ::std::os::raw::c_uint = 78;
pub const JVM_OPC_iastore: ::std::os::raw::c_uint = 79;
pub const JVM_OPC_lastore: ::std::os::raw::c_uint = 80;
pub const JVM_OPC_fastore: ::std::os::raw::c_uint = 81;
pub const JVM_OPC_dastore: ::std::os::raw::c_uint = 82;
pub const JVM_OPC_aastore: ::std::os::raw::c_uint = 83;
pub const JVM_OPC_bastore: ::std::os::raw::c_uint = 84;
pub const JVM_OPC_castore: ::std::os::raw::c_uint = 85;
pub const JVM_OPC_sastore: ::std::os::raw::c_uint = 86;
pub const JVM_OPC_pop: ::std::os::raw::c_uint = 87;
pub const JVM_OPC_pop2: ::std::os::raw::c_uint = 88;
pub const JVM_OPC_dup: ::std::os::raw::c_uint = 89;
pub const JVM_OPC_dup_x1: ::std::os::raw::c_uint = 90;
pub const JVM_OPC_dup_x2: ::std::os::raw::c_uint = 91;
pub const JVM_OPC_dup2: ::std::os::raw::c_uint = 92;
pub const JVM_OPC_dup2_x1: ::std::os::raw::c_uint = 93;
pub const JVM_OPC_dup2_x2: ::std::os::raw::c_uint = 94;
pub const JVM_OPC_swap: ::std::os::raw::c_uint = 95;
pub const JVM_OPC_iadd: ::std::os::raw::c_uint = 96;
pub const JVM_OPC_ladd: ::std::os::raw::c_uint = 97;
pub const JVM_OPC_fadd: ::std::os::raw::c_uint = 98;
pub const JVM_OPC_dadd: ::std::os::raw::c_uint = 99;
pub const JVM_OPC_isub: ::std::os::raw::c_uint = 100;
pub const JVM_OPC_lsub: ::std::os::raw::c_uint = 101;
pub const JVM_OPC_fsub: ::std::os::raw::c_uint = 102;
pub const JVM_OPC_dsub: ::std::os::raw::c_uint = 103;
pub const JVM_OPC_imul: ::std::os::raw::c_uint = 104;
pub const JVM_OPC_lmul: ::std::os::raw::c_uint = 105;
pub const JVM_OPC_fmul: ::std::os::raw::c_uint = 106;
pub const JVM_OPC_dmul: ::std::os::raw::c_uint = 107;
pub const JVM_OPC_idiv: ::std::os::raw::c_uint = 108;
pub const JVM_OPC_ldiv: ::std::os::raw::c_uint = 109;
pub const JVM_OPC_fdiv: ::std::os::raw::c_uint = 110;
pub const JVM_OPC_ddiv: ::std::os::raw::c_uint = 111;
pub const JVM_OPC_irem: ::std::os::raw::c_uint = 112;
pub const JVM_OPC_lrem: ::std::os::raw::c_uint = 113;
pub const JVM_OPC_frem: ::std::os::raw::c_uint = 114;
pub const JVM_OPC_drem: ::std::os::raw::c_uint = 115;
pub const JVM_OPC_ineg: ::std::os::raw::c_uint = 116;
pub const JVM_OPC_lneg: ::std::os::raw::c_uint = 117;
pub const JVM_OPC_fneg: ::std::os::raw::c_uint = 118;
pub const JVM_OPC_dneg: ::std::os::raw::c_uint = 119;
pub const JVM_OPC_ishl: ::std::os::raw::c_uint = 120;
pub const JVM_OPC_lshl: ::std::os::raw::c_uint = 121;
pub const JVM_OPC_ishr: ::std::os::raw::c_uint = 122;
pub const JVM_OPC_lshr: ::std::os::raw::c_uint = 123;
pub const JVM_OPC_iushr: ::std::os::raw::c_uint = 124;
pub const JVM_OPC_lushr: ::std::os::raw::c_uint = 125;
pub const JVM_OPC_iand: ::std::os::raw::c_uint = 126;
pub const JVM_OPC_land: ::std::os::raw::c_uint = 127;
pub const JVM_OPC_ior: ::std::os::raw::c_uint = 128;
pub const JVM_OPC_lor: ::std::os::raw::c_uint = 129;
pub const JVM_OPC_ixor: ::std::os::raw::c_uint = 130;
pub const JVM_OPC_lxor: ::std::os::raw::c_uint = 131;
pub const JVM_OPC_iinc: ::std::os::raw::c_uint = 132;
pub const JVM_OPC_i2l: ::std::os::raw::c_uint = 133;
pub const JVM_OPC_i2f: ::std::os::raw::c_uint = 134;
pub const JVM_OPC_i2d: ::std::os::raw::c_uint = 135;
pub const JVM_OPC_l2i: ::std::os::raw::c_uint = 136;
pub const JVM_OPC_l2f: ::std::os::raw::c_uint = 137;
pub const JVM_OPC_l2d: ::std::os::raw::c_uint = 138;
pub const JVM_OPC_f2i: ::std::os::raw::c_uint = 139;
pub const JVM_OPC_f2l: ::std::os::raw::c_uint = 140;
pub const JVM_OPC_f2d: ::std::os::raw::c_uint = 141;
pub const JVM_OPC_d2i: ::std::os::raw::c_uint = 142;
pub const JVM_OPC_d2l: ::std::os::raw::c_uint = 143;
pub const JVM_OPC_d2f: ::std::os::raw::c_uint = 144;
pub const JVM_OPC_i2b: ::std::os::raw::c_uint = 145;
pub const JVM_OPC_i2c: ::std::os::raw::c_uint = 146;
pub const JVM_OPC_i2s: ::std::os::raw::c_uint = 147;
pub const JVM_OPC_lcmp: ::std::os::raw::c_uint = 148;
pub const JVM_OPC_fcmpl: ::std::os::raw::c_uint = 149;
pub const JVM_OPC_fcmpg: ::std::os::raw::c_uint = 150;
pub const JVM_OPC_dcmpl: ::std::os::raw::c_uint = 151;
pub const JVM_OPC_dcmpg: ::std::os::raw::c_uint = 152;
pub const JVM_OPC_ifeq: ::std::os::raw::c_uint = 153;
pub const JVM_OPC_ifne: ::std::os::raw::c_uint = 154;
pub const JVM_OPC_iflt: ::std::os::raw::c_uint = 155;
pub const JVM_OPC_ifge: ::std::os::raw::c_uint = 156;
pub const JVM_OPC_ifgt: ::std::os::raw::c_uint = 157;
pub const JVM_OPC_ifle: ::std::os::raw::c_uint = 158;
pub const JVM_OPC_if_icmpeq: ::std::os::raw::c_uint = 159;
pub const JVM_OPC_if_icmpne: ::std::os::raw::c_uint = 160;
pub const JVM_OPC_if_icmplt: ::std::os::raw::c_uint = 161;
pub const JVM_OPC_if_icmpge: ::std::os::raw::c_uint = 162;
pub const JVM_OPC_if_icmpgt: ::std::os::raw::c_uint = 163;
pub const JVM_OPC_if_icmple: ::std::os::raw::c_uint = 164;
pub const JVM_OPC_if_acmpeq: ::std::os::raw::c_uint = 165;
pub const JVM_OPC_if_acmpne: ::std::os::raw::c_uint = 166;
pub const JVM_OPC_goto: ::std::os::raw::c_uint = 167;
pub const JVM_OPC_jsr: ::std::os::raw::c_uint = 168;
pub const JVM_OPC_ret: ::std::os::raw::c_uint = 169;
pub const JVM_OPC_tableswitch: ::std::os::raw::c_uint = 170;
pub const JVM_OPC_lookupswitch: ::std::os::raw::c_uint = 171;
pub const JVM_OPC_ireturn: ::std::os::raw::c_uint = 172;
pub const JVM_OPC_lreturn: ::std::os::raw::c_uint = 173;
pub const JVM_OPC_freturn: ::std::os::raw::c_uint = 174;
pub const JVM_OPC_dreturn: ::std::os::raw::c_uint = 175;
pub const JVM_OPC_areturn: ::std::os::raw::c_uint = 176;
pub const JVM_OPC_return: ::std::os::raw::c_uint = 177;
pub const JVM_OPC_getstatic: ::std::os::raw::c_uint = 178;
pub const JVM_OPC_putstatic: ::std::os::raw::c_uint = 179;
pub const JVM_OPC_getfield: ::std::os::raw::c_uint = 180;
pub const JVM_OPC_putfield: ::std::os::raw::c_uint = 181;
pub const JVM_OPC_invokevirtual: ::std::os::raw::c_uint = 182;
pub const JVM_OPC_invokespecial: ::std::os::raw::c_uint = 183;
pub const JVM_OPC_invokestatic: ::std::os::raw::c_uint = 184;
pub const JVM_OPC_invokeinterface: ::std::os::raw::c_uint = 185;
pub const JVM_OPC_invokedynamic: ::std::os::raw::c_uint = 186;
pub const JVM_OPC_new: ::std::os::raw::c_uint = 187;
pub const JVM_OPC_newarray: ::std::os::raw::c_uint = 188;
pub const JVM_OPC_anewarray: ::std::os::raw::c_uint = 189;
pub const JVM_OPC_arraylength: ::std::os::raw::c_uint = 190;
pub const JVM_OPC_athrow: ::std::os::raw::c_uint = 191;
pub const JVM_OPC_checkcast: ::std::os::raw::c_uint = 192;
pub const JVM_OPC_instanceof: ::std::os::raw::c_uint = 193;
pub const JVM_OPC_monitorenter: ::std::os::raw::c_uint = 194;
pub const JVM_OPC_monitorexit: ::std::os::raw::c_uint = 195;
pub const JVM_OPC_wide: ::std::os::raw::c_uint = 196;
pub const JVM_OPC_multianewarray: ::std::os::raw::c_uint = 197;
pub const JVM_OPC_ifnull: ::std::os::raw::c_uint = 198;
pub const JVM_OPC_ifnonnull: ::std::os::raw::c_uint = 199;
pub const JVM_OPC_goto_w: ::std::os::raw::c_uint = 200;
pub const JVM_OPC_jsr_w: ::std::os::raw::c_uint = 201;
pub const JVM_OPC_MAX: ::std::os::raw::c_uint = 201;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct method_size_info {
	pub code: ::std::os::raw::c_ulong,
	pub excs: ::std::os::raw::c_ulong,
	pub etab: ::std::os::raw::c_ulong,
	pub lnum: ::std::os::raw::c_ulong,
	pub lvar: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct class_size_info {
	pub constants: ::std::os::raw::c_uint,
	pub fields: ::std::os::raw::c_uint,
	pub methods: ::std::os::raw::c_uint,
	pub interfaces: ::std::os::raw::c_uint,
	pub fields2: ::std::os::raw::c_uint,
	pub innerclasses: ::std::os::raw::c_uint,
	pub clinit: method_size_info,
	pub main: method_size_info,
}
extern "C" {
	#[doc = "PART 3: I/O and Network Support"]
	pub fn JVM_NativePath(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
pub static JVM_RawMonitorCreate: Weak<fn() -> *mut ::std::os::raw::c_void> = Weak::new("JVM_RawMonitorCreate");
pub static JVM_RawMonitorDestroy: Weak<fn(mon: *mut ::std::os::raw::c_void)> = Weak::new("JVM_RawMonitorDestroy");
pub static JVM_RawMonitorEnter: Weak<fn(mon: *mut ::std::os::raw::c_void) -> jint> = Weak::new("JVM_RawMonitorEnter");
pub static JVM_RawMonitorExit: Weak<fn(mon: *mut ::std::os::raw::c_void)> = Weak::new("JVM_RawMonitorExit");
pub static JVM_GetManagement: Weak<fn(version: jint) -> *mut ::std::os::raw::c_void> = Weak::new("JVM_GetManagement");
pub static JVM_InitAgentProperties: Weak<fn(env: *mut JNIEnv, agent_props: jobject) -> jobject> = Weak::new("JVM_InitAgentProperties");
pub static JVM_GetTemporaryDirectory: Weak<fn(env: *mut JNIEnv) -> jstring> = Weak::new("JVM_GetTemporaryDirectory");
pub static JVM_GetEnclosingMethodInfo: Weak<fn(env: *mut JNIEnv, ofClass: jclass) -> jobjectArray> = Weak::new("JVM_GetEnclosingMethodInfo");
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JDK1_1InitArgs {
	pub version: jint,
	pub properties: *mut *mut ::std::os::raw::c_char,
	pub checkSource: jint,
	pub nativeStackSize: jint,
	pub javaStackSize: jint,
	pub minHeapSize: jint,
	pub maxHeapSize: jint,
	pub verifyMode: jint,
	pub classpath: *mut ::std::os::raw::c_char,
	pub vfprintf: ::std::option::Option<
		unsafe extern "C" fn(
			fp: *mut FILE,
			format: *const ::std::os::raw::c_char,
			args: *mut __va_list_tag,
		) -> jint,
	>,
	pub exit: ::std::option::Option<unsafe extern "C" fn(code: jint)>,
	pub abort: ::std::option::Option<unsafe extern "C" fn()>,
	pub enableClassGC: jint,
	pub enableVerboseGC: jint,
	pub disableAsyncGC: jint,
	pub verbose: jint,
	pub debugging: jboolean,
	pub debugPort: jint,
}
