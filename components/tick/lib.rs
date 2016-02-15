#![feature(mpsc_select)]

extern crate tick_traits;
#[macro_use]
extern crate common;

extern crate schedule_recv as timer;

pub mod tick_thread;
