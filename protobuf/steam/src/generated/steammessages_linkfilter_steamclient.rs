// This file is generated by rust-protobuf 3.5.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `steammessages_linkfilter.steamclient.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::steam_vent_proto_common::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:CCommunity_GetLinkFilterHashPrefixes_Request)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CCommunity_GetLinkFilterHashPrefixes_Request {
    // message fields
    // @@protoc_insertion_point(field:CCommunity_GetLinkFilterHashPrefixes_Request.hit_type)
    pub hit_type: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CCommunity_GetLinkFilterHashPrefixes_Request.count)
    pub count: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CCommunity_GetLinkFilterHashPrefixes_Request.start)
    pub start: ::std::option::Option<u64>,
    // special fields
    // @@protoc_insertion_point(special_field:CCommunity_GetLinkFilterHashPrefixes_Request.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CCommunity_GetLinkFilterHashPrefixes_Request {
    fn default() -> &'a CCommunity_GetLinkFilterHashPrefixes_Request {
        <CCommunity_GetLinkFilterHashPrefixes_Request as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CCommunity_GetLinkFilterHashPrefixes_Request {
    pub fn new() -> CCommunity_GetLinkFilterHashPrefixes_Request {
        ::std::default::Default::default()
    }

    // optional uint32 hit_type = 1;

    pub fn hit_type(&self) -> u32 {
        self.hit_type.unwrap_or(0)
    }

    pub fn clear_hit_type(&mut self) {
        self.hit_type = ::std::option::Option::None;
    }

    pub fn has_hit_type(&self) -> bool {
        self.hit_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hit_type(&mut self, v: u32) {
        self.hit_type = ::std::option::Option::Some(v);
    }

    // optional uint32 count = 2;

    pub fn count(&self) -> u32 {
        self.count.unwrap_or(0)
    }

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = ::std::option::Option::Some(v);
    }

    // optional uint64 start = 3;

    pub fn start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CCommunity_GetLinkFilterHashPrefixes_Request {
    const NAME: &'static str = "CCommunity_GetLinkFilterHashPrefixes_Request";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.hit_type = ::std::option::Option::Some(is.read_uint32()?);
                },
                16 => {
                    self.count = ::std::option::Option::Some(is.read_uint32()?);
                },
                24 => {
                    self.start = ::std::option::Option::Some(is.read_uint64()?);
                },
                tag => {
                    ::steam_vent_proto_common::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.hit_type {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(1, v);
        }
        if let Some(v) = self.count {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(2, v);
        }
        if let Some(v) = self.start {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint64_size(3, v);
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.hit_type {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.count {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.start {
            os.write_uint64(3, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CCommunity_GetLinkFilterHashPrefixes_Request {
        CCommunity_GetLinkFilterHashPrefixes_Request::new()
    }

    fn clear(&mut self) {
        self.hit_type = ::std::option::Option::None;
        self.count = ::std::option::Option::None;
        self.start = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CCommunity_GetLinkFilterHashPrefixes_Request {
        static instance: CCommunity_GetLinkFilterHashPrefixes_Request = CCommunity_GetLinkFilterHashPrefixes_Request {
            hit_type: ::std::option::Option::None,
            count: ::std::option::Option::None,
            start: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CCommunity_GetLinkFilterHashPrefixes_Response)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CCommunity_GetLinkFilterHashPrefixes_Response {
    // message fields
    // @@protoc_insertion_point(field:CCommunity_GetLinkFilterHashPrefixes_Response.hash_prefixes)
    pub hash_prefixes: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:CCommunity_GetLinkFilterHashPrefixes_Response.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CCommunity_GetLinkFilterHashPrefixes_Response {
    fn default() -> &'a CCommunity_GetLinkFilterHashPrefixes_Response {
        <CCommunity_GetLinkFilterHashPrefixes_Response as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CCommunity_GetLinkFilterHashPrefixes_Response {
    pub fn new() -> CCommunity_GetLinkFilterHashPrefixes_Response {
        ::std::default::Default::default()
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CCommunity_GetLinkFilterHashPrefixes_Response {
    const NAME: &'static str = "CCommunity_GetLinkFilterHashPrefixes_Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.hash_prefixes)?;
                },
                8 => {
                    self.hash_prefixes.push(is.read_uint32()?);
                },
                tag => {
                    ::steam_vent_proto_common::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.hash_prefixes {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(1, *value);
        };
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        for v in &self.hash_prefixes {
            os.write_uint32(1, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CCommunity_GetLinkFilterHashPrefixes_Response {
        CCommunity_GetLinkFilterHashPrefixes_Response::new()
    }

    fn clear(&mut self) {
        self.hash_prefixes.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CCommunity_GetLinkFilterHashPrefixes_Response {
        static instance: CCommunity_GetLinkFilterHashPrefixes_Response = CCommunity_GetLinkFilterHashPrefixes_Response {
            hash_prefixes: ::std::vec::Vec::new(),
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CCommunity_GetLinkFilterHashes_Request)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CCommunity_GetLinkFilterHashes_Request {
    // message fields
    // @@protoc_insertion_point(field:CCommunity_GetLinkFilterHashes_Request.hit_type)
    pub hit_type: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CCommunity_GetLinkFilterHashes_Request.count)
    pub count: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CCommunity_GetLinkFilterHashes_Request.start)
    pub start: ::std::option::Option<u64>,
    // special fields
    // @@protoc_insertion_point(special_field:CCommunity_GetLinkFilterHashes_Request.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CCommunity_GetLinkFilterHashes_Request {
    fn default() -> &'a CCommunity_GetLinkFilterHashes_Request {
        <CCommunity_GetLinkFilterHashes_Request as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CCommunity_GetLinkFilterHashes_Request {
    pub fn new() -> CCommunity_GetLinkFilterHashes_Request {
        ::std::default::Default::default()
    }

    // optional uint32 hit_type = 1;

    pub fn hit_type(&self) -> u32 {
        self.hit_type.unwrap_or(0)
    }

    pub fn clear_hit_type(&mut self) {
        self.hit_type = ::std::option::Option::None;
    }

    pub fn has_hit_type(&self) -> bool {
        self.hit_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hit_type(&mut self, v: u32) {
        self.hit_type = ::std::option::Option::Some(v);
    }

    // optional uint32 count = 2;

    pub fn count(&self) -> u32 {
        self.count.unwrap_or(0)
    }

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = ::std::option::Option::Some(v);
    }

    // optional uint64 start = 3;

    pub fn start(&self) -> u64 {
        self.start.unwrap_or(0)
    }

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CCommunity_GetLinkFilterHashes_Request {
    const NAME: &'static str = "CCommunity_GetLinkFilterHashes_Request";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.hit_type = ::std::option::Option::Some(is.read_uint32()?);
                },
                16 => {
                    self.count = ::std::option::Option::Some(is.read_uint32()?);
                },
                24 => {
                    self.start = ::std::option::Option::Some(is.read_uint64()?);
                },
                tag => {
                    ::steam_vent_proto_common::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.hit_type {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(1, v);
        }
        if let Some(v) = self.count {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(2, v);
        }
        if let Some(v) = self.start {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint64_size(3, v);
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.hit_type {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.count {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.start {
            os.write_uint64(3, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CCommunity_GetLinkFilterHashes_Request {
        CCommunity_GetLinkFilterHashes_Request::new()
    }

    fn clear(&mut self) {
        self.hit_type = ::std::option::Option::None;
        self.count = ::std::option::Option::None;
        self.start = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CCommunity_GetLinkFilterHashes_Request {
        static instance: CCommunity_GetLinkFilterHashes_Request = CCommunity_GetLinkFilterHashes_Request {
            hit_type: ::std::option::Option::None,
            count: ::std::option::Option::None,
            start: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CCommunity_GetLinkFilterHashes_Response)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CCommunity_GetLinkFilterHashes_Response {
    // message fields
    // @@protoc_insertion_point(field:CCommunity_GetLinkFilterHashes_Response.hashes)
    pub hashes: ::std::vec::Vec<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:CCommunity_GetLinkFilterHashes_Response.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CCommunity_GetLinkFilterHashes_Response {
    fn default() -> &'a CCommunity_GetLinkFilterHashes_Response {
        <CCommunity_GetLinkFilterHashes_Response as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CCommunity_GetLinkFilterHashes_Response {
    pub fn new() -> CCommunity_GetLinkFilterHashes_Response {
        ::std::default::Default::default()
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CCommunity_GetLinkFilterHashes_Response {
    const NAME: &'static str = "CCommunity_GetLinkFilterHashes_Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.hashes.push(is.read_bytes()?);
                },
                tag => {
                    ::steam_vent_proto_common::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.hashes {
            my_size += ::steam_vent_proto_common::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        for v in &self.hashes {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CCommunity_GetLinkFilterHashes_Response {
        CCommunity_GetLinkFilterHashes_Response::new()
    }

    fn clear(&mut self) {
        self.hashes.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CCommunity_GetLinkFilterHashes_Response {
        static instance: CCommunity_GetLinkFilterHashes_Response = CCommunity_GetLinkFilterHashes_Response {
            hashes: ::std::vec::Vec::new(),
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CCommunity_GetLinkFilterListVersion_Request)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CCommunity_GetLinkFilterListVersion_Request {
    // message fields
    // @@protoc_insertion_point(field:CCommunity_GetLinkFilterListVersion_Request.hit_type)
    pub hit_type: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:CCommunity_GetLinkFilterListVersion_Request.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CCommunity_GetLinkFilterListVersion_Request {
    fn default() -> &'a CCommunity_GetLinkFilterListVersion_Request {
        <CCommunity_GetLinkFilterListVersion_Request as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CCommunity_GetLinkFilterListVersion_Request {
    pub fn new() -> CCommunity_GetLinkFilterListVersion_Request {
        ::std::default::Default::default()
    }

    // optional uint32 hit_type = 1;

    pub fn hit_type(&self) -> u32 {
        self.hit_type.unwrap_or(0)
    }

    pub fn clear_hit_type(&mut self) {
        self.hit_type = ::std::option::Option::None;
    }

    pub fn has_hit_type(&self) -> bool {
        self.hit_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hit_type(&mut self, v: u32) {
        self.hit_type = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CCommunity_GetLinkFilterListVersion_Request {
    const NAME: &'static str = "CCommunity_GetLinkFilterListVersion_Request";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.hit_type = ::std::option::Option::Some(is.read_uint32()?);
                },
                tag => {
                    ::steam_vent_proto_common::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.hit_type {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(1, v);
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.hit_type {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CCommunity_GetLinkFilterListVersion_Request {
        CCommunity_GetLinkFilterListVersion_Request::new()
    }

    fn clear(&mut self) {
        self.hit_type = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CCommunity_GetLinkFilterListVersion_Request {
        static instance: CCommunity_GetLinkFilterListVersion_Request = CCommunity_GetLinkFilterListVersion_Request {
            hit_type: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CCommunity_GetLinkFilterListVersion_Response)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CCommunity_GetLinkFilterListVersion_Response {
    // message fields
    // @@protoc_insertion_point(field:CCommunity_GetLinkFilterListVersion_Response.version)
    pub version: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:CCommunity_GetLinkFilterListVersion_Response.count)
    pub count: ::std::option::Option<u64>,
    // special fields
    // @@protoc_insertion_point(special_field:CCommunity_GetLinkFilterListVersion_Response.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CCommunity_GetLinkFilterListVersion_Response {
    fn default() -> &'a CCommunity_GetLinkFilterListVersion_Response {
        <CCommunity_GetLinkFilterListVersion_Response as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CCommunity_GetLinkFilterListVersion_Response {
    pub fn new() -> CCommunity_GetLinkFilterListVersion_Response {
        ::std::default::Default::default()
    }

    // optional string version = 1;

    pub fn version(&self) -> &str {
        match self.version.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        if self.version.is_none() {
            self.version = ::std::option::Option::Some(::std::string::String::new());
        }
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional uint64 count = 2;

    pub fn count(&self) -> u64 {
        self.count.unwrap_or(0)
    }

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u64) {
        self.count = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CCommunity_GetLinkFilterListVersion_Response {
    const NAME: &'static str = "CCommunity_GetLinkFilterListVersion_Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.version = ::std::option::Option::Some(is.read_string()?);
                },
                16 => {
                    self.count = ::std::option::Option::Some(is.read_uint64()?);
                },
                tag => {
                    ::steam_vent_proto_common::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.version.as_ref() {
            my_size += ::steam_vent_proto_common::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.count {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint64_size(2, v);
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.version.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.count {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CCommunity_GetLinkFilterListVersion_Response {
        CCommunity_GetLinkFilterListVersion_Response::new()
    }

    fn clear(&mut self) {
        self.version = ::std::option::Option::None;
        self.count = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CCommunity_GetLinkFilterListVersion_Response {
        static instance: CCommunity_GetLinkFilterListVersion_Response = CCommunity_GetLinkFilterListVersion_Response {
            version: ::std::option::Option::None,
            count: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}


const _VENT_PROTO_VERSION_CHECK: () = ::steam_vent_proto_common::VERSION_0_5_0;

#[allow(unused_imports)]
use crate::steammessages_base::*;
#[allow(unused_imports)]
use crate::steammessages_unified_base_steamclient::*;
impl ::steam_vent_proto_common::RpcMessage
for CCommunity_GetLinkFilterHashPrefixes_Request {
    fn parse(reader: &mut dyn std::io::Read) -> ::steam_vent_proto_common::protobuf::Result<Self> {
        <Self as ::steam_vent_proto_common::protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> ::steam_vent_proto_common::protobuf::Result<()> {
        use ::steam_vent_proto_common::protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use ::steam_vent_proto_common::protobuf::Message;
        self.compute_size() as usize
    }
}
impl ::steam_vent_proto_common::RpcMessage
for CCommunity_GetLinkFilterHashPrefixes_Response {
    fn parse(reader: &mut dyn std::io::Read) -> ::steam_vent_proto_common::protobuf::Result<Self> {
        <Self as ::steam_vent_proto_common::protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> ::steam_vent_proto_common::protobuf::Result<()> {
        use ::steam_vent_proto_common::protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use ::steam_vent_proto_common::protobuf::Message;
        self.compute_size() as usize
    }
}
impl ::steam_vent_proto_common::RpcMessage for CCommunity_GetLinkFilterHashes_Request {
    fn parse(reader: &mut dyn std::io::Read) -> ::steam_vent_proto_common::protobuf::Result<Self> {
        <Self as ::steam_vent_proto_common::protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> ::steam_vent_proto_common::protobuf::Result<()> {
        use ::steam_vent_proto_common::protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use ::steam_vent_proto_common::protobuf::Message;
        self.compute_size() as usize
    }
}
impl ::steam_vent_proto_common::RpcMessage for CCommunity_GetLinkFilterHashes_Response {
    fn parse(reader: &mut dyn std::io::Read) -> ::steam_vent_proto_common::protobuf::Result<Self> {
        <Self as ::steam_vent_proto_common::protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> ::steam_vent_proto_common::protobuf::Result<()> {
        use ::steam_vent_proto_common::protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use ::steam_vent_proto_common::protobuf::Message;
        self.compute_size() as usize
    }
}
impl ::steam_vent_proto_common::RpcMessage
for CCommunity_GetLinkFilterListVersion_Request {
    fn parse(reader: &mut dyn std::io::Read) -> ::steam_vent_proto_common::protobuf::Result<Self> {
        <Self as ::steam_vent_proto_common::protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> ::steam_vent_proto_common::protobuf::Result<()> {
        use ::steam_vent_proto_common::protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use ::steam_vent_proto_common::protobuf::Message;
        self.compute_size() as usize
    }
}
impl ::steam_vent_proto_common::RpcMessage
for CCommunity_GetLinkFilterListVersion_Response {
    fn parse(reader: &mut dyn std::io::Read) -> ::steam_vent_proto_common::protobuf::Result<Self> {
        <Self as ::steam_vent_proto_common::protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> ::steam_vent_proto_common::protobuf::Result<()> {
        use ::steam_vent_proto_common::protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use ::steam_vent_proto_common::protobuf::Message;
        self.compute_size() as usize
    }
}
///
struct CommunityLinkFilter {}
impl ::steam_vent_proto_common::RpcService for CommunityLinkFilter {
    const SERVICE_NAME: &'static str = "CommunityLinkFilter";
}
impl ::steam_vent_proto_common::RpcMethod
for CCommunity_GetLinkFilterHashPrefixes_Request {
    const METHOD_NAME: &'static str = "CommunityLinkFilter.GetLinkFilterHashPrefixes#1";
    type Response = CCommunity_GetLinkFilterHashPrefixes_Response;
}
impl ::steam_vent_proto_common::RpcMethod for CCommunity_GetLinkFilterHashes_Request {
    const METHOD_NAME: &'static str = "CommunityLinkFilter.GetLinkFilterHashes#1";
    type Response = CCommunity_GetLinkFilterHashes_Response;
}
impl ::steam_vent_proto_common::RpcMethod
for CCommunity_GetLinkFilterListVersion_Request {
    const METHOD_NAME: &'static str = "CommunityLinkFilter.GetLinkFilterListVersion#1";
    type Response = CCommunity_GetLinkFilterListVersion_Response;
}
