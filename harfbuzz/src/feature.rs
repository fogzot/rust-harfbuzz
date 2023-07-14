// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

use sys;

/// This structure holds information about requested feature application.
#[repr(C)]
pub struct Feature {
    hb_feature: sys::hb_feature_t,
}

impl Feature {
    pub fn enable(tag: u32) -> Feature {
        Self { hb_feature: sys::hb_feature_t { tag, value: 1, start: 0, end: u32::MAX }}
    }

    pub fn enable_with_range(tag: u32, start: u32, end: u32) -> Feature {
        Self { hb_feature: sys::hb_feature_t { tag, value: 1, start, end }}
    }

    pub fn disable(tag: u32) -> Feature {
        Self { hb_feature: sys::hb_feature_t { tag, value: 0, start: 0, end: u32::MAX }}
    }

    pub fn disable_with_range(tag: u32, start: u32, end: u32) -> Feature {
        Self { hb_feature: sys::hb_feature_t { tag, value: 0, start, end }}
    }

    pub fn tag(&self) -> u32 {
        self.hb_feature.tag
    }

    pub fn enabled(&self) -> bool {
        self.hb_feature.value == 1
    }

    pub fn disabled(&self) -> bool {
        self.hb_feature.value == 0
    }

    pub fn value(&self) -> u32 {
        self.hb_feature.value
    }

    pub fn start(&self) -> u32 {
        self.hb_feature.start
    }

    pub fn end(&self) -> u32 {
        self.hb_feature.end
    }
}
