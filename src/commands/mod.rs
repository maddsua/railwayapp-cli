pub(super) use crate::{client::*, config::*, gql::*};
pub(super) use anyhow::{Context, Result};
pub(super) use clap::Parser;
pub(super) use colored::Colorize;

pub mod add;
pub mod completion;
pub mod connect;
pub mod delete;
pub mod docs;
pub mod domain;
pub mod environment;
pub mod init;
pub mod link;
pub mod list;
pub mod login;
pub mod logout;
pub mod logs;
pub mod open;
pub mod run;
pub mod service;
pub mod shell;
pub mod starship;
pub mod status;
pub mod unlink;
pub mod up;
pub mod variables;
pub mod whoami;
