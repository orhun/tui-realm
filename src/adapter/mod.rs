//! ## adapters
//!
//! this module contains the event converter for the different backends

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use crate::core::event::{Event, Key, KeyEvent, KeyModifiers};

// -- crossterm
#[cfg(feature = "with-crossterm")]
pub mod crossterm;
#[cfg(feature = "with-crossterm")]
pub use self::crossterm::CrosstermInputListener as InputEventListener;
#[cfg(feature = "with-crossterm")]
pub use self::crossterm::{Frame, Terminal};

// -- termion
#[cfg(feature = "with-termion")]
pub mod termion;
#[cfg(feature = "with-termion")]
pub use self::termion::TermionInputListener as InputEventListener;
#[cfg(feature = "with-termion")]
pub use self::termion::{Frame, Terminal};
