use num_enum::TryFromPrimitive;
#[derive(TryFromPrimitive, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    /// Do nothing
    /// Format: nop
    Nop = 0x0,

    /// Push null
    /// Format: aconst_null
    AconstNull = 0x1,

    /// Push int constant
    /// Format: iconst_<i>
    IconstM1 = 0x2,

    /// Push int constant
    /// Format: iconst_<i>
    Iconst0 = 0x3,

    /// Push int constant
    /// Format: iconst_<i>
    Iconst1 = 0x4,

    /// Push int constant
    /// Format: iconst_<i>
    Iconst2 = 0x5,

    /// Push int constant
    /// Format: iconst_<i>
    Iconst3 = 0x6,

    /// Push int constant
    /// Format: iconst_<i>
    Iconst4 = 0x7,

    /// Push int constant
    /// Format: iconst_<i>
    Iconst5 = 0x8,

    /// Push long constant
    /// Format: lconst_<l>
    Lconst0 = 0x9,

    /// Push long constant
    /// Format: lconst_<l>
    Lconst1 = 0xa,

    /// Push float
    /// Format: fconst_<f>
    Fconst0 = 0xb,

    /// Push float
    /// Format: fconst_<f>
    Fconst1 = 0xc,

    /// Push float
    /// Format: fconst_<f>
    Fconst2 = 0xd,

    /// Push double
    /// Format: dconst_<d>
    Dconst0 = 0xe,

    /// Push double
    /// Format: dconst_<d>
    Dconst1 = 0xf,

    /// Push byte
    /// Format: bipush byte
    Bipush = 0x10,

    /// Push short
    /// Format: sipush byte1 byte2
    Sipush = 0x11,

    /// Push item from run-time constant pool
    /// Format: ldc index
    Ldc = 0x12,

    /// Push item from run-time constant pool (wide index)
    /// Format: ldc_w indexbyte1 indexbyte2
    LdcW = 0x13,

    /// Push long or double from run-time constant pool (wide index)
    /// Format: ldc2_w indexbyte1 indexbyte2
    Ldc2W = 0x14,

    /// Load int from local variable
    /// Format: iload index
    Iload = 0x15,

    /// Load long from local variable
    /// Format: lload index
    Lload = 0x16,

    /// Load float from local variable
    /// Format: fload index
    Fload = 0x17,

    /// Load double from local variable
    /// Format: dload index
    Dload = 0x18,

    /// Load reference from local variable
    /// Format: aload index
    Aload = 0x19,

    /// Load int from local variable
    /// Format: iload_<n>
    Iload0 = 0x1a,

    /// Load int from local variable
    /// Format: iload_<n>
    Iload1 = 0x1b,

    /// Load int from local variable
    /// Format: iload_<n>
    Iload2 = 0x1c,

    /// Load int from local variable
    /// Format: iload_<n>
    Iload3 = 0x1d,

    /// Load long from local variable
    /// Format: lload_<n>
    Lload0 = 0x1e,

    /// Load long from local variable
    /// Format: lload_<n>
    Lload1 = 0x1f,

    /// Load long from local variable
    /// Format: lload_<n>
    Lload2 = 0x20,

    /// Load long from local variable
    /// Format: lload_<n>
    Lload3 = 0x21,

    /// Load float from local variable
    /// Format: fload_<n>
    Fload0 = 0x22,

    /// Load float from local variable
    /// Format: fload_<n>
    Fload1 = 0x23,

    /// Load float from local variable
    /// Format: fload_<n>
    Fload2 = 0x24,

    /// Load float from local variable
    /// Format: fload_<n>
    Fload3 = 0x25,

    /// Load double from local variable
    /// Format: dload_<n>
    Dload0 = 0x26,

    /// Load double from local variable
    /// Format: dload_<n>
    Dload1 = 0x27,

    /// Load double from local variable
    /// Format: dload_<n>
    Dload2 = 0x28,

    /// Load double from local variable
    /// Format: dload_<n>
    Dload3 = 0x29,

    /// Load reference from local variable
    /// Format: aload_<n>
    Aload0 = 0x2a,

    /// Load reference from local variable
    /// Format: aload_<n>
    Aload1 = 0x2b,

    /// Load reference from local variable
    /// Format: aload_<n>
    Aload2 = 0x2c,

    /// Load reference from local variable
    /// Format: aload_<n>
    Aload3 = 0x2d,

    /// Load int from array
    /// Format: iaload
    Iaload = 0x2e,

    /// Load long from array
    /// Format: laload
    Laload = 0x2f,

    /// Load float from array
    /// Format: faload
    Faload = 0x30,

    /// Load double from array
    /// Format: daload
    Daload = 0x31,

    /// Load reference from array
    /// Format: aaload
    Aaload = 0x32,

    /// Load byte or boolean from array
    /// Format: baload
    Baload = 0x33,

    /// Load char from array
    /// Format: caload
    Caload = 0x34,

    /// Load short from array
    /// Format: saload
    Saload = 0x35,

    /// Store int into local variable
    /// Format: istore index
    Istore = 0x36,

    /// Store long into local variable
    /// Format: lstore index
    Lstore = 0x37,

    /// Store float into local variable
    /// Format: fstore index
    Fstore = 0x38,

    /// Store double into local variable
    /// Format: dstore index
    Dstore = 0x39,

    /// Store reference into local variable
    /// Format: astore index
    Astore = 0x3a,

    /// Store int into local variable
    /// Format: istore_<n>
    Istore0 = 0x3b,

    /// Store int into local variable
    /// Format: istore_<n>
    Istore1 = 0x3c,

    /// Store int into local variable
    /// Format: istore_<n>
    Istore2 = 0x3d,

    /// Store int into local variable
    /// Format: istore_<n>
    Istore3 = 0x3e,

    /// Store long into local variable
    /// Format: lstore_<n>
    Lstore0 = 0x3f,

    /// Store long into local variable
    /// Format: lstore_<n>
    Lstore1 = 0x40,

    /// Store long into local variable
    /// Format: lstore_<n>
    Lstore2 = 0x41,

    /// Store long into local variable
    /// Format: lstore_<n>
    Lstore3 = 0x42,

    /// Store float into local variable
    /// Format: fstore_<n>
    Fstore0 = 0x43,

    /// Store float into local variable
    /// Format: fstore_<n>
    Fstore1 = 0x44,

    /// Store float into local variable
    /// Format: fstore_<n>
    Fstore2 = 0x45,

    /// Store float into local variable
    /// Format: fstore_<n>
    Fstore3 = 0x46,

    /// Store double into local variable
    /// Format: dstore_<n>
    Dstore0 = 0x47,

    /// Store double into local variable
    /// Format: dstore_<n>
    Dstore1 = 0x48,

    /// Store double into local variable
    /// Format: dstore_<n>
    Dstore2 = 0x49,

    /// Store double into local variable
    /// Format: dstore_<n>
    Dstore3 = 0x4a,

    /// Store reference into local variable
    /// Format: astore_<n>
    Astore0 = 0x4b,

    /// Store reference into local variable
    /// Format: astore_<n>
    Astore1 = 0x4c,

    /// Store reference into local variable
    /// Format: astore_<n>
    Astore2 = 0x4d,

    /// Store reference into local variable
    /// Format: astore_<n>
    Astore3 = 0x4e,

    /// Store into int array
    /// Format: iastore
    Iastore = 0x4f,

    /// Store into long array
    /// Format: lastore
    Lastore = 0x50,

    /// Store into float array
    /// Format: fastore
    Fastore = 0x51,

    /// Store into double array
    /// Format: dastore
    Dastore = 0x52,

    /// Store into reference array
    /// Format: aastore
    Aastore = 0x53,

    /// Store into byte or boolean array
    /// Format: bastore
    Bastore = 0x54,

    /// Store into char array
    /// Format: castore
    Castore = 0x55,

    /// Store into short array
    /// Format: sastore
    Sastore = 0x56,

    /// Pop the top operand stack value
    /// Format: pop
    Pop = 0x57,

    /// Pop the top one or two operand stack values
    /// Format: pop2
    Pop2 = 0x58,

    /// Duplicate the top operand stack value
    /// Format: dup
    Dup = 0x59,

    /// Duplicate the top operand stack value and insert two values down
    /// Format: dup_x1
    DupX1 = 0x5a,

    /// Duplicate the top operand stack value and insert two or three values down
    /// Format: dup_x2
    DupX2 = 0x5b,

    /// Duplicate the top one or two operand stack values
    /// Format: dup2
    Dup2 = 0x5c,

    /// Duplicate the top one or two operand stack values and insert two or three values down
    /// Format: dup2_x1
    Dup2X1 = 0x5d,

    /// Duplicate the top one or two operand stack values and insert two, three, or four values down
    /// Format: dup2_x2
    Dup2X2 = 0x5e,

    /// Swap the top two operand stack values
    /// Format: swap
    Swap = 0x5f,

    /// Add int
    /// Format: iadd
    Iadd = 0x60,

    /// Add long
    /// Format: ladd
    Ladd = 0x61,

    /// Add float
    /// Format: fadd
    Fadd = 0x62,

    /// Add double
    /// Format: dadd
    Dadd = 0x63,

    /// Subtract int
    /// Format: isub
    Isub = 0x64,

    /// Subtract long
    /// Format: lsub
    Lsub = 0x65,

    /// Subtract float
    /// Format: fsub
    Fsub = 0x66,

    /// Subtract double
    /// Format: dsub
    Dsub = 0x67,

    /// Multiply int
    /// Format: imul
    Imul = 0x68,

    /// Multiply long
    /// Format: lmul
    Lmul = 0x69,

    /// Multiply float
    /// Format: fmul
    Fmul = 0x6a,

    /// Multiply double
    /// Format: dmul
    Dmul = 0x6b,

    /// Divide int
    /// Format: idiv
    Idiv = 0x6c,

    /// Divide long
    /// Format: ldiv
    Ldiv = 0x6d,

    /// Divide float
    /// Format: fdiv
    Fdiv = 0x6e,

    /// Divide double
    /// Format: ddiv
    Ddiv = 0x6f,

    /// Remainder int
    /// Format: irem
    Irem = 0x70,

    /// Remainder long
    /// Format: lrem
    Lrem = 0x71,

    /// Remainder float
    /// Format: frem
    Frem = 0x72,

    /// Remainder double
    /// Format: drem
    Drem = 0x73,

    /// Negate int
    /// Format: ineg
    Ineg = 0x74,

    /// Negate long
    /// Format: lneg
    Lneg = 0x75,

    /// Negate float
    /// Format: fneg
    Fneg = 0x76,

    /// Negate double
    /// Format: dneg
    Dneg = 0x77,

    /// Shift left int
    /// Format: ishl
    Ishl = 0x78,

    /// Shift left long
    /// Format: lshl
    Lshl = 0x79,

    /// Arithmetic shift right int
    /// Format: ishr
    Ishr = 0x7a,

    /// Arithmetic shift right long
    /// Format: lshr
    Lshr = 0x7b,

    /// Logical shift right int
    /// Format: iushr
    Iushr = 0x7c,

    /// Logical shift right long
    /// Format: lushr
    Lushr = 0x7d,

    /// Boolean AND int
    /// Format: iand
    Iand = 0x7e,

    /// Boolean AND long
    /// Format: land
    Land = 0x7f,

    /// Boolean OR int
    /// Format: ior
    Ior = 0x80,

    /// Boolean OR long
    /// Format: lor
    Lor = 0x81,

    /// Boolean XOR int
    /// Format: ixor
    Ixor = 0x82,

    /// Boolean XOR long
    /// Format: lxor
    Lxor = 0x83,

    /// Increment local variable by constant
    /// Format: iinc index const
    Iinc = 0x84,

    /// Convert int to long
    /// Format: i2l
    I2L = 0x85,

    /// Convert int to float
    /// Format: i2f
    I2F = 0x86,

    /// Convert int to double
    /// Format: i2d
    I2D = 0x87,

    /// Convert long to int
    /// Format: l2i
    L2I = 0x88,

    /// Convert long to float
    /// Format: l2f
    L2F = 0x89,

    /// Convert long to double
    /// Format: l2d
    L2D = 0x8a,

    /// Convert float to int
    /// Format: f2i
    F2I = 0x8b,

    /// Convert float to long
    /// Format: f2l
    F2L = 0x8c,

    /// Convert float to double
    /// Format: f2d
    F2D = 0x8d,

    /// Convert double to int
    /// Format: d2i
    D2I = 0x8e,

    /// Convert double to long
    /// Format: d2l
    D2L = 0x8f,

    /// Convert double to float
    /// Format: d2f
    D2F = 0x90,

    /// Convert int to byte
    /// Format: i2b
    I2B = 0x91,

    /// Convert int to char
    /// Format: i2c
    I2C = 0x92,

    /// Convert int to short
    /// Format: i2s
    I2S = 0x93,

    /// Compare long
    /// Format: lcmp
    Lcmp = 0x94,

    /// Compare float
    /// Format: fcmp<op>
    Fcmpl = 0x95,

    /// Compare float
    /// Format: fcmp<op>
    Fcmpg = 0x96,

    /// Compare double
    /// Format: dcmp<op>
    Dcmpl = 0x97,

    /// Compare double
    /// Format: dcmp<op>
    Dcmpg = 0x98,

    /// Branch if int comparison with zero succeeds
    /// Format: if<cond> branchbyte1 branchbyte2
    Ifeq = 0x99,

    /// Branch if int comparison with zero succeeds
    /// Format: if<cond> branchbyte1 branchbyte2
    Ifne = 0x9a,

    /// Branch if int comparison with zero succeeds
    /// Format: if<cond> branchbyte1 branchbyte2
    Iflt = 0x9b,

    /// Branch if int comparison with zero succeeds
    /// Format: if<cond> branchbyte1 branchbyte2
    Ifge = 0x9c,

    /// Branch if int comparison with zero succeeds
    /// Format: if<cond> branchbyte1 branchbyte2
    Ifgt = 0x9d,

    /// Branch if int comparison with zero succeeds
    /// Format: if<cond> branchbyte1 branchbyte2
    Ifle = 0x9e,

    /// Branch if int comparison succeeds
    /// Format: if_icmp<cond> branchbyte1 branchbyte2
    IfIcmpeq = 0x9f,

    /// Branch if int comparison succeeds
    /// Format: if_icmp<cond> branchbyte1 branchbyte2
    IfIcmpne = 0xa0,

    /// Branch if int comparison succeeds
    /// Format: if_icmp<cond> branchbyte1 branchbyte2
    IfIcmplt = 0xa1,

    /// Branch if int comparison succeeds
    /// Format: if_icmp<cond> branchbyte1 branchbyte2
    IfIcmpge = 0xa2,

    /// Branch if int comparison succeeds
    /// Format: if_icmp<cond> branchbyte1 branchbyte2
    IfIcmpgt = 0xa3,

    /// Branch if int comparison succeeds
    /// Format: if_icmp<cond> branchbyte1 branchbyte2
    IfIcmple = 0xa4,

    /// Branch if reference comparison succeeds
    /// Format: if_acmp<cond> branchbyte1 branchbyte2
    IfAcmpeq = 0xa5,

    /// Branch if reference comparison succeeds
    /// Format: if_acmp<cond> branchbyte1 branchbyte2
    IfAcmpne = 0xa6,

    /// Branch always
    /// Format: goto branchbyte1 branchbyte2
    Goto = 0xa7,

    /// Jump subroutine
    /// Format: jsr branchbyte1 branchbyte2
    Jsr = 0xa8,

    /// Return from subroutine
    /// Format: ret index
    Ret = 0xa9,

    /// Access jump table by index and jump
    /// Format: tableswitch <0-3 byte pad> defaultbyte1 defaultbyte2 defaultbyte3 defaultbyte4 lowbyte1 lowbyte2 lowbyte3 lowbyte4 highbyte1 highbyte2 highbyte3 highbyte4 jump offsets...
    Tableswitch = 0xaa,

    /// Access jump table by key match and jump
    /// Format: lookupswitch <0-3 byte pad> defaultbyte1 defaultbyte2 defaultbyte3 defaultbyte4 npairs1 npairs2 npairs3 npairs4 match-offset pairs...
    Lookupswitch = 0xab,

    /// Return int from method
    /// Format: ireturn
    Ireturn = 0xac,

    /// Return long from method
    /// Format: lreturn
    Lreturn = 0xad,

    /// Return float from method
    /// Format: freturn
    Freturn = 0xae,

    /// Return double from method
    /// Format: dreturn
    Dreturn = 0xaf,

    /// Return reference from method
    /// Format: areturn
    Areturn = 0xb0,

    /// Return void from method
    /// Format: return
    Return = 0xb1,

    /// Get static field from class
    /// Format: getstatic indexbyte1 indexbyte2
    Getstatic = 0xb2,

    /// Set static field in class
    /// Format: putstatic indexbyte1 indexbyte2
    Putstatic = 0xb3,

    /// Fetch field from object
    /// Format: getfield indexbyte1 indexbyte2
    Getfield = 0xb4,

    /// Set field in object
    /// Format: putfield indexbyte1 indexbyte2
    Putfield = 0xb5,

    /// Invoke instance method; dispatch based on class
    /// Format: invokevirtual indexbyte1 indexbyte2
    Invokevirtual = 0xb6,

    /// Invoke instance method;  direct invocation of instance initialization methods and methods of the current class and its supertypes
    /// Format: invokespecial indexbyte1 indexbyte2
    Invokespecial = 0xb7,

    /// Invoke a class (static) method
    /// Format: invokestatic indexbyte1 indexbyte2
    Invokestatic = 0xb8,

    /// Invoke interface method
    /// Format: invokeinterface indexbyte1 indexbyte2 count 0
    Invokeinterface = 0xb9,

    /// Invoke a dynamically-computed call site
    /// Format: invokedynamic indexbyte1 indexbyte2 0 0
    Invokedynamic = 0xba,

    /// Create new object
    /// Format: new indexbyte1 indexbyte2
    New = 0xbb,

    /// Create new array
    /// Format: newarray atype
    Newarray = 0xbc,

    /// Create new array of reference
    /// Format: anewarray indexbyte1 indexbyte2
    Anewarray = 0xbd,

    /// Get length of array
    /// Format: arraylength
    Arraylength = 0xbe,

    /// Throw exception or error
    /// Format: athrow
    Athrow = 0xbf,

    /// Check whether object is of given type
    /// Format: checkcast indexbyte1 indexbyte2
    Checkcast = 0xc0,

    /// Determine if object is of given type
    /// Format: instanceof indexbyte1 indexbyte2
    Instanceof = 0xc1,

    /// Enter monitor for object
    /// Format: monitorenter
    Monitorenter = 0xc2,

    /// Exit monitor for object
    /// Format: monitorexit
    Monitorexit = 0xc3,

    /// Extend local variable index by additional bytes
    /// Format: wide iinc indexbyte1 indexbyte2 constbyte1 constbyte2 | wide <opcode> indexbyte1 indexbyte2
    Wide = 0xc4,

    /// Create new multidimensional array
    /// Format: multianewarray indexbyte1 indexbyte2 dimensions
    Multianewarray = 0xc5,

    /// Branch if reference is null
    /// Format: ifnull branchbyte1 branchbyte2
    Ifnull = 0xc6,

    /// Branch if reference not null
    /// Format: ifnonnull branchbyte1 branchbyte2
    Ifnonnull = 0xc7,

    /// Branch always (wide index)
    /// Format: goto_w branchbyte1 branchbyte2 branchbyte3 branchbyte4
    GotoW = 0xc8,

    /// Jump subroutine (wide index)
    /// Format: jsr_w branchbyte1 branchbyte2 branchbyte3 branchbyte4
    JsrW = 0xc9,

    Breakpoint = 0xca,

    Impdep1 = 0xfe,
    Impdep2 = 0xff,
    // invalid instructions: 203..=253
}
