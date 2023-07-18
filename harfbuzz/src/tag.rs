// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// Compact representation of features, scripts and languages.
pub struct Tag {
    tag: u32,
}

impl Tag {
    /// Constructs a tag from a `u32` value.
    pub fn new(value: u32) -> Tag {
        Tag { tag: value }
    }

    /// Constructs a tag from four character literals.
    pub fn from_chars(c1: char, c2: char, c3: char, c4: char) -> Tag {
        Tag::new((c1 as u32 & 0xff) << 24 | (c2 as u32 & 0xff) << 16 | (c3 as u32 & 0xff) << 8 | (c4 as u32 & 0xff))
    }

    /// Constructs a tag from a string.
    ///
    /// Valid tags are four characters. Shorter input strings will be
    /// padded with spaces. Longer input strings will be truncated.
    pub fn from_string(s: &str) -> Tag {
        let mut value: u32 = 0;
        let mut cs = s.chars();
        value |= match cs.next() { Some(c) => c as u32, None => 0x20 } << 24;
        value |= match cs.next() { Some(c) => c as u32, None => 0x20 } << 16;
        value |= match cs.next() { Some(c) => c as u32, None => 0x20 } << 8;
        value |= match cs.next() { Some(c) => c as u32, None => 0x20 };
        Tag::new(value)
    }

    /// Converts tag to a string.
    pub fn as_string(&self) -> String {
        let mut s = String::with_capacity(4);
        s.push(char::from_u32(self.tag & 0xff000000 >> 24).unwrap());
        s.push(char::from_u32(self.tag & 0x00ff0000 >> 16).unwrap());
        s.push(char::from_u32(self.tag & 0x0000ff00 >> 8).unwrap());
        s.push(char::from_u32(self.tag & 0x000000ff).unwrap());
        s
    }
}

impl From<sys::hb_tag_t> for Tag {
    fn from(s: sys::hb_tag_t) -> Self {
        Tag::new(s)
    }
}

impl From<Tag> for sys::hb_tag_t {
    fn from(s: Tag) -> Self {
        s.tag
    }
}

