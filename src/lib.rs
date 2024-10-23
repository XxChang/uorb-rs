#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(all(test, feature = "std"))]
extern crate test;

mod platform;


