#![no_std]

// Library interface for emon32 Rust POC
// This allows testing of modules without the embedded runtime

pub mod board;
pub mod energy;

pub use energy::{EnergyCalculator, PowerData};