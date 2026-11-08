/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs LLC <hello@stalw.art>
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

use super::{after, is_field_end};
use memchr::memchr_iter;

const DASH_LINE: &[u8] = b"\n--";

#[cfg(feature = "std")]
pub(crate) fn dash_line(hay: &[u8], from: usize) -> Option<usize> {
    use memchr::memmem::Finder;
    use std::sync::LazyLock;

    static FINDER: LazyLock<Finder<'static>> = LazyLock::new(|| Finder::new(DASH_LINE));
    FINDER.find(hay.get(from..)?).map(|pos| pos + from)
}

#[cfg(not(feature = "std"))]
pub(crate) fn dash_line(hay: &[u8], from: usize) -> Option<usize> {
    memchr::memmem::find(hay.get(from..)?, DASH_LINE).map(|pos| pos + from)
}

pub(crate) fn field_end(hay: &[u8], from: usize) -> Option<usize> {
    memchr_iter(b'\n', hay.get(from..)?)
        .map(|pos| pos + from)
        .find(|&pos| is_field_end(after(hay, pos)))
}
