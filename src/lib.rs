#![warn(bare_trait_objects)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

extern crate regex;

//#[macro_use]
extern crate lazy_static;

extern crate yaml_rust;

extern crate strum;
#[macro_use]
extern crate strum_macros;

extern crate unicase;

pub mod rpg;

