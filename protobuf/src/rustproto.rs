// This file is generated by rust-protobuf 1.4.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const carllerche_bytes_for_bytes_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:b\n\x1ecarllerche_bytes_for_bytes_all\x18\xf3\x84\x01\x20\x01(\x08\
    \x12\x1c.google.protobuf.FileOptionsR\x1acarllercheBytesForBytesAll:d\n\
    \x1fcarllerche_bytes_for_string_all\x18\xf4\x84\x01\x20\x01(\x08\x12\x1c\
    .google.protobuf.FileOptionsR\x1bcarllercheBytesForStringAll:^\n\x1acarl\
    lerche_bytes_for_bytes\x18\xf3\x84\x01\x20\x01(\x08\x12\x1f.google.proto\
    buf.MessageOptionsR\x17carllercheBytesForBytes:`\n\x1bcarllerche_bytes_f\
    or_string\x18\xf4\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOp\
    tionsR\x18carllercheBytesForString:g\n\x20carllerche_bytes_for_bytes_fie\
    ld\x18\xf3\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\
    \x1ccarllercheBytesForBytesField:i\n!carllerche_bytes_for_string_field\
    \x18\xf4\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x1dc\
    arllercheBytesForStringFieldJ\xcb\x06\n\x06\x12\x04\0\0\x1c\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07)\nh\n\x01\x02\
    \x12\x03\x07\x08\x112^\x20see\x20https://github.com/gogo/protobuf/blob/m\
    aster/gogoproto/gogo.proto\n\x20for\x20the\x20original\x20idea\n\n\t\n\
    \x01\x07\x12\x04\t\0\x0e\x01\n2\n\x02\x07\0\x12\x03\x0b\x049\x1a'\x20Use\
    \x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\0\x02\x12\
    \x03\t\x07\"\n\n\n\x03\x07\0\x04\x12\x03\x0b\x04\x0c\n\n\n\x03\x07\0\x05\
    \x12\x03\x0b\r\x11\n\n\n\x03\x07\0\x01\x12\x03\x0b\x120\n\n\n\x03\x07\0\
    \x03\x12\x03\x0b38\n3\n\x02\x07\x01\x12\x03\r\x04:\x1a(\x20Use\x20`bytes\
    ::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x01\x02\x12\x03\t\
    \x07\"\n\n\n\x03\x07\x01\x04\x12\x03\r\x04\x0c\n\n\n\x03\x07\x01\x05\x12\
    \x03\r\r\x11\n\n\n\x03\x07\x01\x01\x12\x03\r\x121\n\n\n\x03\x07\x01\x03\
    \x12\x03\r49\n\t\n\x01\x07\x12\x04\x10\0\x15\x01\n2\n\x02\x07\x02\x12\
    \x03\x12\x045\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\
    \n\n\n\n\x03\x07\x02\x02\x12\x03\x10\x07%\n\n\n\x03\x07\x02\x04\x12\x03\
    \x12\x04\x0c\n\n\n\x03\x07\x02\x05\x12\x03\x12\r\x11\n\n\n\x03\x07\x02\
    \x01\x12\x03\x12\x12,\n\n\n\x03\x07\x02\x03\x12\x03\x12/4\n3\n\x02\x07\
    \x03\x12\x03\x14\x046\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`string`\
    \x20fields\n\n\n\n\x03\x07\x03\x02\x12\x03\x10\x07%\n\n\n\x03\x07\x03\
    \x04\x12\x03\x14\x04\x0c\n\n\n\x03\x07\x03\x05\x12\x03\x14\r\x11\n\n\n\
    \x03\x07\x03\x01\x12\x03\x14\x12-\n\n\n\x03\x07\x03\x03\x12\x03\x1405\n\
    \t\n\x01\x07\x12\x04\x17\0\x1c\x01\n2\n\x02\x07\x04\x12\x03\x19\x04;\x1a\
    '\x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\
    \x04\x02\x12\x03\x17\x07#\n\n\n\x03\x07\x04\x04\x12\x03\x19\x04\x0c\n\n\
    \n\x03\x07\x04\x05\x12\x03\x19\r\x11\n\n\n\x03\x07\x04\x01\x12\x03\x19\
    \x122\n\n\n\x03\x07\x04\x03\x12\x03\x195:\n3\n\x02\x07\x05\x12\x03\x1b\
    \x04<\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\
    \x03\x07\x05\x02\x12\x03\x17\x07#\n\n\n\x03\x07\x05\x04\x12\x03\x1b\x04\
    \x0c\n\n\n\x03\x07\x05\x05\x12\x03\x1b\r\x11\n\n\n\x03\x07\x05\x01\x12\
    \x03\x1b\x123\n\n\n\x03\x07\x05\x03\x12\x03\x1b6;\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
