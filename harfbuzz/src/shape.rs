// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::marker::PhantomData;
use std::mem::transmute;
use std::slice;
use sys;

use crate::{Font, Buffer, Feature};

/// Shapes buffer using font turning its Unicode characters content to
/// positioned glyphs.
///
/// If features is not NULL, it will be used to control the features
/// applied during shaping. If two features have the same tag but
/// overlapping ranges the value of the feature with the higher
/// index takes precedence.
///
/// Note that the buffer given as argument is consumed and a
/// `ShapedBuffer` is returned in its place. A call to
/// `clear_contents` is necessary to revert the ·`ShapedBuffer`
/// to a `Buffer` able to hold unicode characters.
pub fn hb_shape<'a>(font: &Font, buffer: Buffer, features: &[Feature]) -> ShapedBuffer<'a> {
    unsafe {
        sys::hb_shape(font.as_ptr(), buffer.as_ptr(), features.as_ptr() as *const sys::hb_feature_t, features.len() as u32);
        ShapedBuffer::from_raw(buffer.into_raw())
    }
}

/// A series of positioned glyphs.
pub struct ShapedBuffer<'a> {
    /// The underlying `hb_buffer_t` from the `harfbuzz-sys` crate.
    ///
    /// This isn't commonly needed unless interfacing directly with
    /// functions from the `harfbuzz-sys` crate that haven't been
    /// safely exposed.
    raw: *mut sys::hb_buffer_t,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a> ShapedBuffer<'a> {

    /// Construct a `Buffer` from a raw pointer. Takes ownership of the buffer.
    pub unsafe fn from_raw(raw: *mut sys::hb_buffer_t) -> Self {
        ShapedBuffer { raw, phantom: PhantomData }
    }

    /// Throw away glyph information stored in the buffer, but maintain the
    /// currently configured Unicode functions and flags.
    pub fn clear_contents(self) -> Buffer {
        unsafe { Buffer::from_raw(self.into_raw()) }
    }

    /// Retrieve glyph information from the buffer.
    pub fn get_glyph_infos(&self) -> &'a [GlyphInfo] {
        let mut length: u32 = 0;
        unsafe {
            let info = sys::hb_buffer_get_glyph_infos(self.as_ptr(), &mut length);
            slice::from_raw_parts(transmute(info), length as usize)
        }
    }

    /// Retrieve glyph positions from the buffer.
    pub fn get_glyph_positions(&self) -> &'a [GlyphPosition] {
        let mut length: u32 = 0;
        unsafe {
            let positions = sys::hb_buffer_get_glyph_positions(self.as_ptr(), &mut length);
            slice::from_raw_parts(transmute(positions), length as usize)
        }
    }

    /// Borrows a raw pointer to the buffer.
    pub fn as_ptr(&self) -> *mut sys::hb_buffer_t {
        self.raw
    }

    /// Gives up ownership and returns a raw pointer to the buffer.
    pub fn into_raw(self) -> *mut sys::hb_buffer_t {
        let raw = self.raw;
        std::mem::forget(self);
        raw
    }
}

/// Glyph information
#[repr(C, packed)]
pub struct GlyphInfo {
    codepoint: sys::hb_codepoint_t,
    mask: sys::hb_mask_t,
    cluster: u32,
    var1: sys::hb_var_int_t,
    var2: sys::hb_var_int_t,
}

impl GlyphInfo {
    pub fn index(&self) -> u32 {
        self.codepoint
    }

    pub fn cluster(&self) -> u32 {
        self.cluster
    }
}

/// Glyph position
#[repr(C, packed)]
pub struct GlyphPosition {
    pub x_advance: sys::hb_position_t,
    pub y_advance: sys::hb_position_t,
    pub x_offset: sys::hb_position_t,
    pub y_offset: sys::hb_position_t,
    pub var: sys::hb_var_int_t,
}

impl GlyphPosition {
    pub fn x_advance(&self) -> i32 {
        self.x_advance
    }

    pub fn y_advance(&self) -> i32 {
        self.y_advance
    }

    pub fn x_offset(&self) -> i32 {
        self.x_offset
    }

    pub fn y_offset(&self) -> i32 {
        self.y_offset
    }
}