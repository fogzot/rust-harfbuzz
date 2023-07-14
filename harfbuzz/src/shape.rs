// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

use crate::{Font, Buffer, Feature};

/// Shapes buffer using font turning its Unicode characters content to
/// positioned glyphs.
///
/// If features is not NULL, it will be used to control the features
/// applied during shaping. If two features have the same tag but
/// overlapping ranges the value of the feature with the higher
/// index takes precedence.
pub fn hb_shape(font: &Font, buffer: &Buffer, features: &[Feature]) {
    unsafe {
        sys::hb_shape(font.as_ptr(), buffer.as_ptr(), features.as_ptr() as *const sys::hb_feature_t, features.len() as u32)
    }
}
