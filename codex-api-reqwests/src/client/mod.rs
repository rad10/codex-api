use codex_api_lib::{ApiCommon, STANDARD_ENDPOINT};
use http::HeaderMap;
use reqwest::Client;
use reqwest::IntoUrl;
#[cfg(feature = "middleware")]
use reqwest_middleware::ClientWithMiddleware;
use url::Url;

use crate::client::traits::NoAccountId;
#[cfg(feature = "middleware")]
use crate::error::MiddlewareError;
#[cfg(feature = "async")]
use crate::response::ApiResponse;
use crate::{
    client::traits::{CodexAccountId, CodexAuthorization},
    error::ApiError,
};

pub mod traits;

/// The Codex Client
#[derive(Debug, Clone)]
pub struct CodexClient<
    Auth: CodexAuthorization,
    Acc: CodexAccountId = NoAccountId,
    U: IntoUrl = Url,
> {
    /// The reqwest client
    pub(crate) client: Client,
    /// The endpoint that the API sends out to
    pub(crate) endpoint: U,
    /// The object containing authorization data
    pub(crate) authorization: Auth,
    /// The optional account id to attach to headers
    pub(crate) account_id: Option<Acc>,
    /// Extra headers to send with every request
    pub(crate) extra_headers: HeaderMap,
}

/// The Codex Client with a Middleware Wrapper
#[cfg(feature = "middleware")]
#[derive(Debug, Clone)]
pub struct CodexMiddleware<
    Auth: CodexAuthorization,
    Acc: CodexAccountId = NoAccountId,
    U: IntoUrl = Url,
> {
    /// The reqwest client
    pub(crate) client: ClientWithMiddleware,
    /// The endpoint that the API sends out to
    pub(crate) endpoint: U,
    /// The object containing authorization data
    pub(crate) authorization: Auth,
    /// The optional account id to attach to headers
    pub(crate) account_id: Option<Acc>,
    /// Extra headers to send with every request
    pub(crate) extra_headers: HeaderMap,
}

impl<Auth: CodexAuthorization, Acc: CodexAccountId> CodexClient<Auth, Acc, &'static str> {
    pub fn new(authorization: Auth) -> Result<Self, reqwest::Error> {
        Ok(Self::with_client(Client::builder().build()?, authorization))
    }

    pub fn with_client(client: Client, authorization: Auth) -> Self {
        Self {
            client,
            endpoint: STANDARD_ENDPOINT,
            authorization,
            account_id: None,
            extra_headers: HeaderMap::new(),
        }
    }
}

impl<Auth: CodexAuthorization, Acc: CodexAccountId, End: IntoUrl> CodexClient<Auth, Acc, End> {
    pub fn with_account<A: CodexAccountId>(self, account: A) -> CodexClient<Auth, A, End> {
        let Self {
            client,
            endpoint,
            authorization,
            extra_headers,
            ..
        } = self;
        CodexClient {
            client,
            endpoint,
            authorization,
            account_id: Some(account),
            extra_headers,
        }
    }

    pub fn with_endpoint<U: IntoUrl>(self, endpoint: U) -> CodexClient<Auth, Acc, U> {
        let Self {
            client,
            authorization,
            account_id,
            extra_headers,
            ..
        } = self;
        CodexClient {
            client,
            endpoint,
            authorization,
            account_id,
            extra_headers,
        }
    }

    pub fn with_endpoint_as_url(self) -> Result<CodexClient<Auth, Acc, Url>, reqwest::Error> {
        let Self {
            client,
            endpoint,
            authorization,
            account_id,
            extra_headers,
        } = self;
        Ok(CodexClient {
            client,
            endpoint: endpoint.into_url()?,
            authorization,
            account_id,
            extra_headers,
        })
    }

    pub fn with_headers(self, headers: HeaderMap) -> Self {
        Self {
            extra_headers: headers,
            ..self
        }
    }

    pub const fn headers(&self) -> &HeaderMap {
        &self.extra_headers
    }
}

#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId> CodexMiddleware<Auth, Acc, &'static str> {
    pub fn with_middleware(client: ClientWithMiddleware, authorization: Auth) -> Self {
        Self {
            client,
            endpoint: STANDARD_ENDPOINT,
            authorization,
            account_id: None,
            extra_headers: HeaderMap::new(),
        }
    }
}

#[cfg(feature = "middleware")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, End: IntoUrl> CodexMiddleware<Auth, Acc, End> {
    pub fn with_account<A: CodexAccountId>(self, account: A) -> CodexMiddleware<Auth, A, End> {
        let Self {
            client,
            endpoint,
            authorization,
            extra_headers,
            ..
        } = self;
        CodexMiddleware {
            client,
            endpoint,
            authorization,
            account_id: Some(account),
            extra_headers,
        }
    }

    pub fn with_endpoint<U: IntoUrl>(self, endpoint: U) -> CodexMiddleware<Auth, Acc, U> {
        let Self {
            client,
            authorization,
            account_id,
            extra_headers,
            ..
        } = self;
        CodexMiddleware {
            client,
            endpoint,
            authorization,
            account_id,
            extra_headers,
        }
    }

    pub fn with_endpoint_as_url(self) -> Result<CodexMiddleware<Auth, Acc, Url>, reqwest::Error> {
        let Self {
            client,
            endpoint,
            authorization,
            account_id,
            extra_headers,
        } = self;
        Ok(CodexMiddleware {
            client,
            endpoint: endpoint.into_url()?,
            authorization,
            account_id,
            extra_headers,
        })
    }

    pub fn with_headers(self, headers: HeaderMap) -> Self {
        Self {
            extra_headers: headers,
            ..self
        }
    }

    pub const fn headers(&self) -> &HeaderMap {
        &self.extra_headers
    }
}

