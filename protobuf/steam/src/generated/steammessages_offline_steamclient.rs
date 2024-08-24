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

//! Generated file from `steammessages_offline.steamclient.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::steam_vent_proto_common::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:COffline_GetOfflineLogonTicket_Request)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct COffline_GetOfflineLogonTicket_Request {
    // message fields
    // @@protoc_insertion_point(field:COffline_GetOfflineLogonTicket_Request.priority)
    pub priority: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:COffline_GetOfflineLogonTicket_Request.perform_encryption)
    pub perform_encryption: ::std::option::Option<bool>,
    // special fields
    // @@protoc_insertion_point(special_field:COffline_GetOfflineLogonTicket_Request.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a COffline_GetOfflineLogonTicket_Request {
    fn default() -> &'a COffline_GetOfflineLogonTicket_Request {
        <COffline_GetOfflineLogonTicket_Request as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl COffline_GetOfflineLogonTicket_Request {
    pub fn new() -> COffline_GetOfflineLogonTicket_Request {
        ::std::default::Default::default()
    }

    // optional uint32 priority = 1;

    pub fn priority(&self) -> u32 {
        self.priority.unwrap_or(0)
    }

    pub fn clear_priority(&mut self) {
        self.priority = ::std::option::Option::None;
    }

    pub fn has_priority(&self) -> bool {
        self.priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_priority(&mut self, v: u32) {
        self.priority = ::std::option::Option::Some(v);
    }

    // optional bool perform_encryption = 2;

    pub fn perform_encryption(&self) -> bool {
        self.perform_encryption.unwrap_or(false)
    }

    pub fn clear_perform_encryption(&mut self) {
        self.perform_encryption = ::std::option::Option::None;
    }

    pub fn has_perform_encryption(&self) -> bool {
        self.perform_encryption.is_some()
    }

    // Param is passed by value, moved
    pub fn set_perform_encryption(&mut self, v: bool) {
        self.perform_encryption = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for COffline_GetOfflineLogonTicket_Request {
    const NAME: &'static str = "COffline_GetOfflineLogonTicket_Request";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.priority = ::std::option::Option::Some(is.read_uint32()?);
                },
                16 => {
                    self.perform_encryption = ::std::option::Option::Some(is.read_bool()?);
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
        if let Some(v) = self.priority {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(1, v);
        }
        if let Some(v) = self.perform_encryption {
            my_size += 1 + 1;
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.priority {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.perform_encryption {
            os.write_bool(2, v)?;
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

    fn new() -> COffline_GetOfflineLogonTicket_Request {
        COffline_GetOfflineLogonTicket_Request::new()
    }

    fn clear(&mut self) {
        self.priority = ::std::option::Option::None;
        self.perform_encryption = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static COffline_GetOfflineLogonTicket_Request {
        static instance: COffline_GetOfflineLogonTicket_Request = COffline_GetOfflineLogonTicket_Request {
            priority: ::std::option::Option::None,
            perform_encryption: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:COffline_GetOfflineLogonTicket_Response)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct COffline_GetOfflineLogonTicket_Response {
    // message fields
    // @@protoc_insertion_point(field:COffline_GetOfflineLogonTicket_Response.serialized_ticket)
    pub serialized_ticket: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:COffline_GetOfflineLogonTicket_Response.signature)
    pub signature: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:COffline_GetOfflineLogonTicket_Response.encrypted_ticket)
    pub encrypted_ticket: ::steam_vent_proto_common::protobuf::MessageField<super::offline_ticket::Offline_Ticket>,
    // special fields
    // @@protoc_insertion_point(special_field:COffline_GetOfflineLogonTicket_Response.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a COffline_GetOfflineLogonTicket_Response {
    fn default() -> &'a COffline_GetOfflineLogonTicket_Response {
        <COffline_GetOfflineLogonTicket_Response as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl COffline_GetOfflineLogonTicket_Response {
    pub fn new() -> COffline_GetOfflineLogonTicket_Response {
        ::std::default::Default::default()
    }

    // optional bytes serialized_ticket = 1;

    pub fn serialized_ticket(&self) -> &[u8] {
        match self.serialized_ticket.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_serialized_ticket(&mut self) {
        self.serialized_ticket = ::std::option::Option::None;
    }

    pub fn has_serialized_ticket(&self) -> bool {
        self.serialized_ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serialized_ticket(&mut self, v: ::std::vec::Vec<u8>) {
        self.serialized_ticket = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serialized_ticket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.serialized_ticket.is_none() {
            self.serialized_ticket = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.serialized_ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_serialized_ticket(&mut self) -> ::std::vec::Vec<u8> {
        self.serialized_ticket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes signature = 2;

    pub fn signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_signature(&mut self) {
        self.signature = ::std::option::Option::None;
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::steam_vent_proto_common::protobuf::Message for COffline_GetOfflineLogonTicket_Response {
    const NAME: &'static str = "COffline_GetOfflineLogonTicket_Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.serialized_ticket = ::std::option::Option::Some(is.read_bytes()?);
                },
                18 => {
                    self.signature = ::std::option::Option::Some(is.read_bytes()?);
                },
                26 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.encrypted_ticket)?;
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
        if let Some(v) = self.serialized_ticket.as_ref() {
            my_size += ::steam_vent_proto_common::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.signature.as_ref() {
            my_size += ::steam_vent_proto_common::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.encrypted_ticket.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.serialized_ticket.as_ref() {
            os.write_bytes(1, v)?;
        }
        if let Some(v) = self.signature.as_ref() {
            os.write_bytes(2, v)?;
        }
        if let Some(v) = self.encrypted_ticket.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> COffline_GetOfflineLogonTicket_Response {
        COffline_GetOfflineLogonTicket_Response::new()
    }

    fn clear(&mut self) {
        self.serialized_ticket = ::std::option::Option::None;
        self.signature = ::std::option::Option::None;
        self.encrypted_ticket.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static COffline_GetOfflineLogonTicket_Response {
        static instance: COffline_GetOfflineLogonTicket_Response = COffline_GetOfflineLogonTicket_Response {
            serialized_ticket: ::std::option::Option::None,
            signature: ::std::option::Option::None,
            encrypted_ticket: ::steam_vent_proto_common::protobuf::MessageField::none(),
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:COffline_GetUnsignedOfflineLogonTicket_Request)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct COffline_GetUnsignedOfflineLogonTicket_Request {
    // special fields
    // @@protoc_insertion_point(special_field:COffline_GetUnsignedOfflineLogonTicket_Request.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a COffline_GetUnsignedOfflineLogonTicket_Request {
    fn default() -> &'a COffline_GetUnsignedOfflineLogonTicket_Request {
        <COffline_GetUnsignedOfflineLogonTicket_Request as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl COffline_GetUnsignedOfflineLogonTicket_Request {
    pub fn new() -> COffline_GetUnsignedOfflineLogonTicket_Request {
        ::std::default::Default::default()
    }
}

impl ::steam_vent_proto_common::protobuf::Message for COffline_GetUnsignedOfflineLogonTicket_Request {
    const NAME: &'static str = "COffline_GetUnsignedOfflineLogonTicket_Request";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
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
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> COffline_GetUnsignedOfflineLogonTicket_Request {
        COffline_GetUnsignedOfflineLogonTicket_Request::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static COffline_GetUnsignedOfflineLogonTicket_Request {
        static instance: COffline_GetUnsignedOfflineLogonTicket_Request = COffline_GetUnsignedOfflineLogonTicket_Request {
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:COffline_OfflineLogonTicket)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct COffline_OfflineLogonTicket {
    // message fields
    // @@protoc_insertion_point(field:COffline_OfflineLogonTicket.accountid)
    pub accountid: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:COffline_OfflineLogonTicket.rtime32_creation_time)
    pub rtime32_creation_time: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:COffline_OfflineLogonTicket.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a COffline_OfflineLogonTicket {
    fn default() -> &'a COffline_OfflineLogonTicket {
        <COffline_OfflineLogonTicket as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl COffline_OfflineLogonTicket {
    pub fn new() -> COffline_OfflineLogonTicket {
        ::std::default::Default::default()
    }

    // optional uint32 accountid = 1;

    pub fn accountid(&self) -> u32 {
        self.accountid.unwrap_or(0)
    }

    pub fn clear_accountid(&mut self) {
        self.accountid = ::std::option::Option::None;
    }

    pub fn has_accountid(&self) -> bool {
        self.accountid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accountid(&mut self, v: u32) {
        self.accountid = ::std::option::Option::Some(v);
    }

    // optional fixed32 rtime32_creation_time = 2;

    pub fn rtime32_creation_time(&self) -> u32 {
        self.rtime32_creation_time.unwrap_or(0)
    }

    pub fn clear_rtime32_creation_time(&mut self) {
        self.rtime32_creation_time = ::std::option::Option::None;
    }

    pub fn has_rtime32_creation_time(&self) -> bool {
        self.rtime32_creation_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rtime32_creation_time(&mut self, v: u32) {
        self.rtime32_creation_time = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for COffline_OfflineLogonTicket {
    const NAME: &'static str = "COffline_OfflineLogonTicket";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.accountid = ::std::option::Option::Some(is.read_uint32()?);
                },
                21 => {
                    self.rtime32_creation_time = ::std::option::Option::Some(is.read_fixed32()?);
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
        if let Some(v) = self.accountid {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(1, v);
        }
        if let Some(v) = self.rtime32_creation_time {
            my_size += 1 + 4;
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.accountid {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.rtime32_creation_time {
            os.write_fixed32(2, v)?;
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

    fn new() -> COffline_OfflineLogonTicket {
        COffline_OfflineLogonTicket::new()
    }

    fn clear(&mut self) {
        self.accountid = ::std::option::Option::None;
        self.rtime32_creation_time = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static COffline_OfflineLogonTicket {
        static instance: COffline_OfflineLogonTicket = COffline_OfflineLogonTicket {
            accountid: ::std::option::Option::None,
            rtime32_creation_time: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:COffline_GetUnsignedOfflineLogonTicket_Response)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct COffline_GetUnsignedOfflineLogonTicket_Response {
    // message fields
    // @@protoc_insertion_point(field:COffline_GetUnsignedOfflineLogonTicket_Response.ticket)
    pub ticket: ::steam_vent_proto_common::protobuf::MessageField<COffline_OfflineLogonTicket>,
    // special fields
    // @@protoc_insertion_point(special_field:COffline_GetUnsignedOfflineLogonTicket_Response.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a COffline_GetUnsignedOfflineLogonTicket_Response {
    fn default() -> &'a COffline_GetUnsignedOfflineLogonTicket_Response {
        <COffline_GetUnsignedOfflineLogonTicket_Response as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl COffline_GetUnsignedOfflineLogonTicket_Response {
    pub fn new() -> COffline_GetUnsignedOfflineLogonTicket_Response {
        ::std::default::Default::default()
    }
}

impl ::steam_vent_proto_common::protobuf::Message for COffline_GetUnsignedOfflineLogonTicket_Response {
    const NAME: &'static str = "COffline_GetUnsignedOfflineLogonTicket_Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.ticket)?;
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
        if let Some(v) = self.ticket.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.ticket.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> COffline_GetUnsignedOfflineLogonTicket_Response {
        COffline_GetUnsignedOfflineLogonTicket_Response::new()
    }

    fn clear(&mut self) {
        self.ticket.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static COffline_GetUnsignedOfflineLogonTicket_Response {
        static instance: COffline_GetUnsignedOfflineLogonTicket_Response = COffline_GetUnsignedOfflineLogonTicket_Response {
            ticket: ::steam_vent_proto_common::protobuf::MessageField::none(),
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
#[allow(unused_imports)]
use crate::offline_ticket::*;
impl ::steam_vent_proto_common::RpcMessage for COffline_GetOfflineLogonTicket_Request {
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
impl ::steam_vent_proto_common::RpcMessage for COffline_GetOfflineLogonTicket_Response {
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
for COffline_GetUnsignedOfflineLogonTicket_Request {
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
impl ::steam_vent_proto_common::RpcMessage for COffline_OfflineLogonTicket {
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
for COffline_GetUnsignedOfflineLogonTicket_Response {
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
struct Offline {}
impl ::steam_vent_proto_common::RpcService for Offline {
    const SERVICE_NAME: &'static str = "Offline";
}
impl ::steam_vent_proto_common::RpcMethod for COffline_GetOfflineLogonTicket_Request {
    const METHOD_NAME: &'static str = "Offline.GetOfflineLogonTicket#1";
    type Response = COffline_GetOfflineLogonTicket_Response;
}
impl ::steam_vent_proto_common::RpcMethod
for COffline_GetUnsignedOfflineLogonTicket_Request {
    const METHOD_NAME: &'static str = "Offline.GetUnsignedOfflineLogonTicket#1";
    type Response = COffline_GetUnsignedOfflineLogonTicket_Response;
}
