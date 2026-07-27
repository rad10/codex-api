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
    pub fn events<'a, C: AnalyticsEvents>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.codex_analytics_events_events()
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
    impl<C: super::r#async::AnalyticsEvents + WasmNotSync> AnalyticsEvents for C {
        async fn codex_analytics_events_events(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::AnalyticsEvents::codex_analytics_events_events(self).await
        }
    }

    pub async fn events<R: AsyncTryInto<String>, E>(
        client: &dyn AnalyticsEvents<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.codex_analytics_events_events().await
    }
}
