use nom::{  IResult,
            be_u8,be_u16,
            be_i32,be_f32,
            ErrorKind,Err};

pub enum ConstEntry {
    Utf8(Utf8Constant),
    Integer(IntegerConstant),
    Float(FloatConstant),
//    Long(LongConstant),
//    Double(DoubleConstant),
    Class(ClassConstant),
    String(StringConstant),
    FieldRef(FieldRefConstant),
    MethodRef(MethodRefConstant),
    InterfaceMethodRef(InterfaceMethodRefConstant),
    NameAndType(NameAndTypeConstant),
//    MethodHandle(MethodHandleConstant),
//    MethodType(MethodTypeConstant),
//    InvokeDynamic(InvokeDynamicConstant),
}

impl ConstEntry {
    pub fn to_string(&self) -> String {
        match *self {
            /*
             * For all of the below matches, a reference to
             * the underlying struct must be used; this is because we have
             * borrowed self, and thus cannot take
             * ownership of anything owned by self.
             */
            ConstEntry::Utf8(ref s) => format!(
                "Utf8Constant[utf8_string=\"{}\"]", s.utf8_string),
            ConstEntry::Integer(ref s) => format!(
                "IntegerConstant[value=\"{}\"]", s.value),
            ConstEntry::Float(ref s) => format!(
                "FloatConstant[value=\"{}\"]", s.value),
            ConstEntry::Class(ref s) => format!(
                "ClassConstant[name_index={}]", s.name_index),
            ConstEntry::String(ref s) => format!(
                "StringConstant[string_index={}]", s.string_index),
            ConstEntry::FieldRef(ref s) => format!(
                "FieldRefConstant[class_index={}, name_and_type_index={}]",
                    s.class_index, s.name_and_type_index),
            ConstEntry::MethodRef(ref s) => format!(
                "MethodRefConstant[class_index={}, name_and_type_index={}]",
                    s.class_index, s.name_and_type_index),
            ConstEntry::InterfaceMethodRef(ref s) => format!(
                "InterfaceMethodRefConstant[class_index={}, name_and_type_index={}]",
                    s.class_index, s.name_and_type_index),
            ConstEntry::NameAndType(ref s) => format!(
                "NameAndTypeConstant[name_index={}, descriptor_index={}]",
                    s.name_index, s.descriptor_index),
        }
    }
}

pub struct Utf8Constant {
    pub utf8_string: String,
}

pub struct IntegerConstant {
    pub value: i32,
}
pub struct FloatConstant {
    pub value: f32,
}

pub struct ClassConstant {
    pub name_index: u16,
}

pub struct StringConstant {
    pub string_index: u16,
}

