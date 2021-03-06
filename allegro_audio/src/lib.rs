// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_audio"]
#![crate_type = "lib"]

#![feature(libc)]
#![feature(thread_local)]
#![feature(optin_builtin_traits)]
#![feature(old_io)]
#![feature(core)]

extern crate allegro;
extern crate "allegro_audio-sys" as allegro_audio_sys;
extern crate libc;

pub use addon::*;
pub use stream::*;
pub use properties::*;
pub use sink::*;
pub use mixer::*;
pub use sample::*;

#[macro_use]
mod macros;

mod addon;
mod stream;
mod properties;
mod sink;
mod mixer;
mod sample;

mod internal;
