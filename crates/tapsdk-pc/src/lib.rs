//! High-level Rust bindings to TapTap PC SDK
//!
//! This crate provides a safe, idiomatic Rust API for the TapTap PC SDK.
//!
//! # Quick Start
//!
//! ```no_run
//! use tapsdk_pc::{TapSdk, user, ownership, dlc};
//! use tapsdk_pc::callback::TapEvent;
//!
//! fn main() -> tapsdk_pc::error::Result<()> {
//!     // Check if restart is needed (call before init)
//!     if tapsdk_pc::restart_app_if_necessary("your_client_id")? {
//!         // TapTap will relaunch the game, exit now
//!         return Ok(());
//!     }
//!
//!     // Initialize the SDK
//!     let sdk = TapSdk::init("your_public_key")?;
//!
//!     // Check game ownership
//!     if !ownership::is_game_owned() {
//!         println!("User does not own this game!");
//!         return Ok(());
//!     }
//!
//!     // Request user authorization
//!     user::authorize("public_profile")?;
//!
//!     // Game loop
//!     loop {
//!         // Poll for SDK events
//!         for event in sdk.run_callbacks() {
//!             match event {
//!                 TapEvent::AuthorizeFinished(data) => {
//!                     if let Some(token) = data.token {
//!                         println!("User authorized! OpenID: {:?}", user::get_open_id());
//!                     }
//!                 }
//!                 TapEvent::SystemStateChanged(data) => {
//!                     println!("System state: {:?}", data.state);
//!                 }
//!                 _ => {}
//!             }
//!         }
//!         
//!         // ... your game logic ...
//!         # break;
//!     }
//!
//!     // SDK is automatically shut down when `sdk` is dropped
//!     Ok(())
//! }
//! ```

pub mod callback;
pub mod cloudsave;
pub mod dlc;
pub mod error;
pub mod ownership;
pub mod sdk;
pub mod user;

// Re-export commonly used types at the crate root
pub use callback::TapEvent;
pub use cloudsave::CloudSave;
pub use error::{Result, TapSdkError};
pub use sdk::{is_initialized, restart_app_if_necessary, TapSdk};

// Re-export the sys crate for advanced users
pub use tapsdk_pc_sys as sys;
