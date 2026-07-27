pub const MODULE_ANALYTICS_EVENTS: &str = "analytics-events";
pub const ENDPOINT_EVENTS: &str = "events";

#[cfg(feature = "sync")]
pub mod sync {
    use crate::ApiCommon;

    pub trait AnalyticsEvents: ApiCommon {
        fn codex_analytics_events_events(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;
    }

    #[inline]
    pub fn events<C: AnalyticsEvents>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.codex_analytics_events_events()
    }
}

#[cfg(feature = "async")]
pub mod r#async {
    use crate::{ApiCommon, AsyncTryInto};

    #[allow(async_fn_in_trait)]
    pub trait AnalyticsEvents: ApiCommon {
        async fn codex_analytics_events_events(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[inline]
    pub fn events<C: AnalyticsEvents>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.codex_analytics_events_events()
    }

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use super::{ApiCommon, AsyncTryInto};

        pub trait AnalyticsEvents: ApiCommon {
            fn codex_analytics_events_events(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn events<C: AnalyticsEvents>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.codex_analytics_events_events()
        }
    }

    #[cfg(feature = "threaded")]
    pub mod wasm_safe {
        use crate::FutureNotSend;

        use super::{ApiCommon, AsyncTryInto};

        pub trait AnalyticsEvents: ApiCommon {
            fn codex_analytics_events_events(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn events<C: AnalyticsEvents>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.codex_analytics_events_events()
        }

        // Blanket implementation based on arch
        #[cfg(not(target_arch = "wasm32"))]
        impl<T: super::thread_safe::AnalyticsEvents> AnalyticsEvents for T {
            fn codex_analytics_events_events(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::AnalyticsEvents::codex_analytics_events_events(self)
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl<T: super::AnalyticsEvents> AnalyticsEvents for T {
            fn codex_analytics_events_events(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::AnalyticsEvents::codex_analytics_events_events(self)
            }
        }
    }
}

#[cfg(feature = "boxed")]
pub mod boxed {
    use async_trait::async_trait;
    use wasm_not_send_sync::WasmNotSync;

    use crate::{ApiCommon, AsyncTryInto};

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    pub trait AnalyticsEvents: ApiCommon {
        async fn codex_analytics_events_events(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl<C: super::r#async::wasm_safe::AnalyticsEvents + WasmNotSync> AnalyticsEvents for C {
        async fn codex_analytics_events_events(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::AnalyticsEvents::codex_analytics_events_events(self).await
        }
    }

    pub async fn events<R: AsyncTryInto<String>, E>(
        client: &dyn AnalyticsEvents<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.codex_analytics_events_events().await
    }
}
