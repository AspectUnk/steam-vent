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

//! Generated file from `webuimessages_sharedjscontext.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::steam_vent_proto_common::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:CSharedJSContext_GetDesiredSteamUIWindows_Request)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CSharedJSContext_GetDesiredSteamUIWindows_Request {
    // special fields
    // @@protoc_insertion_point(special_field:CSharedJSContext_GetDesiredSteamUIWindows_Request.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CSharedJSContext_GetDesiredSteamUIWindows_Request {
    fn default() -> &'a CSharedJSContext_GetDesiredSteamUIWindows_Request {
        <CSharedJSContext_GetDesiredSteamUIWindows_Request as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CSharedJSContext_GetDesiredSteamUIWindows_Request {
    pub fn new() -> CSharedJSContext_GetDesiredSteamUIWindows_Request {
        ::std::default::Default::default()
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CSharedJSContext_GetDesiredSteamUIWindows_Request {
    const NAME: &'static str = "CSharedJSContext_GetDesiredSteamUIWindows_Request";

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

    fn new() -> CSharedJSContext_GetDesiredSteamUIWindows_Request {
        CSharedJSContext_GetDesiredSteamUIWindows_Request::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CSharedJSContext_GetDesiredSteamUIWindows_Request {
        static instance: CSharedJSContext_GetDesiredSteamUIWindows_Request = CSharedJSContext_GetDesiredSteamUIWindows_Request {
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CMsgSteamUIBrowserWindow)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CMsgSteamUIBrowserWindow {
    // message fields
    // @@protoc_insertion_point(field:CMsgSteamUIBrowserWindow.id)
    pub id: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CMsgSteamUIBrowserWindow.pid)
    pub pid: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CMsgSteamUIBrowserWindow.browser_id)
    pub browser_id: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CMsgSteamUIBrowserWindow.window_type)
    pub window_type: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CMsgSteamUIBrowserWindow.x)
    pub x: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CMsgSteamUIBrowserWindow.y)
    pub y: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CMsgSteamUIBrowserWindow.appid)
    pub appid: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:CMsgSteamUIBrowserWindow.parent_window_handle)
    pub parent_window_handle: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:CMsgSteamUIBrowserWindow.app_name)
    pub app_name: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:CMsgSteamUIBrowserWindow.gamepadui_via_gamescope)
    pub gamepadui_via_gamescope: ::std::option::Option<bool>,
    // special fields
    // @@protoc_insertion_point(special_field:CMsgSteamUIBrowserWindow.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CMsgSteamUIBrowserWindow {
    fn default() -> &'a CMsgSteamUIBrowserWindow {
        <CMsgSteamUIBrowserWindow as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CMsgSteamUIBrowserWindow {
    pub fn new() -> CMsgSteamUIBrowserWindow {
        ::std::default::Default::default()
    }

    // optional int32 id = 1;

    pub fn id(&self) -> i32 {
        self.id.unwrap_or(0)
    }

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = ::std::option::Option::Some(v);
    }

    // optional int32 pid = 2;

    pub fn pid(&self) -> i32 {
        self.pid.unwrap_or(0)
    }

    pub fn clear_pid(&mut self) {
        self.pid = ::std::option::Option::None;
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: i32) {
        self.pid = ::std::option::Option::Some(v);
    }

    // optional int32 browser_id = 3;

    pub fn browser_id(&self) -> i32 {
        self.browser_id.unwrap_or(0)
    }

    pub fn clear_browser_id(&mut self) {
        self.browser_id = ::std::option::Option::None;
    }

    pub fn has_browser_id(&self) -> bool {
        self.browser_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_browser_id(&mut self, v: i32) {
        self.browser_id = ::std::option::Option::Some(v);
    }

    // optional int32 window_type = 4;

    pub fn window_type(&self) -> i32 {
        self.window_type.unwrap_or(0)
    }

    pub fn clear_window_type(&mut self) {
        self.window_type = ::std::option::Option::None;
    }

    pub fn has_window_type(&self) -> bool {
        self.window_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_window_type(&mut self, v: i32) {
        self.window_type = ::std::option::Option::Some(v);
    }

    // optional int32 x = 5;

    pub fn x(&self) -> i32 {
        self.x.unwrap_or(0)
    }

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = ::std::option::Option::Some(v);
    }

    // optional int32 y = 6;

    pub fn y(&self) -> i32 {
        self.y.unwrap_or(0)
    }

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = ::std::option::Option::Some(v);
    }

    // optional uint64 appid = 7;

    pub fn appid(&self) -> u64 {
        self.appid.unwrap_or(0)
    }

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u64) {
        self.appid = ::std::option::Option::Some(v);
    }

    // optional uint64 parent_window_handle = 8;

    pub fn parent_window_handle(&self) -> u64 {
        self.parent_window_handle.unwrap_or(0)
    }

    pub fn clear_parent_window_handle(&mut self) {
        self.parent_window_handle = ::std::option::Option::None;
    }

    pub fn has_parent_window_handle(&self) -> bool {
        self.parent_window_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_window_handle(&mut self, v: u64) {
        self.parent_window_handle = ::std::option::Option::Some(v);
    }

    // optional string app_name = 9;

    pub fn app_name(&self) -> &str {
        match self.app_name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_app_name(&mut self) {
        self.app_name = ::std::option::Option::None;
    }

    pub fn has_app_name(&self) -> bool {
        self.app_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_name(&mut self, v: ::std::string::String) {
        self.app_name = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_app_name(&mut self) -> &mut ::std::string::String {
        if self.app_name.is_none() {
            self.app_name = ::std::option::Option::Some(::std::string::String::new());
        }
        self.app_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_app_name(&mut self) -> ::std::string::String {
        self.app_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional bool gamepadui_via_gamescope = 10;

    pub fn gamepadui_via_gamescope(&self) -> bool {
        self.gamepadui_via_gamescope.unwrap_or(false)
    }

    pub fn clear_gamepadui_via_gamescope(&mut self) {
        self.gamepadui_via_gamescope = ::std::option::Option::None;
    }

    pub fn has_gamepadui_via_gamescope(&self) -> bool {
        self.gamepadui_via_gamescope.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gamepadui_via_gamescope(&mut self, v: bool) {
        self.gamepadui_via_gamescope = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CMsgSteamUIBrowserWindow {
    const NAME: &'static str = "CMsgSteamUIBrowserWindow";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.id = ::std::option::Option::Some(is.read_int32()?);
                },
                16 => {
                    self.pid = ::std::option::Option::Some(is.read_int32()?);
                },
                24 => {
                    self.browser_id = ::std::option::Option::Some(is.read_int32()?);
                },
                32 => {
                    self.window_type = ::std::option::Option::Some(is.read_int32()?);
                },
                40 => {
                    self.x = ::std::option::Option::Some(is.read_int32()?);
                },
                48 => {
                    self.y = ::std::option::Option::Some(is.read_int32()?);
                },
                56 => {
                    self.appid = ::std::option::Option::Some(is.read_uint64()?);
                },
                64 => {
                    self.parent_window_handle = ::std::option::Option::Some(is.read_uint64()?);
                },
                74 => {
                    self.app_name = ::std::option::Option::Some(is.read_string()?);
                },
                80 => {
                    self.gamepadui_via_gamescope = ::std::option::Option::Some(is.read_bool()?);
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
        if let Some(v) = self.id {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(1, v);
        }
        if let Some(v) = self.pid {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(2, v);
        }
        if let Some(v) = self.browser_id {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(3, v);
        }
        if let Some(v) = self.window_type {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(4, v);
        }
        if let Some(v) = self.x {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(5, v);
        }
        if let Some(v) = self.y {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(6, v);
        }
        if let Some(v) = self.appid {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint64_size(7, v);
        }
        if let Some(v) = self.parent_window_handle {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint64_size(8, v);
        }
        if let Some(v) = self.app_name.as_ref() {
            my_size += ::steam_vent_proto_common::protobuf::rt::string_size(9, &v);
        }
        if let Some(v) = self.gamepadui_via_gamescope {
            my_size += 1 + 1;
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.id {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.pid {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.browser_id {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.window_type {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.x {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.y {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.appid {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.parent_window_handle {
            os.write_uint64(8, v)?;
        }
        if let Some(v) = self.app_name.as_ref() {
            os.write_string(9, v)?;
        }
        if let Some(v) = self.gamepadui_via_gamescope {
            os.write_bool(10, v)?;
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

    fn new() -> CMsgSteamUIBrowserWindow {
        CMsgSteamUIBrowserWindow::new()
    }

    fn clear(&mut self) {
        self.id = ::std::option::Option::None;
        self.pid = ::std::option::Option::None;
        self.browser_id = ::std::option::Option::None;
        self.window_type = ::std::option::Option::None;
        self.x = ::std::option::Option::None;
        self.y = ::std::option::Option::None;
        self.appid = ::std::option::Option::None;
        self.parent_window_handle = ::std::option::Option::None;
        self.app_name = ::std::option::Option::None;
        self.gamepadui_via_gamescope = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CMsgSteamUIBrowserWindow {
        static instance: CMsgSteamUIBrowserWindow = CMsgSteamUIBrowserWindow {
            id: ::std::option::Option::None,
            pid: ::std::option::Option::None,
            browser_id: ::std::option::Option::None,
            window_type: ::std::option::Option::None,
            x: ::std::option::Option::None,
            y: ::std::option::Option::None,
            appid: ::std::option::Option::None,
            parent_window_handle: ::std::option::Option::None,
            app_name: ::std::option::Option::None,
            gamepadui_via_gamescope: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CSharedJSContext_GetDesiredSteamUIWindows_Response)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CSharedJSContext_GetDesiredSteamUIWindows_Response {
    // message fields
    // @@protoc_insertion_point(field:CSharedJSContext_GetDesiredSteamUIWindows_Response.windows)
    pub windows: ::std::vec::Vec<CMsgSteamUIBrowserWindow>,
    // special fields
    // @@protoc_insertion_point(special_field:CSharedJSContext_GetDesiredSteamUIWindows_Response.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CSharedJSContext_GetDesiredSteamUIWindows_Response {
    fn default() -> &'a CSharedJSContext_GetDesiredSteamUIWindows_Response {
        <CSharedJSContext_GetDesiredSteamUIWindows_Response as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CSharedJSContext_GetDesiredSteamUIWindows_Response {
    pub fn new() -> CSharedJSContext_GetDesiredSteamUIWindows_Response {
        ::std::default::Default::default()
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CSharedJSContext_GetDesiredSteamUIWindows_Response {
    const NAME: &'static str = "CSharedJSContext_GetDesiredSteamUIWindows_Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.windows.push(is.read_message()?);
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
        for value in &self.windows {
            let len = value.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        for v in &self.windows {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> CSharedJSContext_GetDesiredSteamUIWindows_Response {
        CSharedJSContext_GetDesiredSteamUIWindows_Response::new()
    }

    fn clear(&mut self) {
        self.windows.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CSharedJSContext_GetDesiredSteamUIWindows_Response {
        static instance: CSharedJSContext_GetDesiredSteamUIWindows_Response = CSharedJSContext_GetDesiredSteamUIWindows_Response {
            windows: ::std::vec::Vec::new(),
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}


const _VENT_PROTO_VERSION_CHECK: () = ::steam_vent_proto_common::VERSION_0_5_0;

#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use crate::steammessages_base::*;
#[allow(unused_imports)]
use crate::webuimessages_base::*;
impl ::steam_vent_proto_common::RpcMessage
for CSharedJSContext_GetDesiredSteamUIWindows_Request {
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
impl ::steam_vent_proto_common::RpcMessage for CMsgSteamUIBrowserWindow {
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
for CSharedJSContext_GetDesiredSteamUIWindows_Response {
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
struct SharedJSContext {}
impl ::steam_vent_proto_common::RpcService for SharedJSContext {
    const SERVICE_NAME: &'static str = "SharedJSContext";
}
impl ::steam_vent_proto_common::RpcMethod
for CSharedJSContext_GetDesiredSteamUIWindows_Request {
    const METHOD_NAME: &'static str = "SharedJSContext.GetDesiredSteamUIWindows#1";
    type Response = CSharedJSContext_GetDesiredSteamUIWindows_Response;
}
