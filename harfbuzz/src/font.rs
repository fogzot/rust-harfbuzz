// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Face;
use std::marker::PhantomData;
use std::mem;
use sys;

/// A font object represents a font face at a specific size and with
/// certain other parameters (pixels-per-em, points-per-em, variation
/// settings) specified. `Font` objects are created from font `Face`
/// objects, and are used as input to shaping functions, among other
/// things.
pub struct Font<'a> {
    raw: *mut sys::hb_font_t,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a> Font<'a> {
    /// Constructs a new font object from the specified face.
    ///
    /// ```
    /// # use harfbuzz::{Face, Font};
    /// let face = Face::new_from_file("../harfbuzz-sys/harfbuzz/test/api/fonts/SourceSansPro-Regular.otf", 0).unwrap();
    /// let font = Font::new(&face);
    /// assert_eq!(font.get_scale(), (1000, 1000));
    /// ```
    pub fn new(face: &Face) -> Font<'static> {
        unsafe {
            let font = sys::hb_font_create(face.as_ptr());
            Font::from_raw(font)
        }
    }

    /// Construct a `Font` from a raw pointer. Takes ownership of the font.
    pub unsafe fn from_raw(raw: *mut sys::hb_font_t) -> Self {
        Font {
            raw,
            phantom: PhantomData,
        }
    }

    /// Sets the horizontal and vertical pixels-per-em (PPEM) of a font.
    pub fn set_ppem(&mut self, x_ppem: u32, y_ppem: u32) {
        unsafe {
            sys::hb_font_set_ppem(self.raw, x_ppem, y_ppem);
        }
    }

    /// Fetches the horizontal and vertical points-per-em (PPEM) of a font.
    pub fn get_ppem(&self) -> (u32, u32) {
        unsafe {
            let mut x_ppem: u32 = 0;
            let mut y_ppem: u32 = 0;
            sys::hb_font_get_ppem(self.raw, &mut x_ppem, &mut y_ppem);
            (x_ppem, y_ppem)
        }
    }

    /// Sets the "point size" of a font. Set to zero to unset.
    ///
    /// Used in CoreText to implement optical sizing.
    pub fn set_ptem(&mut self, ptem: f32) {
        unsafe {
            sys::hb_font_set_ptem(self.raw, ptem);
        }
    }

    /// Fetches the "point size" of a font.
    ///
    /// Used in CoreText to implement optical sizing.
    pub fn get_ptem(&self) -> f32 {
        unsafe {
            sys::hb_font_get_ptem(self.raw)
        }
    }

    /// Sets the horizontal and vertical scale of a font.
    pub fn set_scale(&mut self, x_scale: i32, y_scale: i32) {
        unsafe {
            sys::hb_font_set_scale(self.raw, x_scale, y_scale);
        }
    }

    /// Fetches the horizontal and vertical scale of a font.
    pub fn get_scale(&self) -> (i32, i32) {
        unsafe {
            let mut x_scale: i32 = 0;
            let mut y_scale: i32 = 0;
            sys::hb_font_get_scale(self.raw, &mut x_scale, &mut y_scale);
            (x_scale, y_scale)
        }
    }

    /// Make this font immutable.
    pub fn make_immutable(&mut self) {
        unsafe {
            sys::hb_font_make_immutable(self.raw);
        }
    }

    /// Returns true if the font is immutable.
    pub fn is_immutable(&self) -> bool {
        unsafe { sys::hb_font_is_immutable(self.raw) != 0 }
    }

    /// Borrows a raw pointer to the font.
    pub fn as_ptr(&self) -> *mut sys::hb_font_t {
        self.raw
    }

    /// Gives up ownership and returns a raw pointer to the font.
    pub fn into_raw(self) -> *mut sys::hb_font_t {
        let raw = self.raw;
        mem::forget(self);
        raw
    }
}

impl<'a> Drop for Font<'a> {
    /// Decrement the reference count, and destroy the font if the reference count is zero.
    fn drop(&mut self) {
        unsafe {
            sys::hb_font_destroy(self.raw);
        }
    }
}
