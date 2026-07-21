#[cfg(feature = "async")]
use codex_api_lib::ps::plugins::PluginsAsync;
#[cfg(feature = "sync")]
use codex_api_lib::ps::plugins::PluginsSync;
use reqwest::IntoUrl;

#[cfg(feature = "async")]
use crate::client::CodexClient;
#[cfg(all(feature = "async", feature = "middleware"))]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::traits::{CodexAuthorization, CodexAccountId};

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PluginsAsync for CodexClient<Auth, Acc, U> {
    fn ps_plugins_installed(
        &self,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }

    fn ps_plugins_list(
        &self,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }

    fn ps_plugins_suggested(
        &self,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PluginsAsync for CodexMiddleware<Auth, Acc, U> {
    fn ps_plugins_installed(
        &self,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }

    fn ps_plugins_list(
        &self,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }

    fn ps_plugins_suggested(
        &self,
    ) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> PluginsSync for blocking::CodexClient<Auth, Acc, U> {
    fn ps_plugins_installed(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }

    fn ps_plugins_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }

    fn ps_plugins_suggested(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}
