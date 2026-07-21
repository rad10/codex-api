#[cfg(feature = "async")]
use codex_api_lib::connectors::directory::DirectoryAsync;
#[cfg(feature = "sync")]
use codex_api_lib::connectors::directory::DirectorySync;
use reqwest::IntoUrl;

#[cfg(feature = "async")]
use crate::client::CodexClient;
#[cfg(all(feature = "async", feature = "middleware"))]
use crate::client::CodexMiddleware;
#[cfg(feature = "sync")]
use crate::client::blocking;
use crate::client::traits::{CodexAuthorization, CodexAccountId};

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> DirectoryAsync for CodexClient<Auth, Acc, U> {
    fn connectors_directory_list(
        &self,
    ) -> impl codex_api_lib::FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String> {
        todo!()
    }

    fn connectors_directory_list_workspace(
        &self,
    ) -> impl codex_api_lib::FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> DirectoryAsync for CodexMiddleware<Auth, Acc, U> {
    fn connectors_directory_list(
        &self,
    ) -> impl codex_api_lib::FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String> {
        todo!()
    }

    fn connectors_directory_list_workspace(
        &self,
    ) -> impl codex_api_lib::FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}

#[cfg(feature = "sync")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> DirectorySync for blocking::CodexClient<Auth, Acc, U> {
    fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }

    fn connectors_directory_list_workspace(&self) -> Result<Self::Response, Self::ApiError>
    where
        Self::Response: TryInto<String> {
        todo!()
    }
}
