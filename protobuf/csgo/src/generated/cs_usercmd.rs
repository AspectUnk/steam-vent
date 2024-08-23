// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `cs_usercmd.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::steam_vent_proto_common::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CSGOInterpolationInfoPB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CSGOInterpolationInfoPB {
    // message fields
    // @@protoc_insertion_point(field:CSGOInterpolationInfoPB.src_tick)
    pub src_tick: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CSGOInterpolationInfoPB.dst_tick)
    pub dst_tick: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CSGOInterpolationInfoPB.frac)
    pub frac: ::std::option::Option<f32>,
    // special fields
    // @@protoc_insertion_point(special_field:CSGOInterpolationInfoPB.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CSGOInterpolationInfoPB {
    fn default() -> &'a CSGOInterpolationInfoPB {
        <CSGOInterpolationInfoPB as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CSGOInterpolationInfoPB {
    pub fn new() -> CSGOInterpolationInfoPB {
        ::std::default::Default::default()
    }

    // optional int32 src_tick = 1;

    pub fn src_tick(&self) -> i32 {
        self.src_tick.unwrap_or(-1i32)
    }

    pub fn clear_src_tick(&mut self) {
        self.src_tick = ::std::option::Option::None;
    }

    pub fn has_src_tick(&self) -> bool {
        self.src_tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src_tick(&mut self, v: i32) {
        self.src_tick = ::std::option::Option::Some(v);
    }

    // optional int32 dst_tick = 2;

    pub fn dst_tick(&self) -> i32 {
        self.dst_tick.unwrap_or(-1i32)
    }

    pub fn clear_dst_tick(&mut self) {
        self.dst_tick = ::std::option::Option::None;
    }

    pub fn has_dst_tick(&self) -> bool {
        self.dst_tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dst_tick(&mut self, v: i32) {
        self.dst_tick = ::std::option::Option::Some(v);
    }

    // optional float frac = 3;

    pub fn frac(&self) -> f32 {
        self.frac.unwrap_or(0.0f32)
    }

    pub fn clear_frac(&mut self) {
        self.frac = ::std::option::Option::None;
    }

    pub fn has_frac(&self) -> bool {
        self.frac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frac(&mut self, v: f32) {
        self.frac = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CSGOInterpolationInfoPB {
    const NAME: &'static str = "CSGOInterpolationInfoPB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.src_tick = ::std::option::Option::Some(is.read_int32()?);
                },
                16 => {
                    self.dst_tick = ::std::option::Option::Some(is.read_int32()?);
                },
                29 => {
                    self.frac = ::std::option::Option::Some(is.read_float()?);
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
        if let Some(v) = self.src_tick {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(1, v);
        }
        if let Some(v) = self.dst_tick {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(2, v);
        }
        if let Some(v) = self.frac {
            my_size += 1 + 4;
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.src_tick {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.dst_tick {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.frac {
            os.write_float(3, v)?;
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

    fn new() -> CSGOInterpolationInfoPB {
        CSGOInterpolationInfoPB::new()
    }

    fn clear(&mut self) {
        self.src_tick = ::std::option::Option::None;
        self.dst_tick = ::std::option::Option::None;
        self.frac = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CSGOInterpolationInfoPB {
        static instance: CSGOInterpolationInfoPB = CSGOInterpolationInfoPB {
            src_tick: ::std::option::Option::None,
            dst_tick: ::std::option::Option::None,
            frac: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CSGOInputHistoryEntryPB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CSGOInputHistoryEntryPB {
    // message fields
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.view_angles)
    pub view_angles: ::steam_vent_proto_common::protobuf::MessageField<super::networkbasetypes::CMsgQAngle>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.render_tick_count)
    pub render_tick_count: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.render_tick_fraction)
    pub render_tick_fraction: ::std::option::Option<f32>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.player_tick_count)
    pub player_tick_count: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.player_tick_fraction)
    pub player_tick_fraction: ::std::option::Option<f32>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.cl_interp)
    pub cl_interp: ::steam_vent_proto_common::protobuf::MessageField<CSGOInterpolationInfoPB>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.sv_interp0)
    pub sv_interp0: ::steam_vent_proto_common::protobuf::MessageField<CSGOInterpolationInfoPB>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.sv_interp1)
    pub sv_interp1: ::steam_vent_proto_common::protobuf::MessageField<CSGOInterpolationInfoPB>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.player_interp)
    pub player_interp: ::steam_vent_proto_common::protobuf::MessageField<CSGOInterpolationInfoPB>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.frame_number)
    pub frame_number: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.target_ent_index)
    pub target_ent_index: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.shoot_position)
    pub shoot_position: ::steam_vent_proto_common::protobuf::MessageField<super::networkbasetypes::CMsgVector>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.target_head_pos_check)
    pub target_head_pos_check: ::steam_vent_proto_common::protobuf::MessageField<super::networkbasetypes::CMsgVector>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.target_abs_pos_check)
    pub target_abs_pos_check: ::steam_vent_proto_common::protobuf::MessageField<super::networkbasetypes::CMsgVector>,
    // @@protoc_insertion_point(field:CSGOInputHistoryEntryPB.target_abs_ang_check)
    pub target_abs_ang_check: ::steam_vent_proto_common::protobuf::MessageField<super::networkbasetypes::CMsgQAngle>,
    // special fields
    // @@protoc_insertion_point(special_field:CSGOInputHistoryEntryPB.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CSGOInputHistoryEntryPB {
    fn default() -> &'a CSGOInputHistoryEntryPB {
        <CSGOInputHistoryEntryPB as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CSGOInputHistoryEntryPB {
    pub fn new() -> CSGOInputHistoryEntryPB {
        ::std::default::Default::default()
    }

    // optional int32 render_tick_count = 4;

    pub fn render_tick_count(&self) -> i32 {
        self.render_tick_count.unwrap_or(0)
    }

    pub fn clear_render_tick_count(&mut self) {
        self.render_tick_count = ::std::option::Option::None;
    }

    pub fn has_render_tick_count(&self) -> bool {
        self.render_tick_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_render_tick_count(&mut self, v: i32) {
        self.render_tick_count = ::std::option::Option::Some(v);
    }

    // optional float render_tick_fraction = 5;

    pub fn render_tick_fraction(&self) -> f32 {
        self.render_tick_fraction.unwrap_or(0.)
    }

    pub fn clear_render_tick_fraction(&mut self) {
        self.render_tick_fraction = ::std::option::Option::None;
    }

    pub fn has_render_tick_fraction(&self) -> bool {
        self.render_tick_fraction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_render_tick_fraction(&mut self, v: f32) {
        self.render_tick_fraction = ::std::option::Option::Some(v);
    }

    // optional int32 player_tick_count = 6;

    pub fn player_tick_count(&self) -> i32 {
        self.player_tick_count.unwrap_or(0)
    }

    pub fn clear_player_tick_count(&mut self) {
        self.player_tick_count = ::std::option::Option::None;
    }

    pub fn has_player_tick_count(&self) -> bool {
        self.player_tick_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_tick_count(&mut self, v: i32) {
        self.player_tick_count = ::std::option::Option::Some(v);
    }

    // optional float player_tick_fraction = 7;

    pub fn player_tick_fraction(&self) -> f32 {
        self.player_tick_fraction.unwrap_or(0.)
    }

    pub fn clear_player_tick_fraction(&mut self) {
        self.player_tick_fraction = ::std::option::Option::None;
    }

    pub fn has_player_tick_fraction(&self) -> bool {
        self.player_tick_fraction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_tick_fraction(&mut self, v: f32) {
        self.player_tick_fraction = ::std::option::Option::Some(v);
    }

    // optional int32 frame_number = 20;

    pub fn frame_number(&self) -> i32 {
        self.frame_number.unwrap_or(0)
    }

    pub fn clear_frame_number(&mut self) {
        self.frame_number = ::std::option::Option::None;
    }

    pub fn has_frame_number(&self) -> bool {
        self.frame_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frame_number(&mut self, v: i32) {
        self.frame_number = ::std::option::Option::Some(v);
    }

    // optional int32 target_ent_index = 8;

    pub fn target_ent_index(&self) -> i32 {
        self.target_ent_index.unwrap_or(-1i32)
    }

    pub fn clear_target_ent_index(&mut self) {
        self.target_ent_index = ::std::option::Option::None;
    }

    pub fn has_target_ent_index(&self) -> bool {
        self.target_ent_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_ent_index(&mut self, v: i32) {
        self.target_ent_index = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CSGOInputHistoryEntryPB {
    const NAME: &'static str = "CSGOInputHistoryEntryPB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.view_angles)?;
                },
                32 => {
                    self.render_tick_count = ::std::option::Option::Some(is.read_int32()?);
                },
                45 => {
                    self.render_tick_fraction = ::std::option::Option::Some(is.read_float()?);
                },
                48 => {
                    self.player_tick_count = ::std::option::Option::Some(is.read_int32()?);
                },
                61 => {
                    self.player_tick_fraction = ::std::option::Option::Some(is.read_float()?);
                },
                98 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.cl_interp)?;
                },
                106 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.sv_interp0)?;
                },
                114 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.sv_interp1)?;
                },
                122 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.player_interp)?;
                },
                160 => {
                    self.frame_number = ::std::option::Option::Some(is.read_int32()?);
                },
                64 => {
                    self.target_ent_index = ::std::option::Option::Some(is.read_int32()?);
                },
                26 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.shoot_position)?;
                },
                74 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.target_head_pos_check)?;
                },
                82 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.target_abs_pos_check)?;
                },
                90 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.target_abs_ang_check)?;
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
        if let Some(v) = self.view_angles.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.render_tick_count {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(4, v);
        }
        if let Some(v) = self.render_tick_fraction {
            my_size += 1 + 4;
        }
        if let Some(v) = self.player_tick_count {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(6, v);
        }
        if let Some(v) = self.player_tick_fraction {
            my_size += 1 + 4;
        }
        if let Some(v) = self.cl_interp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.sv_interp0.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.sv_interp1.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.player_interp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.frame_number {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(20, v);
        }
        if let Some(v) = self.target_ent_index {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(8, v);
        }
        if let Some(v) = self.shoot_position.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.target_head_pos_check.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.target_abs_pos_check.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.target_abs_ang_check.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.view_angles.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.render_tick_count {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.render_tick_fraction {
            os.write_float(5, v)?;
        }
        if let Some(v) = self.player_tick_count {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.player_tick_fraction {
            os.write_float(7, v)?;
        }
        if let Some(v) = self.cl_interp.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if let Some(v) = self.sv_interp0.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.sv_interp1.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.player_interp.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.frame_number {
            os.write_int32(20, v)?;
        }
        if let Some(v) = self.target_ent_index {
            os.write_int32(8, v)?;
        }
        if let Some(v) = self.shoot_position.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.target_head_pos_check.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.target_abs_pos_check.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.target_abs_ang_check.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> CSGOInputHistoryEntryPB {
        CSGOInputHistoryEntryPB::new()
    }

    fn clear(&mut self) {
        self.view_angles.clear();
        self.render_tick_count = ::std::option::Option::None;
        self.render_tick_fraction = ::std::option::Option::None;
        self.player_tick_count = ::std::option::Option::None;
        self.player_tick_fraction = ::std::option::Option::None;
        self.cl_interp.clear();
        self.sv_interp0.clear();
        self.sv_interp1.clear();
        self.player_interp.clear();
        self.frame_number = ::std::option::Option::None;
        self.target_ent_index = ::std::option::Option::None;
        self.shoot_position.clear();
        self.target_head_pos_check.clear();
        self.target_abs_pos_check.clear();
        self.target_abs_ang_check.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CSGOInputHistoryEntryPB {
        static instance: CSGOInputHistoryEntryPB = CSGOInputHistoryEntryPB {
            view_angles: ::steam_vent_proto_common::protobuf::MessageField::none(),
            render_tick_count: ::std::option::Option::None,
            render_tick_fraction: ::std::option::Option::None,
            player_tick_count: ::std::option::Option::None,
            player_tick_fraction: ::std::option::Option::None,
            cl_interp: ::steam_vent_proto_common::protobuf::MessageField::none(),
            sv_interp0: ::steam_vent_proto_common::protobuf::MessageField::none(),
            sv_interp1: ::steam_vent_proto_common::protobuf::MessageField::none(),
            player_interp: ::steam_vent_proto_common::protobuf::MessageField::none(),
            frame_number: ::std::option::Option::None,
            target_ent_index: ::std::option::Option::None,
            shoot_position: ::steam_vent_proto_common::protobuf::MessageField::none(),
            target_head_pos_check: ::steam_vent_proto_common::protobuf::MessageField::none(),
            target_abs_pos_check: ::steam_vent_proto_common::protobuf::MessageField::none(),
            target_abs_ang_check: ::steam_vent_proto_common::protobuf::MessageField::none(),
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CSGOUserCmdPB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CSGOUserCmdPB {
    // message fields
    // @@protoc_insertion_point(field:CSGOUserCmdPB.base)
    pub base: ::steam_vent_proto_common::protobuf::MessageField<super::usercmd::CBaseUserCmdPB>,
    // @@protoc_insertion_point(field:CSGOUserCmdPB.input_history)
    pub input_history: ::std::vec::Vec<CSGOInputHistoryEntryPB>,
    // @@protoc_insertion_point(field:CSGOUserCmdPB.attack1_start_history_index)
    pub attack1_start_history_index: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CSGOUserCmdPB.attack2_start_history_index)
    pub attack2_start_history_index: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CSGOUserCmdPB.attack3_start_history_index)
    pub attack3_start_history_index: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:CSGOUserCmdPB.left_hand_desired)
    pub left_hand_desired: ::std::option::Option<bool>,
    // special fields
    // @@protoc_insertion_point(special_field:CSGOUserCmdPB.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CSGOUserCmdPB {
    fn default() -> &'a CSGOUserCmdPB {
        <CSGOUserCmdPB as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CSGOUserCmdPB {
    pub fn new() -> CSGOUserCmdPB {
        ::std::default::Default::default()
    }

    // optional int32 attack1_start_history_index = 6;

    pub fn attack1_start_history_index(&self) -> i32 {
        self.attack1_start_history_index.unwrap_or(-1i32)
    }

    pub fn clear_attack1_start_history_index(&mut self) {
        self.attack1_start_history_index = ::std::option::Option::None;
    }

    pub fn has_attack1_start_history_index(&self) -> bool {
        self.attack1_start_history_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack1_start_history_index(&mut self, v: i32) {
        self.attack1_start_history_index = ::std::option::Option::Some(v);
    }

    // optional int32 attack2_start_history_index = 7;

    pub fn attack2_start_history_index(&self) -> i32 {
        self.attack2_start_history_index.unwrap_or(-1i32)
    }

    pub fn clear_attack2_start_history_index(&mut self) {
        self.attack2_start_history_index = ::std::option::Option::None;
    }

    pub fn has_attack2_start_history_index(&self) -> bool {
        self.attack2_start_history_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack2_start_history_index(&mut self, v: i32) {
        self.attack2_start_history_index = ::std::option::Option::Some(v);
    }

    // optional int32 attack3_start_history_index = 8;

    pub fn attack3_start_history_index(&self) -> i32 {
        self.attack3_start_history_index.unwrap_or(-1i32)
    }

    pub fn clear_attack3_start_history_index(&mut self) {
        self.attack3_start_history_index = ::std::option::Option::None;
    }

    pub fn has_attack3_start_history_index(&self) -> bool {
        self.attack3_start_history_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack3_start_history_index(&mut self, v: i32) {
        self.attack3_start_history_index = ::std::option::Option::Some(v);
    }

    // optional bool left_hand_desired = 9;

    pub fn left_hand_desired(&self) -> bool {
        self.left_hand_desired.unwrap_or(false)
    }

    pub fn clear_left_hand_desired(&mut self) {
        self.left_hand_desired = ::std::option::Option::None;
    }

    pub fn has_left_hand_desired(&self) -> bool {
        self.left_hand_desired.is_some()
    }

    // Param is passed by value, moved
    pub fn set_left_hand_desired(&mut self, v: bool) {
        self.left_hand_desired = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CSGOUserCmdPB {
    const NAME: &'static str = "CSGOUserCmdPB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::steam_vent_proto_common::protobuf::rt::read_singular_message_into_field(is, &mut self.base)?;
                },
                18 => {
                    self.input_history.push(is.read_message()?);
                },
                48 => {
                    self.attack1_start_history_index = ::std::option::Option::Some(is.read_int32()?);
                },
                56 => {
                    self.attack2_start_history_index = ::std::option::Option::Some(is.read_int32()?);
                },
                64 => {
                    self.attack3_start_history_index = ::std::option::Option::Some(is.read_int32()?);
                },
                72 => {
                    self.left_hand_desired = ::std::option::Option::Some(is.read_bool()?);
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
        if let Some(v) = self.base.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.input_history {
            let len = value.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.attack1_start_history_index {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(6, v);
        }
        if let Some(v) = self.attack2_start_history_index {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(7, v);
        }
        if let Some(v) = self.attack3_start_history_index {
            my_size += ::steam_vent_proto_common::protobuf::rt::int32_size(8, v);
        }
        if let Some(v) = self.left_hand_desired {
            my_size += 1 + 1;
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.base.as_ref() {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        for v in &self.input_history {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if let Some(v) = self.attack1_start_history_index {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.attack2_start_history_index {
            os.write_int32(7, v)?;
        }
        if let Some(v) = self.attack3_start_history_index {
            os.write_int32(8, v)?;
        }
        if let Some(v) = self.left_hand_desired {
            os.write_bool(9, v)?;
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

    fn new() -> CSGOUserCmdPB {
        CSGOUserCmdPB::new()
    }

    fn clear(&mut self) {
        self.base.clear();
        self.input_history.clear();
        self.attack1_start_history_index = ::std::option::Option::None;
        self.attack2_start_history_index = ::std::option::Option::None;
        self.attack3_start_history_index = ::std::option::Option::None;
        self.left_hand_desired = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CSGOUserCmdPB {
        static instance: CSGOUserCmdPB = CSGOUserCmdPB {
            base: ::steam_vent_proto_common::protobuf::MessageField::none(),
            input_history: ::std::vec::Vec::new(),
            attack1_start_history_index: ::std::option::Option::None,
            attack2_start_history_index: ::std::option::Option::None,
            attack3_start_history_index: ::std::option::Option::None,
            left_hand_desired: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}


const _VENT_PROTO_VERSION_CHECK: () = ::steam_vent_proto_common::VERSION_0_5_0;

#[allow(unused_imports)]
use crate::networkbasetypes::*;
#[allow(unused_imports)]
use crate::usercmd::*;
impl ::steam_vent_proto_common::RpcMessage for CSGOInterpolationInfoPB {
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
impl ::steam_vent_proto_common::RpcMessage for CSGOInputHistoryEntryPB {
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
impl ::steam_vent_proto_common::RpcMessage for CSGOUserCmdPB {
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
