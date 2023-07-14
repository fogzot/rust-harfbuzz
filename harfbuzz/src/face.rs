// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Blob;
use std::marker::PhantomData;
use std::mem;
use sys;

/// Face wrap a chunk of binary data to handle lifecycle management of data
/// while it is passed between client and HarfBuzz.
pub struct Face<'a> {
    raw: *mut sys::hb_face_t,
    phantom: PhantomData<&'a [u8]>,
}

impl<'a> Face<'a> {
    /// Create a new font face from the given blob and face index into the blob.
    ///
    /// The face index is used for blobs of file formats such as TTC and DFont
    /// that can contain more than one face.
    ///
    /// Note that the `Blob` is consumed.
    /// ```
    /// # use harfbuzz::{Blob, Face};
    /// let blob = Blob::new_from_file("../harfbuzz-sys/harfbuzz/test/api/fonts/SourceSansPro-Regular.otf").unwrap();
    /// let face = Face::new(blob, 0);
    /// assert_eq!(face.get_glyph_count(), 1942);
    /// ```
    pub fn new(blob: Blob, index: u32) -> Face<'static> {
        unsafe {
            let face = sys::hb_face_create(blob.into_raw(), index);
            Face::from_raw(face)
        }
    }

    /// Create a new font face from the given file and face index into the file.
    ///
    /// The face index is used for files of formats such as TTC and DFont
    /// that can contain more than one face.
    ///
    /// ```
    /// # use harfbuzz::Face;
    /// let face = Face::new_from_file("../harfbuzz-sys/harfbuzz/test/api/fonts/SourceSansPro-Regular.otf", 0).unwrap();
    /// assert_eq!(face.get_glyph_count(), 1942);
    /// ```
    pub fn new_from_file(path: &str, index: u32) -> Option<Face<'static>> {
        let blob = Blob::new_from_file(path)?;
        Some(Face::new(blob, index))
    }

    /// Construct a `Face` from a raw pointer. Takes ownership of the face.
    pub unsafe fn from_raw(raw: *mut sys::hb_face_t) -> Self {
        Face {
            raw,
            phantom: PhantomData,
        }
    }

    /// Assigns the specified face-index to face.
    ///
    /// Fails if the face is immutable.
    pub fn set_index(&self, index: u32) {
        unsafe { sys::hb_face_set_index(self.raw, index) }
    }

    /// Fetches the face-index corresponding to the given face.
    pub fn get_index(&self) -> u32 {
        unsafe { sys::hb_face_get_index(self.raw) }
    }

    /// Fetches the units-per-em (UPEM) value of the specified face object.
    ///
    /// Typical UPEM values for fonts are 1000, or 2048, but any value in
    /// between 16 and 16,384 is allowed for OpenType fonts.
    pub fn get_upem(&self) -> u32 {
        unsafe { sys::hb_face_get_upem(self.raw) }
    }

    /// Sets the units-per-em (upem) for a face object to the specified value.
    pub fn set_upem(&self, index: u32) {
        unsafe { sys::hb_face_set_upem(self.raw, index) }
    }

    /// Fetches the glyph-count value of the face.
    pub fn get_glyph_count(&self) -> u32 {
        unsafe { sys::hb_face_get_glyph_count(self.raw) }
    }

    /// Make this face immutable.
    pub fn make_immutable(&mut self) {
        unsafe {
            sys::hb_face_make_immutable(self.raw);
        }
    }

    /// Returns true if the face is immutable.
    pub fn is_immutable(&self) -> bool {
        unsafe { sys::hb_face_is_immutable(self.raw) != 0 }
    }

    /// Borrows a raw pointer to the face.
    pub fn as_ptr(&self) -> *mut sys::hb_face_t {
        self.raw
    }

    /// Gives up ownership and returns a raw pointer to the face.
    pub fn into_raw(self) -> *mut sys::hb_face_t {
        let raw = self.raw;
        mem::forget(self);
        raw
    }
}

impl<'a> Drop for Face<'a> {
    /// Decrement the reference count, and destroy the face if the reference count is zero.
    fn drop(&mut self) {
        unsafe {
            sys::hb_face_destroy(self.raw);
        }
    }
}