#[cfg(feature = "async")]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> ApiCommon
    for CodexClient<Auth, Acc, U>
{
    type Response = ApiResponse;

    type ApiError = ApiError;
}

#[cfg(all(feature = "async", feature = "middleware"))]
impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> ApiCommon
    for CodexMiddleware<Auth, Acc, U>
{
    type Response = ApiResponse;

    type ApiError = MiddlewareError;
}

impl<A: CodexAuthorization + Default, C: CodexAccountId> Default
    for CodexClient<A, C, &'static str>
{
    fn default() -> Self {
        Self {
            client: Default::default(),
            endpoint: STANDARD_ENDPOINT,
            authorization: Default::default(),
            account_id: None,
            extra_headers: HeaderMap::new(),
        }
    }
}

#[cfg(feature = "middleware")]
impl<A: CodexAuthorization + Default, C: CodexAccountId> Default
    for CodexMiddleware<A, C, &'static str>
{
    fn default() -> Self {
        Self {
            client: Default::default(),
            endpoint: STANDARD_ENDPOINT,
            authorization: Default::default(),
            account_id: None,
            extra_headers: HeaderMap::new(),
        }
    }
}
/// Contains the client used for sync calls
#[cfg(feature = "sync")]
pub mod blocking {
    use reqwest::blocking::Client;

    use crate::response::BlockingApiResponse;

    use super::{
        ApiCommon, ApiError, CodexAccountId, CodexAuthorization, HeaderMap, IntoUrl, NoAccountId,
        STANDARD_ENDPOINT, Url,
    };

    /// The Codex Client
    #[derive(Debug, Clone)]
    pub struct CodexClient<
        Auth: CodexAuthorization,
        Acc: CodexAccountId = NoAccountId,
        U: IntoUrl = Url,
    > {
        /// The reqwest client
        pub(crate) client: Client,
        /// The endpoint that the API sends out to
        pub(crate) endpoint: U,
        /// The object containing authorization data
        pub(crate) authorization: Auth,
        /// The optional account id to attach to headers
        pub(crate) account_id: Option<Acc>,
        /// Extra headers to send with every request
        pub(crate) extra_headers: HeaderMap,
    }

    impl<Auth: CodexAuthorization, Acc: CodexAccountId> CodexClient<Auth, Acc, &'static str> {
        pub fn new(authorization: Auth) -> Result<Self, reqwest::Error> {
            Ok(Self::with_client(Client::builder().build()?, authorization))
        }

        pub fn with_client(client: Client, authorization: Auth) -> Self {
            Self {
                client,
                endpoint: STANDARD_ENDPOINT,
                authorization,
                account_id: None,
                extra_headers: HeaderMap::new(),
            }
        }
    }

    impl<Auth: CodexAuthorization, Acc: CodexAccountId, End: IntoUrl> CodexClient<Auth, Acc, End> {
        pub fn with_account<A: CodexAccountId>(self, account: A) -> CodexClient<Auth, A, End> {
            let Self {
                client,
                endpoint,
                authorization,
                extra_headers,
                ..
            } = self;
            CodexClient {
                client,
                endpoint,
                authorization,
                account_id: Some(account),
                extra_headers,
            }
        }

        pub fn with_endpoint<U: IntoUrl>(self, endpoint: U) -> CodexClient<Auth, Acc, U> {
            let Self {
                client,
                authorization,
                account_id,
                extra_headers,
                ..
            } = self;
            CodexClient {
                client,
                endpoint,
                authorization,
                account_id,
                extra_headers,
            }
        }

        pub fn with_endpoint_as_url(self) -> Result<CodexClient<Auth, Acc, Url>, reqwest::Error> {
            let Self {
                client,
                endpoint,
                authorization,
                account_id,
                extra_headers,
            } = self;
            Ok(CodexClient {
                client,
                endpoint: endpoint.into_url()?,
                authorization,
                account_id,
                extra_headers,
            })
        }

        pub fn with_headers(self, headers: HeaderMap) -> Self {
            Self {
                extra_headers: headers,
                ..self
            }
        }

        pub const fn headers(&self) -> &HeaderMap {
            &self.extra_headers
        }
    }

    impl<Auth: CodexAuthorization, Acc: CodexAccountId, U: IntoUrl> ApiCommon
        for CodexClient<Auth, Acc, U>
    {
        type Response = BlockingApiResponse;

        type ApiError = ApiError;
    }

    impl<A: CodexAuthorization + Default, C: CodexAccountId> Default
        for CodexClient<A, C, &'static str>
    {
        fn default() -> Self {
            Self {
                client: Default::default(),
                endpoint: STANDARD_ENDPOINT,
                authorization: Default::default(),
                account_id: None,
                extra_headers: HeaderMap::new(),
            }
        }
    }
}
