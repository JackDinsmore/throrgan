//! # Description
//! 
//! This binary is a small executable that tests the basic functionality of 
//! the `throrgan` crate. This binary is not intended to be published, and if
//! you are reading this, it was likely published accidentally.
use throrgan;

use std::{
    thread,
    time::Duration
};
use rg3d_sound::{
    source::{
        generic::GenericSourceBuilder,
        SoundSource,
        Status
    },
    context::Context,
    buffer::{
        DataSource,
        SoundBuffer
    },
};

fn main() {
    throrgan::compile("foo.txt", "bar.wav").unwrap();
    
    // This currently doesn't work.
    /*let context = Context::new();

    let sound_buffer = SoundBuffer::new_generic(
        DataSource::from_file("bar.wav").unwrap()).unwrap();

    let source = GenericSourceBuilder::new(sound_buffer)
    .with_status(Status::Playing)
    .build_source()
    .unwrap();

    context.state()
    .add_source(source);

    thread::sleep(Duration::from_secs(3));*/
}