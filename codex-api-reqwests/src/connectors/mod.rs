use codex_api_lib::connectors::ConnectorsSub;

use crate::client::CodexClient;
#[cfg(feature = "middleware")]
use crate::client::CodexMiddleware;
use reqwest::IntoUrl;

#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::traits::{CodexAuthorization, CodexAccountId};


pub mod directory;

impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> ConnectorsSub for CodexClient<Auth, Acc, U> {}
#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> ConnectorsSub for CodexMiddleware<Auth, Acc, U> {}
#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> ConnectorsSub for blocking::CodexClient<Auth, Acc, U> {}
