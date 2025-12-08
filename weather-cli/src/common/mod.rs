mod config;
mod error;
pub mod logging;
mod state;

pub use self::{
    error::{Error, Result},
    state::APP_STATE,
};
