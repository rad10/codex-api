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
    pub fn events_response<C: AnalyticsEvents>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.codex_analytics_events_events()
    }

    #[inline]
    pub fn events<C: AnalyticsEvents, E>(client: &C) -> Result<String, E>
    where
        C::Response: TryInto<String>,
        E: From<C::ApiError> + From<<C::Response as TryInto<String>>::Error>,
    {
        client
            .codex_analytics_events_events()?
            .try_into()
            .map_err(E::from)
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
    pub fn events_response<C: AnalyticsEvents>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.codex_analytics_events_events()
    }

    pub async fn events<C: AnalyticsEvents, E>(client: &C) -> Result<String, E>
    where
        C::Response: AsyncTryInto<String>,
        E: From<C::ApiError> + From<<C::Response as AsyncTryInto<String>>::Error>,
    {
        client
            .codex_analytics_events_events()
            .await?
            .async_try_into()
            .await
            .map_err(E::from)
    }

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use async_from::thread_safe::AsyncTryIntoThreadSafe;

        use super::{ApiCommon, AsyncTryInto};

        pub trait AnalyticsEvents: ApiCommon {
            fn codex_analytics_events_events(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn events_response<C: AnalyticsEvents>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.codex_analytics_events_events()
        }

        pub async fn events<C: AnalyticsEvents, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoThreadSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoThreadSafe<String>>::Error>,
        {
            client
                .codex_analytics_events_events()
                .await?
                .async_try_into_threaded()
                .await
                .map_err(E::from)
        }
    }

    #[cfg(feature = "threaded")]
    pub mod wasm_safe {
        use async_from::wasm_safe::AsyncTryIntoWasmSafe;

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
        pub fn events_response<C: AnalyticsEvents>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.codex_analytics_events_events()
        }

        pub async fn events<C: AnalyticsEvents, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoWasmSafe<String>>::Error>,
        {
            client
                .codex_analytics_events_events()
                .await?
                .async_try_into_wasm()
                .await
                .map_err(E::from)
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
    use async_from::wasm_safe::AsyncTryIntoWasmSafe;
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

    pub async fn events_response<R: AsyncTryInto<String>, E>(
        client: &dyn AnalyticsEvents<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.codex_analytics_events_events().await
    }

    pub async fn events<
        R: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
        Re,
        E: From<Re> + From<<R as AsyncTryIntoWasmSafe<String>>::Error>,
    >(
        client: &dyn AnalyticsEvents<Response = R, ApiError = Re>,
    ) -> Result<String, E> {
        client
            .codex_analytics_events_events()
            .await?
            .async_try_into_wasm()
            .await
            .map_err(E::from)
    }
}
