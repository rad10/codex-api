// Table of endpoint constants
pub const MODULE_PLUGINS: &str = "plugins";
pub const ENDPOINT_FEATURED: &str = "featured";

#[cfg(feature = "sync")]
pub mod sync {
    use crate::ApiCommon;

    pub trait Plugins: ApiCommon {
        /// Gets the settings for the given user's account
        fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;
    }

    #[inline]
    pub fn featured_response<C: Plugins>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.plugins_featured()
    }

    pub fn featured<C: Plugins, E>(client: &C) -> Result<String, E>
    where
        C::Response: TryInto<String>,
        E: From<C::ApiError> + From<<C::Response as TryInto<String>>::Error>,
    {
        client.plugins_featured()?.try_into().map_err(E::from)
    }
}

#[cfg(feature = "async")]
pub mod r#async {
    use crate::{ApiCommon, AsyncTryInto};

    #[allow(async_fn_in_trait)]
    pub trait Plugins: ApiCommon {
        /// Gets the settings for the given user's account
        async fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[inline]
    pub fn featured_response<C: Plugins>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.plugins_featured()
    }

    pub async fn featured<C: Plugins, E>(client: &C) -> Result<String, E>
    where
        C::Response: AsyncTryInto<String>,
        E: From<C::ApiError> + From<<C::Response as AsyncTryInto<String>>::Error>,
    {
        client
            .plugins_featured()
            .await?
            .async_try_into()
            .await
            .map_err(E::from)
    }

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use async_from::thread_safe::AsyncTryIntoThreadSafe;

        use super::{ApiCommon, AsyncTryInto};

        pub trait Plugins: ApiCommon {
            /// Gets the settings for the given user's account
            fn plugins_featured(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn featured_response<C: Plugins>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.plugins_featured()
        }

        pub async fn featured<C: Plugins, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoThreadSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoThreadSafe<String>>::Error>,
        {
            client
                .plugins_featured()
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

        pub trait Plugins: ApiCommon {
            /// Gets the settings for the given user's account
            fn plugins_featured(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn featured_response<C: Plugins>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.plugins_featured()
        }

        pub async fn featured<C: Plugins, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoWasmSafe<String>>::Error>,
        {
            client
                .plugins_featured()
                .await?
                .async_try_into_wasm()
                .await
                .map_err(E::from)
        }

        // Blanket implementation based on arch
        #[cfg(not(target_arch = "wasm32"))]
        impl<T: super::thread_safe::Plugins> Plugins for T {
            fn plugins_featured(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::Plugins::plugins_featured(self)
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl<T: super::Plugins> Plugins for T {
            fn plugins_featured(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::Plugins::plugins_featured(self)
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
    pub trait Plugins: ApiCommon {
        /// Gets the settings for the given user's account
        async fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl<C: super::r#async::wasm_safe::Plugins + WasmNotSync> Plugins for C {
        async fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::Plugins::plugins_featured(self).await
        }
    }

    pub async fn events_response<R: AsyncTryInto<String>, E>(
        client: &dyn Plugins<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.plugins_featured().await
    }

    pub async fn events<
        R: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
        Re,
        E: From<Re> + From<<R as AsyncTryIntoWasmSafe<String>>::Error>,
    >(
        client: &dyn Plugins<Response = R, ApiError = Re>,
    ) -> Result<String, E> {
        client
            .plugins_featured()
            .await?
            .async_try_into_wasm()
            .await
            .map_err(E::from)
    }
}
