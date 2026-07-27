use std::sync::{Arc, OnceLock};

use codex_api_types::codex::SessionSource;
use http::HeaderMap;

pub mod analytics_events;

// Table of endpoint constants
pub const MODULE_CODEX: &str = "codex";
pub const ENDPOINT_MODELS: &str = "models";
pub const ENDPOINT_RESPONSES: &str = "responses";

#[cfg(feature = "sync")]
pub mod sync {
    use codex_api_types::codex::{ModelsResponse, ResponseEvent, ResponsesApiRequest};

    use crate::{
        ApiCommon,
        codex::{ResponsesOptions, analytics_events::sync::AnalyticsEvents},
    };

    pub trait Codex: ApiCommon + AnalyticsEvents {
        /// Collects models from Codex's library
        fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<ModelsResponse>;

        /// Collects a response from ChatGPT's API
        fn codex_responses(
            &self,
            request: ResponsesApiRequest,
            options: ResponsesOptions,
        ) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<Vec<ResponseEvent>>;
    }

    #[inline]
    pub fn models<C: Codex>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<ModelsResponse>,
    {
        client.codex_models()
    }

    #[inline]
    pub fn responses<C: Codex>(
        client: &C,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<Vec<ResponseEvent>>,
    {
        client.codex_responses(request, options)
    }
}

#[cfg(feature = "async")]
pub mod r#async {
    use codex_api_types::codex::{ModelsResponse, ResponseEvent, ResponsesApiRequest};

    use crate::{
        ApiCommon, AsyncTryInto,
        codex::{ResponsesOptions, analytics_events::r#async::AnalyticsEvents},
    };

    #[allow(async_fn_in_trait)]
    pub trait Codex: ApiCommon + AnalyticsEvents {
        /// Collects models from Codex's library
        async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<ModelsResponse>;

        /// Collects a response from ChatGPT's API
        async fn codex_responses(
            &self,
            request: ResponsesApiRequest,
            options: ResponsesOptions,
        ) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<Vec<ResponseEvent>>;
    }

    #[inline]
    pub fn models<C: Codex>(client: &C) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<ModelsResponse>,
    {
        client.codex_models()
    }

    #[inline]
    pub fn responses<C: Codex>(
        client: &C,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<Vec<ResponseEvent>>,
    {
        client.codex_responses(request, options)
    }

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use crate::codex::analytics_events::r#async::thread_safe::AnalyticsEvents;

        use super::{
            ApiCommon, AsyncTryInto, ModelsResponse, ResponseEvent, ResponsesApiRequest,
            ResponsesOptions,
        };

        pub trait Codex: ApiCommon + AnalyticsEvents {
            /// Collects models from Codex's library
            fn codex_models(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<ModelsResponse>;

            /// Collects a response from ChatGPT's API
            fn codex_responses(
                &self,
                request: ResponsesApiRequest,
                options: ResponsesOptions,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<Vec<ResponseEvent>>;
        }

        #[inline]
        pub fn models<C: Codex>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<ModelsResponse>,
        {
            client.codex_models()
        }

        #[inline]
        pub fn responses<C: Codex>(
            client: &C,
            request: ResponsesApiRequest,
            options: ResponsesOptions,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<Vec<ResponseEvent>>,
        {
            client.codex_responses(request, options)
        }
    }

    #[cfg(feature = "threaded")]
    pub mod wasm_safe {
        use crate::{FutureNotSend, codex::analytics_events::r#async::wasm_safe::AnalyticsEvents};

        use super::{
            ApiCommon, AsyncTryInto, ModelsResponse, ResponseEvent, ResponsesApiRequest,
            ResponsesOptions,
        };

        pub trait Codex: ApiCommon + AnalyticsEvents {
            /// Collects models from Codex's library
            fn codex_models(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<ModelsResponse>;

            /// Collects a response from ChatGPT's API
            fn codex_responses(
                &self,
                request: ResponsesApiRequest,
                options: ResponsesOptions,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<Vec<ResponseEvent>>;
        }

        #[inline]
        pub fn models<C: Codex>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<ModelsResponse>,
        {
            client.codex_models()
        }

        #[inline]
        pub fn responses<C: Codex>(
            client: &C,
            request: ResponsesApiRequest,
            options: ResponsesOptions,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<Vec<ResponseEvent>>,
        {
            client.codex_responses(request, options)
        }

        // Blanket implementation based on arch
        #[cfg(not(target_arch = "wasm32"))]
        impl<T: super::thread_safe::Codex> Codex for T {
            fn codex_models(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<ModelsResponse>,
            {
                super::thread_safe::Codex::codex_models(self)
            }

            fn codex_responses(
                &self,
                request: ResponsesApiRequest,
                options: ResponsesOptions,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<Vec<ResponseEvent>>,
            {
                super::thread_safe::Codex::codex_responses(self, request, options)
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl<T: super::Codex> Codex for T {
            fn codex_models(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<ModelsResponse>,
            {
                super::Codex::codex_models(self)
            }

            fn codex_responses(
                &self,
                request: ResponsesApiRequest,
                options: ResponsesOptions,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<Vec<ResponseEvent>>,
            {
                super::Codex::codex_responses(self, request, options)
            }
        }
    }
}

#[cfg(feature = "boxed")]
pub mod boxed {
    use async_trait::async_trait;
    use codex_api_types::codex::{ModelsResponse, ResponseEvent, ResponsesApiRequest};
    use wasm_not_send_sync::WasmNotSync;

    use crate::{
        ApiCommon, AsyncTryInto,
        codex::{ResponsesOptions, analytics_events::boxed::AnalyticsEvents},
    };

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    pub trait Codex: ApiCommon + AnalyticsEvents {
        /// Collects models from Codex's library
        async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<ModelsResponse>;

        /// Collects a response from ChatGPT's API
        async fn codex_responses(
            &self,
            request: ResponsesApiRequest,
            options: ResponsesOptions,
        ) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<Vec<ResponseEvent>>;
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl<C: super::r#async::wasm_safe::Codex + WasmNotSync> Codex for C {
        async fn codex_models(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<ModelsResponse>,
        {
            super::r#async::wasm_safe::Codex::codex_models(self).await
        }

        async fn codex_responses(
            &self,
            request: ResponsesApiRequest,
            options: ResponsesOptions,
        ) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<Vec<ResponseEvent>>,
        {
            super::r#async::wasm_safe::Codex::codex_responses(self, request, options).await
        }
    }

    pub async fn models<R: AsyncTryInto<ModelsResponse>, E>(
        client: &dyn Codex<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.codex_models().await
    }

    pub async fn responses<R: AsyncTryInto<Vec<ResponseEvent>>, E>(
        client: &dyn Codex<Response = R, ApiError = E>,
        request: ResponsesApiRequest,
        options: ResponsesOptions,
    ) -> Result<R, E> {
        client.codex_responses(request, options).await
    }
}

#[derive(Default)]
pub struct ResponsesOptions {
    pub session_id: Option<String>,
    pub thread_id: Option<String>,
    pub session_source: Option<SessionSource>,
    pub extra_headers: HeaderMap,
    pub turn_state: Option<Arc<OnceLock<String>>>,
}
