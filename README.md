# jvm-rs

JVM bindings for rust.

Currently bound to JVM 15.

Generated with `bindgen --no-layout-tests`

## Usage

Add to Cargo.toml
```toml
jvm-rs = "0.1.0"
```

Follow documentation: https://docs.rs/jvm-rs/0.1.0/jvm_rs/

## Interfaces this supports
* [classfile_constants](https://github.com/openjdk/jdk/blob/jdk-16+24/src/java.base/share/native/include/classfile_constants.h.template)
* [jawt](https://github.com/openjdk/jdk/blob/jdk-16+24/src/java.desktop/share/native/include/jawt.h)
* [jdwp](https://github.com/openjdk/jdk/blob/jdk-16+24/src/jdk.jdwp.agent/share/native/include/jdwpTransport.h)
* [jmm](https://github.com/openjdk/jdk/blob/jdk-16+24/src/hotspot/share/include/jmm.h)
* [jni](https://github.com/openjdk/jdk/blob/jdk-16+24/src/java.base/share/native/include/jni.h)
* [jvm](https://github.com/openjdk/jdk/blob/jdk-16+24/src/hotspot/share/include/jvm.h)
* [jvmti](https://github.com/openjdk/jdk/blob/jdk-16+24/src/hotspot/share/prims/jvmtiH.xsl)
* [jvmticmlr](https://github.com/openjdk/jdk/blob/jdk-16+24/src/java.base/share/native/include/jvmticmlr.h)
