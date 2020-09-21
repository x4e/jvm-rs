#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]


pub const JNI_FALSE: u32 = 0;
pub const JNI_TRUE: u32 = 1;
pub const JNI_OK: u32 = 0;
pub const JNI_ERR: i32 = -1;
pub const JNI_EDETACHED: i32 = -2;
pub const JNI_EVERSION: i32 = -3;
pub const JNI_ENOMEM: i32 = -4;
pub const JNI_EEXIST: i32 = -5;
pub const JNI_EINVAL: i32 = -6;
pub const JNI_COMMIT: u32 = 1;
pub const JNI_ABORT: u32 = 2;
pub const JNI_VERSION_1_1: u32 = 65537;
pub const JNI_VERSION_1_2: u32 = 65538;
pub const JNI_VERSION_1_4: u32 = 65540;
pub const JNI_VERSION_1_6: u32 = 65542;
pub const JNI_VERSION_1_8: u32 = 65544;

pub type jint = ::std::os::raw::c_int;
pub type jlong = ::std::os::raw::c_long;
pub type jbyte = ::std::os::raw::c_schar;
pub type jboolean = ::std::os::raw::c_uchar;
pub type jchar = ::std::os::raw::c_ushort;
pub type jshort = ::std::os::raw::c_short;
pub type jfloat = f32;
pub type jdouble = f64;
pub type jsize = jint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jobject {
    _unused: [u8; 0],
}
pub type jobject = *mut _jobject;
pub type jclass = jobject;
pub type jthrowable = jobject;
pub type jstring = jobject;
pub type jarray = jobject;
pub type jbooleanArray = jarray;
pub type jbyteArray = jarray;
pub type jcharArray = jarray;
pub type jshortArray = jarray;
pub type jintArray = jarray;
pub type jlongArray = jarray;
pub type jfloatArray = jarray;
pub type jdoubleArray = jarray;
pub type jobjectArray = jarray;
pub type jweak = jobject;
#[repr(C)]
#[derive(Copy, Clone)]
pub union jvalue {
    pub z: jboolean,
    pub b: jbyte,
    pub c: jchar,
    pub s: jshort,
    pub i: jint,
    pub j: jlong,
    pub f: jfloat,
    pub d: jdouble,
    pub l: jobject,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jfieldID {
    _unused: [u8; 0],
}
pub type jfieldID = *mut _jfieldID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jmethodID {
    _unused: [u8; 0],
}
pub type jmethodID = *mut _jmethodID;
pub const _jobjectType_JNIInvalidRefType: _jobjectType = 0;
pub const _jobjectType_JNILocalRefType: _jobjectType = 1;
pub const _jobjectType_JNIGlobalRefType: _jobjectType = 2;
pub const _jobjectType_JNIWeakGlobalRefType: _jobjectType = 3;
pub type _jobjectType = ::std::os::raw::c_uint;
pub use self::_jobjectType as jobjectRefType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JNINativeMethod {
    pub name: *mut ::std::os::raw::c_char,
    pub signature: *mut ::std::os::raw::c_char,
    pub fnPtr: *mut ::std::os::raw::c_void,
}
pub type JNIEnv = *const JNINativeInterface_;
pub type JavaVM = *const JNIInvokeInterface_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JNINativeInterface_ {
    pub reserved0: *mut ::std::os::raw::c_void,
    pub reserved1: *mut ::std::os::raw::c_void,
    pub reserved2: *mut ::std::os::raw::c_void,
    pub reserved3: *mut ::std::os::raw::c_void,
    pub GetVersion: ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv) -> jint>,
    pub DefineClass: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            name: *const ::std::os::raw::c_char,
            loader: jobject,
            buf: *const jbyte,
            len: jsize,
        ) -> jclass,
    >,
    pub FindClass: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, name: *const ::std::os::raw::c_char) -> jclass,
    >,
    pub FromReflectedMethod:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, method: jobject) -> jmethodID>,
    pub FromReflectedField:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, field: jobject) -> jfieldID>,
    pub ToReflectedMethod: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            cls: jclass,
            methodID: jmethodID,
            isStatic: jboolean,
        ) -> jobject,
    >,
    pub GetSuperclass:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, sub: jclass) -> jclass>,
    pub IsAssignableFrom: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, sub: jclass, sup: jclass) -> jboolean,
    >,
    pub ToReflectedField: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            cls: jclass,
            fieldID: jfieldID,
            isStatic: jboolean,
        ) -> jobject,
    >,
    pub Throw:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, obj: jthrowable) -> jint>,
    pub ThrowNew: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            msg: *const ::std::os::raw::c_char,
        ) -> jint,
    >,
    pub ExceptionOccurred:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv) -> jthrowable>,
    pub ExceptionDescribe: ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv)>,
    pub ExceptionClear: ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv)>,
    pub FatalError: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, msg: *const ::std::os::raw::c_char),
    >,
    pub PushLocalFrame:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, capacity: jint) -> jint>,
    pub PopLocalFrame:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, result: jobject) -> jobject>,
    pub NewGlobalRef:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, lobj: jobject) -> jobject>,
    pub DeleteGlobalRef:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, gref: jobject)>,
    pub DeleteLocalRef: ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject)>,
    pub IsSameObject: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj1: jobject, obj2: jobject) -> jboolean,
    >,
    pub NewLocalRef:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, ref_: jobject) -> jobject>,
    pub EnsureLocalCapacity:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, capacity: jint) -> jint>,
    pub AllocObject:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass) -> jobject>,
    pub NewObject: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jobject,
    >,
    pub NewObjectV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jobject,
    >,
    pub NewObjectA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jobject,
    >,
    pub GetObjectClass:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject) -> jclass>,
    pub IsInstanceOf: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, clazz: jclass) -> jboolean,
    >,
    pub GetMethodID: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            name: *const ::std::os::raw::c_char,
            sig: *const ::std::os::raw::c_char,
        ) -> jmethodID,
    >,
    pub CallObjectMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jobject,
    >,
    pub CallObjectMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jobject,
    >,
    pub CallObjectMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jobject,
    >,
    pub CallBooleanMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jboolean,
    >,
    pub CallBooleanMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jboolean,
    >,
    pub CallBooleanMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jboolean,
    >,
    pub CallByteMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jbyte,
    >,
    pub CallByteMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jbyte,
    >,
    pub CallByteMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jbyte,
    >,
    pub CallCharMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jchar,
    >,
    pub CallCharMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jchar,
    >,
    pub CallCharMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jchar,
    >,
    pub CallShortMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jshort,
    >,
    pub CallShortMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jshort,
    >,
    pub CallShortMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jshort,
    >,
    pub CallIntMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jint,
    >,
    pub CallIntMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jint,
    >,
    pub CallIntMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jint,
    >,
    pub CallLongMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jlong,
    >,
    pub CallLongMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jlong,
    >,
    pub CallLongMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jlong,
    >,
    pub CallFloatMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jfloat,
    >,
    pub CallFloatMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jfloat,
    >,
    pub CallFloatMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jfloat,
    >,
    pub CallDoubleMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jdouble,
    >,
    pub CallDoubleMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jdouble,
    >,
    pub CallDoubleMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jdouble,
    >,
    pub CallVoidMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...),
    >,
    pub CallVoidMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ),
    >,
    pub CallVoidMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            methodID: jmethodID,
            args: *const jvalue,
        ),
    >,
    pub CallNonvirtualObjectMethod: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            ...
        ) -> jobject,
    >,
    pub CallNonvirtualObjectMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jobject,
    >,
    pub CallNonvirtualObjectMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jobject,
    >,
    pub CallNonvirtualBooleanMethod: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            ...
        ) -> jboolean,
    >,
    pub CallNonvirtualBooleanMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jboolean,
    >,
    pub CallNonvirtualBooleanMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jboolean,
    >,
    pub CallNonvirtualByteMethod: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            ...
        ) -> jbyte,
    >,
    pub CallNonvirtualByteMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jbyte,
    >,
    pub CallNonvirtualByteMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jbyte,
    >,
    pub CallNonvirtualCharMethod: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            ...
        ) -> jchar,
    >,
    pub CallNonvirtualCharMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jchar,
    >,
    pub CallNonvirtualCharMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jchar,
    >,
    pub CallNonvirtualShortMethod: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            ...
        ) -> jshort,
    >,
    pub CallNonvirtualShortMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jshort,
    >,
    pub CallNonvirtualShortMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jshort,
    >,
    pub CallNonvirtualIntMethod: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            ...
        ) -> jint,
    >,
    pub CallNonvirtualIntMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jint,
    >,
    pub CallNonvirtualIntMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jint,
    >,
    pub CallNonvirtualLongMethod: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            ...
        ) -> jlong,
    >,
    pub CallNonvirtualLongMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jlong,
    >,
    pub CallNonvirtualLongMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jlong,
    >,
    pub CallNonvirtualFloatMethod: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            ...
        ) -> jfloat,
    >,
    pub CallNonvirtualFloatMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jfloat,
    >,
    pub CallNonvirtualFloatMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jfloat,
    >,
    pub CallNonvirtualDoubleMethod: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            ...
        ) -> jdouble,
    >,
    pub CallNonvirtualDoubleMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jdouble,
    >,
    pub CallNonvirtualDoubleMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jdouble,
    >,
    pub CallNonvirtualVoidMethod: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            ...
        ),
    >,
    pub CallNonvirtualVoidMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ),
    >,
    pub CallNonvirtualVoidMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            obj: jobject,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ),
    >,
    pub GetFieldID: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            name: *const ::std::os::raw::c_char,
            sig: *const ::std::os::raw::c_char,
        ) -> jfieldID,
    >,
    pub GetObjectField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jobject,
    >,
    pub GetBooleanField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jboolean,
    >,
    pub GetByteField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jbyte,
    >,
    pub GetCharField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jchar,
    >,
    pub GetShortField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jshort,
    >,
    pub GetIntField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jint,
    >,
    pub GetLongField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jlong,
    >,
    pub GetFloatField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jfloat,
    >,
    pub GetDoubleField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jdouble,
    >,
    pub SetObjectField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jobject),
    >,
    pub SetBooleanField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jboolean),
    >,
    pub SetByteField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jbyte),
    >,
    pub SetCharField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jchar),
    >,
    pub SetShortField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jshort),
    >,
    pub SetIntField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jint),
    >,
    pub SetLongField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jlong),
    >,
    pub SetFloatField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jfloat),
    >,
    pub SetDoubleField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jdouble),
    >,
    pub GetStaticMethodID: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            name: *const ::std::os::raw::c_char,
            sig: *const ::std::os::raw::c_char,
        ) -> jmethodID,
    >,
    pub CallStaticObjectMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jobject,
    >,
    pub CallStaticObjectMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jobject,
    >,
    pub CallStaticObjectMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jobject,
    >,
    pub CallStaticBooleanMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jboolean,
    >,
    pub CallStaticBooleanMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jboolean,
    >,
    pub CallStaticBooleanMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jboolean,
    >,
    pub CallStaticByteMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jbyte,
    >,
    pub CallStaticByteMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jbyte,
    >,
    pub CallStaticByteMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jbyte,
    >,
    pub CallStaticCharMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jchar,
    >,
    pub CallStaticCharMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jchar,
    >,
    pub CallStaticCharMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jchar,
    >,
    pub CallStaticShortMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jshort,
    >,
    pub CallStaticShortMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jshort,
    >,
    pub CallStaticShortMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jshort,
    >,
    pub CallStaticIntMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jint,
    >,
    pub CallStaticIntMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jint,
    >,
    pub CallStaticIntMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jint,
    >,
    pub CallStaticLongMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jlong,
    >,
    pub CallStaticLongMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jlong,
    >,
    pub CallStaticLongMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jlong,
    >,
    pub CallStaticFloatMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jfloat,
    >,
    pub CallStaticFloatMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jfloat,
    >,
    pub CallStaticFloatMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jfloat,
    >,
    pub CallStaticDoubleMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, methodID: jmethodID, ...) -> jdouble,
    >,
    pub CallStaticDoubleMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ) -> jdouble,
    >,
    pub CallStaticDoubleMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ) -> jdouble,
    >,
    pub CallStaticVoidMethod: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, cls: jclass, methodID: jmethodID, ...),
    >,
    pub CallStaticVoidMethodV: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            cls: jclass,
            methodID: jmethodID,
            args: *mut __va_list_tag,
        ),
    >,
    pub CallStaticVoidMethodA: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            cls: jclass,
            methodID: jmethodID,
            args: *const jvalue,
        ),
    >,
    pub GetStaticFieldID: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            name: *const ::std::os::raw::c_char,
            sig: *const ::std::os::raw::c_char,
        ) -> jfieldID,
    >,
    pub GetStaticObjectField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jobject,
    >,
    pub GetStaticBooleanField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jboolean,
    >,
    pub GetStaticByteField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jbyte,
    >,
    pub GetStaticCharField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jchar,
    >,
    pub GetStaticShortField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jshort,
    >,
    pub GetStaticIntField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jint,
    >,
    pub GetStaticLongField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jlong,
    >,
    pub GetStaticFloatField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jfloat,
    >,
    pub GetStaticDoubleField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID) -> jdouble,
    >,
    pub SetStaticObjectField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jobject),
    >,
    pub SetStaticBooleanField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jboolean),
    >,
    pub SetStaticByteField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jbyte),
    >,
    pub SetStaticCharField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jchar),
    >,
    pub SetStaticShortField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jshort),
    >,
    pub SetStaticIntField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jint),
    >,
    pub SetStaticLongField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jlong),
    >,
    pub SetStaticFloatField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jfloat),
    >,
    pub SetStaticDoubleField: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass, fieldID: jfieldID, value: jdouble),
    >,
    pub NewString: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, unicode: *const jchar, len: jsize) -> jstring,
    >,
    pub GetStringLength:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, str_: jstring) -> jsize>,
    pub GetStringChars: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            str_: jstring,
            isCopy: *mut jboolean,
        ) -> *const jchar,
    >,
    pub ReleaseStringChars: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, str_: jstring, chars: *const jchar),
    >,
    pub NewStringUTF: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, utf: *const ::std::os::raw::c_char) -> jstring,
    >,
    pub GetStringUTFLength:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, str_: jstring) -> jsize>,
    pub GetStringUTFChars: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            str_: jstring,
            isCopy: *mut jboolean,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub ReleaseStringUTFChars: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, str_: jstring, chars: *const ::std::os::raw::c_char),
    >,
    pub GetArrayLength:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, array: jarray) -> jsize>,
    pub NewObjectArray: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            len: jsize,
            clazz: jclass,
            init: jobject,
        ) -> jobjectArray,
    >,
    pub GetObjectArrayElement: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, array: jobjectArray, index: jsize) -> jobject,
    >,
    pub SetObjectArrayElement: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, array: jobjectArray, index: jsize, val: jobject),
    >,
    pub NewBooleanArray:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, len: jsize) -> jbooleanArray>,
    pub NewByteArray:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, len: jsize) -> jbyteArray>,
    pub NewCharArray:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, len: jsize) -> jcharArray>,
    pub NewShortArray:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, len: jsize) -> jshortArray>,
    pub NewIntArray:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, len: jsize) -> jintArray>,
    pub NewLongArray:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, len: jsize) -> jlongArray>,
    pub NewFloatArray:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, len: jsize) -> jfloatArray>,
    pub NewDoubleArray:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, len: jsize) -> jdoubleArray>,
    pub GetBooleanArrayElements: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jbooleanArray,
            isCopy: *mut jboolean,
        ) -> *mut jboolean,
    >,
    pub GetByteArrayElements: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jbyteArray,
            isCopy: *mut jboolean,
        ) -> *mut jbyte,
    >,
    pub GetCharArrayElements: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jcharArray,
            isCopy: *mut jboolean,
        ) -> *mut jchar,
    >,
    pub GetShortArrayElements: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jshortArray,
            isCopy: *mut jboolean,
        ) -> *mut jshort,
    >,
    pub GetIntArrayElements: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jintArray,
            isCopy: *mut jboolean,
        ) -> *mut jint,
    >,
    pub GetLongArrayElements: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jlongArray,
            isCopy: *mut jboolean,
        ) -> *mut jlong,
    >,
    pub GetFloatArrayElements: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jfloatArray,
            isCopy: *mut jboolean,
        ) -> *mut jfloat,
    >,
    pub GetDoubleArrayElements: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jdoubleArray,
            isCopy: *mut jboolean,
        ) -> *mut jdouble,
    >,
    pub ReleaseBooleanArrayElements: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jbooleanArray,
            elems: *mut jboolean,
            mode: jint,
        ),
    >,
    pub ReleaseByteArrayElements: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, array: jbyteArray, elems: *mut jbyte, mode: jint),
    >,
    pub ReleaseCharArrayElements: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, array: jcharArray, elems: *mut jchar, mode: jint),
    >,
    pub ReleaseShortArrayElements: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, array: jshortArray, elems: *mut jshort, mode: jint),
    >,
    pub ReleaseIntArrayElements: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, array: jintArray, elems: *mut jint, mode: jint),
    >,
    pub ReleaseLongArrayElements: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, array: jlongArray, elems: *mut jlong, mode: jint),
    >,
    pub ReleaseFloatArrayElements: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, array: jfloatArray, elems: *mut jfloat, mode: jint),
    >,
    pub ReleaseDoubleArrayElements: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jdoubleArray,
            elems: *mut jdouble,
            mode: jint,
        ),
    >,
    pub GetBooleanArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jbooleanArray,
            start: jsize,
            l: jsize,
            buf: *mut jboolean,
        ),
    >,
    pub GetByteArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jbyteArray,
            start: jsize,
            len: jsize,
            buf: *mut jbyte,
        ),
    >,
    pub GetCharArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jcharArray,
            start: jsize,
            len: jsize,
            buf: *mut jchar,
        ),
    >,
    pub GetShortArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jshortArray,
            start: jsize,
            len: jsize,
            buf: *mut jshort,
        ),
    >,
    pub GetIntArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jintArray,
            start: jsize,
            len: jsize,
            buf: *mut jint,
        ),
    >,
    pub GetLongArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jlongArray,
            start: jsize,
            len: jsize,
            buf: *mut jlong,
        ),
    >,
    pub GetFloatArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jfloatArray,
            start: jsize,
            len: jsize,
            buf: *mut jfloat,
        ),
    >,
    pub GetDoubleArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jdoubleArray,
            start: jsize,
            len: jsize,
            buf: *mut jdouble,
        ),
    >,
    pub SetBooleanArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jbooleanArray,
            start: jsize,
            l: jsize,
            buf: *const jboolean,
        ),
    >,
    pub SetByteArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jbyteArray,
            start: jsize,
            len: jsize,
            buf: *const jbyte,
        ),
    >,
    pub SetCharArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jcharArray,
            start: jsize,
            len: jsize,
            buf: *const jchar,
        ),
    >,
    pub SetShortArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jshortArray,
            start: jsize,
            len: jsize,
            buf: *const jshort,
        ),
    >,
    pub SetIntArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jintArray,
            start: jsize,
            len: jsize,
            buf: *const jint,
        ),
    >,
    pub SetLongArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jlongArray,
            start: jsize,
            len: jsize,
            buf: *const jlong,
        ),
    >,
    pub SetFloatArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jfloatArray,
            start: jsize,
            len: jsize,
            buf: *const jfloat,
        ),
    >,
    pub SetDoubleArrayRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jdoubleArray,
            start: jsize,
            len: jsize,
            buf: *const jdouble,
        ),
    >,
    pub RegisterNatives: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            clazz: jclass,
            methods: *const JNINativeMethod,
            nMethods: jint,
        ) -> jint,
    >,
    pub UnregisterNatives:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, clazz: jclass) -> jint>,
    pub MonitorEnter:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject) -> jint>,
    pub MonitorExit:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject) -> jint>,
    pub GetJavaVM:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint>,
    pub GetStringRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            str_: jstring,
            start: jsize,
            len: jsize,
            buf: *mut jchar,
        ),
    >,
    pub GetStringUTFRegion: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            str_: jstring,
            start: jsize,
            len: jsize,
            buf: *mut ::std::os::raw::c_char,
        ),
    >,
    pub GetPrimitiveArrayCritical: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jarray,
            isCopy: *mut jboolean,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub ReleasePrimitiveArrayCritical: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            array: jarray,
            carray: *mut ::std::os::raw::c_void,
            mode: jint,
        ),
    >,
    pub GetStringCritical: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            string: jstring,
            isCopy: *mut jboolean,
        ) -> *const jchar,
    >,
    pub ReleaseStringCritical: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, string: jstring, cstring: *const jchar),
    >,
    pub NewWeakGlobalRef:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject) -> jweak>,
    pub DeleteWeakGlobalRef:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, ref_: jweak)>,
    pub ExceptionCheck: ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv) -> jboolean>,
    pub NewDirectByteBuffer: ::std::option::Option<
        unsafe extern "C" fn(
            env: *mut JNIEnv,
            address: *mut ::std::os::raw::c_void,
            capacity: jlong,
        ) -> jobject,
    >,
    pub GetDirectBufferAddress: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, buf: jobject) -> *mut ::std::os::raw::c_void,
    >,
    pub GetDirectBufferCapacity:
        ::std::option::Option<unsafe extern "C" fn(env: *mut JNIEnv, buf: jobject) -> jlong>,
    pub GetObjectRefType: ::std::option::Option<
        unsafe extern "C" fn(env: *mut JNIEnv, obj: jobject) -> jobjectRefType,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JNIEnv_ {
    pub functions: *const JNINativeInterface_,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JavaVMOption {
    pub optionString: *mut ::std::os::raw::c_char,
    pub extraInfo: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JavaVMInitArgs {
    pub version: jint,
    pub nOptions: jint,
    pub options: *mut JavaVMOption,
    pub ignoreUnrecognized: jboolean,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JavaVMAttachArgs {
    pub version: jint,
    pub name: *mut ::std::os::raw::c_char,
    pub group: jobject,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JNIInvokeInterface_ {
    pub reserved0: *mut ::std::os::raw::c_void,
    pub reserved1: *mut ::std::os::raw::c_void,
    pub reserved2: *mut ::std::os::raw::c_void,
    pub DestroyJavaVM: ::std::option::Option<unsafe extern "C" fn(vm: *mut JavaVM) -> jint>,
    pub AttachCurrentThread: ::std::option::Option<
        unsafe extern "C" fn(
            vm: *mut JavaVM,
            penv: *mut *mut ::std::os::raw::c_void,
            args: *mut ::std::os::raw::c_void,
        ) -> jint,
    >,
    pub DetachCurrentThread: ::std::option::Option<unsafe extern "C" fn(vm: *mut JavaVM) -> jint>,
    pub GetEnv: ::std::option::Option<
        unsafe extern "C" fn(
            vm: *mut JavaVM,
            penv: *mut *mut ::std::os::raw::c_void,
            version: jint,
        ) -> jint,
    >,
    pub AttachCurrentThreadAsDaemon: ::std::option::Option<
        unsafe extern "C" fn(
            vm: *mut JavaVM,
            penv: *mut *mut ::std::os::raw::c_void,
            args: *mut ::std::os::raw::c_void,
        ) -> jint,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JavaVM_ {
    pub functions: *const JNIInvokeInterface_,
}
extern "C" {
    pub fn JNI_GetDefaultJavaVMInitArgs(args: *mut ::std::os::raw::c_void) -> jint;
}
extern "C" {
    pub fn JNI_CreateJavaVM(
        pvm: *mut *mut JavaVM,
        penv: *mut *mut ::std::os::raw::c_void,
        args: *mut ::std::os::raw::c_void,
    ) -> jint;
}
extern "C" {
    pub fn JNI_GetCreatedJavaVMs(arg1: *mut *mut JavaVM, arg2: jsize, arg3: *mut jsize) -> jint;
}
extern "C" {
    pub fn JNI_OnLoad(vm: *mut JavaVM, reserved: *mut ::std::os::raw::c_void) -> jint;
}
extern "C" {
    pub fn JNI_OnUnload(vm: *mut JavaVM, reserved: *mut ::std::os::raw::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
