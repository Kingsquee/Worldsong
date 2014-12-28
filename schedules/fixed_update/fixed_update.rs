#![feature(phase)]
#[phase(link, plugin)]
extern crate common;

schedule!(
    graphics(sim, window)
)