pub struct FieldRefConstant {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

pub struct MethodRefConstant {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

pub struct InterfaceMethodRefConstant {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

pub struct NameAndTypeConstant {
    pub name_index: u16,
    pub descriptor_index: u16,
}

pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub const_pool_size: u16,
    pub const_pool: Vec<ConstEntry>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
//     pub interfaces[interfaces_count]: u16,
//     pub fields_count: u16,
//     pub fields[fields_count]: u16,
//     pub methods_count: u16,
//     pub methods[methods_count]: u16,
//     pub attributes_count: u16,
//     pub attributes[attributes_count]: u16,
}

named!(magic_ident, tag!(&[0xCA, 0xFE, 0xBA, 0xBE]));

named!(const_utf8<&[u8], ConstEntry>, chain!(
    // tag!([0x01]) ~
    length: be_u16 ~
    utf8_str: take_str!(length),
    || {
        ConstEntry::Utf8(
            Utf8Constant {
                utf8_string: utf8_str.to_owned(),
            }
        )
    }
));

named!(const_integer<&[u8], ConstEntry>, chain!(
    // tag!([0x03]) ~
    value: be_i32,
    || {
        ConstEntry::Integer(
            IntegerConstant {
                value: value,
            }
        )
    }
));

named!(const_float<&[u8], ConstEntry>, chain!(
    // tag!([0x04]) ~
    value: be_f32,
    || {
        ConstEntry::Float(
            FloatConstant {
                value: value,
            }
        )
    }
));

named!(const_class<&[u8], ConstEntry>, chain!(
    // tag: tag!([0x07]) ~
    name_index: be_u16,
    || {
        ConstEntry::Class(
            ClassConstant {
                name_index: name_index,
            }
        )
    }
));

named!(const_string<&[u8], ConstEntry>, chain!(
    // tag: tag!([0x08]) ~
    string_index: be_u16,
    || {
        ConstEntry::String(
            StringConstant {
                string_index: string_index,
            }
        )
    }
));

named!(const_field_ref<&[u8], ConstEntry>, chain!(
    // tag: tag!([0x09]) ~
    class_index: be_u16 ~
    name_and_type_index: be_u16,
    || {
        ConstEntry::FieldRef(
            FieldRefConstant {
                class_index: class_index,
                name_and_type_index: name_and_type_index,
            }
        )
    }
));

named!(const_method_ref<&[u8], ConstEntry>, chain!(
    // tag: tag!([0x0A]) ~
    class_index: be_u16 ~
    name_and_type_index: be_u16,
    || {
        ConstEntry::MethodRef(
            MethodRefConstant {
                class_index: class_index,
                name_and_type_index: name_and_type_index,
            }
        )
    }
));

named!(const_interface_method_ref<&[u8], ConstEntry>, chain!(
    // tag: tag!([0x0B]) ~
    class_index: be_u16 ~
    name_and_type_index: be_u16,
    || {
        ConstEntry::InterfaceMethodRef(
            InterfaceMethodRefConstant {
                class_index: class_index,
                name_and_type_index: name_and_type_index,
            }
        )
    }
));

named!(const_name_and_type<&[u8], ConstEntry>, chain!(
    // tag: tag!([0x0C]) ~
    name_index: be_u16 ~
    descriptor_index: be_u16,
    || {
        ConstEntry::NameAndType(
            NameAndTypeConstant {
                name_index: name_index,
                descriptor_index: descriptor_index,
            }
        )
    }
));

pub fn parse_const(input: &[u8]) -> IResult<&[u8], ConstEntry> {
    chain!(input,
        const_type: be_u8 ~
        const_block: apply!(const_block, const_type),
        || {
            const_block
        }
    )
}
pub fn const_block(input: &[u8], const_type: u8) -> IResult<&[u8], ConstEntry> {
    match const_type {
        1 => const_utf8(input),
        3 => const_integer(input),
        4 => const_float(input),
        // // 5 => //CONSTANT_Long,
        // // 6 => //CONSTANT_Double,
        7 => const_class(input),
        8 => const_string(input),
        9 => const_field_ref(input),
        10 => const_method_ref(input),
        11 => const_interface_method_ref(input),
        12 => const_name_and_type(input),
        // // 15 => //CONSTANT_MethodHandle,
        // // 16 => //CONSTANT_MethodType,
        // // 18 => //CONSTANT_InvokeDynamic,
        _ => IResult::Error(Err::Position(ErrorKind::Alt, input)),
    }
}

// TODO - This is a bitmask op, flag u16 matches multiple of these, how to stoe in class spec?
pub enum AccessFlags {
    Public,     // 	0x0001 	Declared public; may be accessed from outside its package.
    Final,      // 	0x0010 	Declared final; no subclasses allowed.
    Super,      // 	0x0020 	Treat superclass methods specially when invoked by the invokespecial instruction.
    Interface,  // 	0x0200 	Is an interface, not a class.
    Abstract,   // 	0x0400 	Declared abstract; must not be instantiated.
    Synthetic,  // 	0x1000 	Declared synthetic; not present in the source code.
    Annotation, // 	0x2000 	Declared as an annotation type.
    Enum,       // 	0x4000 	Declared as an enum type
}

pub fn parse_classfile(input: &[u8]) -> IResult<&[u8], ClassFile> {
  chain!(input,
    magic_ident ~
    minor_version: be_u16 ~
    major_version: be_u16 ~
    const_pool_size: be_u16 ~
    const_pool: count!(parse_const, (const_pool_size - 1) as usize) ~
    access_flags: be_u16 ~
    this_class: be_u16 ~
    super_class: be_u16 ~
    interfaces_count: be_u16,
    || {
        ClassFile {
            minor_version: minor_version,
            major_version: major_version,
            const_pool_size: const_pool_size,
            const_pool: const_pool,
            access_flags: access_flags,
            this_class: this_class,
            super_class: super_class,
            interfaces_count: interfaces_count,
        }
    }
  )
}

#[test]
fn test_valid_class() {
    let valid_class = include_bytes!("../assets/SimpleMath.class");
    let res = parse_classfile(valid_class);
    match res {
        IResult::Done(_, c) => {
            println!("Valid class file, version {},{} const_pool({}), this=const[{}], super=const[{}], interfaces({})", c.major_version, c.minor_version, c.const_pool_size, c.this_class, c.super_class, c.interfaces_count);
            println!("Constant pool:");
            let mut const_index = 1;
            for f in &c.const_pool {
                println!("\t[{}] = {}", const_index, f.to_string());
                const_index += 1;
            }
        },
        _ => panic!("Not a class file"),
    };
}

#[test]
fn test_malformed_class() {
    let malformed_class = include_bytes!("../assets/malformed.class");
    let res = parse_classfile(malformed_class);
    match res {
        IResult::Done(_, _) => panic!("The file is not valid and shouldn't be parsed"),
        _ => res,
    };
}

#[test]
fn test_constant_utf8() {
    let hello_world_data = &[
        // 0x01, // tag = 1
        0x00, 0x0C, // length = 12
        0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21 // 'Hello world!' in UTF8
    ];
    let res = const_utf8(hello_world_data);

    match res {
        IResult::Done(_, c) =>
        match c {
            ConstEntry::Utf8(ref s) =>
                 println!("Valid UTF8 const: {}", s.utf8_string),
            _ => panic!("It's a const, but of what type?")
        },
        _ => panic!("Not a UTF type const?"),
    };
}