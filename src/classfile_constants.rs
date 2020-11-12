#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub const JVM_CLASSFILE_MAJOR_VERSION: u32 = 59;
pub const JVM_CLASSFILE_MINOR_VERSION: u32 = 0;
pub const JVM_ACC_PUBLIC_BIT: u32 = 0;
pub const JVM_ACC_PRIVATE_BIT: u32 = 1;
pub const JVM_ACC_PROTECTED_BIT: u32 = 2;
pub const JVM_ACC_STATIC_BIT: u32 = 3;
pub const JVM_ACC_FINAL_BIT: u32 = 4;
pub const JVM_ACC_SYNCHRONIZED_BIT: u32 = 5;
pub const JVM_ACC_SUPER_BIT: u32 = 5;
pub const JVM_ACC_VOLATILE_BIT: u32 = 6;
pub const JVM_ACC_BRIDGE_BIT: u32 = 6;
pub const JVM_ACC_TRANSIENT_BIT: u32 = 7;
pub const JVM_ACC_VARARGS_BIT: u32 = 7;
pub const JVM_ACC_NATIVE_BIT: u32 = 8;
pub const JVM_ACC_INTERFACE_BIT: u32 = 9;
pub const JVM_ACC_ABSTRACT_BIT: u32 = 10;
pub const JVM_ACC_STRICT_BIT: u32 = 11;
pub const JVM_ACC_SYNTHETIC_BIT: u32 = 12;
pub const JVM_ACC_ANNOTATION_BIT: u32 = 13;
pub const JVM_ACC_ENUM_BIT: u32 = 14;
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