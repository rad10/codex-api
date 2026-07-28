pub mod profiles;

// Table of endpoint constants
pub const MODULE_WHAM: &str = "wham";
pub const ENDPOINT_RATE_LIMIT_RESET_CREDITS: &str = "rate-limit-reset-credits";
pub const ENDPOINT_USAGE: &str = "usage";

#[cfg(feature = "sync")]
pub mod sync {
    use crate::{ApiCommon, wham::profiles::sync::Profiles};

    pub trait Wham: ApiCommon + Profiles {
        fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;

        fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;
    }

    #[inline]
    pub fn rate_limit_reset_credits_response<C: Wham>(
        client: &C,
    ) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.wham_rate_limit_reset_credits()
    }

    #[inline]
    pub fn usage_response<C: Wham>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.wham_usage()
    }

    pub fn rate_limit_reset_credits<C: Wham, E>(client: &C) -> Result<String, E>
    where
        C::Response: TryInto<String>,
        E: From<C::ApiError> + From<<C::Response as TryInto<String>>::Error>,
    {
        client
            .wham_rate_limit_reset_credits()?
            .try_into()
            .map_err(E::from)
    }

    pub fn usage<C: Wham, E>(client: &C) -> Result<String, E>
    where
        C::Response: TryInto<String>,
        E: From<C::ApiError> + From<<C::Response as TryInto<String>>::Error>,
    {
        client.wham_usage()?.try_into().map_err(E::from)
    }
}

#[cfg(feature = "async")]
pub mod r#async {
    use crate::{ApiCommon, AsyncTryInto, wham::profiles::r#async::Profiles};

    #[allow(async_fn_in_trait)]
    pub trait Wham: ApiCommon + Profiles {
        async fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;

        async fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[inline]
    pub fn rate_limit_reset_credits_response<C: Wham>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.wham_rate_limit_reset_credits()
    }

    #[inline]
    pub fn usage_response<C: Wham>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.wham_usage()
    }

    pub async fn rate_limit_reset_credits<C: Wham, E>(client: &C) -> Result<String, E>
    where
        C::Response: AsyncTryInto<String>,
        E: From<C::ApiError> + From<<C::Response as AsyncTryInto<String>>::Error>,
    {
        client
            .wham_rate_limit_reset_credits()
            .await?
            .async_try_into()
            .await
            .map_err(E::from)
    }

    pub async fn usage<C: Wham, E>(client: &C) -> Result<String, E>
    where
        C::Response: AsyncTryInto<String>,
        E: From<C::ApiError> + From<<C::Response as AsyncTryInto<String>>::Error>,
    {
        client
            .wham_usage()
            .await?
            .async_try_into()
            .await
            .map_err(E::from)
    }

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use async_from::thread_safe::AsyncTryIntoThreadSafe;

        use crate::wham::profiles::r#async::thread_safe::Profiles;

        use super::{ApiCommon, AsyncTryInto};

        pub trait Wham: ApiCommon + Profiles {
            fn wham_rate_limit_reset_credits(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;

            fn wham_usage(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn rate_limit_reset_credits_response<C: Wham>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.wham_rate_limit_reset_credits()
        }

        #[inline]
        pub fn usage_response<C: Wham>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.wham_usage()
        }

        pub async fn rate_limit_reset_credits<C: Wham, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoThreadSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoThreadSafe<String>>::Error>,
        {
            client
                .wham_rate_limit_reset_credits()
                .await?
                .async_try_into_threaded()
                .await
                .map_err(E::from)
        }

        pub async fn usage<C: Wham, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoThreadSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoThreadSafe<String>>::Error>,
        {
            client
                .wham_usage()
                .await?
                .async_try_into_threaded()
                .await
                .map_err(E::from)
        }
    }

    #[cfg(feature = "threaded")]
    pub mod wasm_safe {
        use async_from::wasm_safe::AsyncTryIntoWasmSafe;

        use crate::{FutureNotSend, wham::profiles::r#async::wasm_safe::Profiles};

        use super::{ApiCommon, AsyncTryInto};

        pub trait Wham: ApiCommon + Profiles {
            fn wham_rate_limit_reset_credits(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;

            fn wham_usage(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn rate_limit_reset_credits_response<C: Wham>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.wham_rate_limit_reset_credits()
        }

        #[inline]
        pub fn usage_response<C: Wham>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.wham_usage()
        }

        pub async fn rate_limit_reset_credits<C: Wham, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoWasmSafe<String>>::Error>,
        {
            client
                .wham_rate_limit_reset_credits()
                .await?
                .async_try_into_wasm()
                .await
                .map_err(E::from)
        }

        pub async fn usage<C: Wham, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoWasmSafe<String>>::Error>,
        {
            client
                .wham_usage()
                .await?
                .async_try_into_wasm()
                .await
                .map_err(E::from)
        }

        // Blanket implementation based on arch
        #[cfg(not(target_arch = "wasm32"))]
        impl<T: super::thread_safe::Wham> Wham for T {
            fn wham_rate_limit_reset_credits(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::Wham::wham_rate_limit_reset_credits(self)
            }

            fn wham_usage(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::Wham::wham_usage(self)
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl<T: super::Wham> Wham for T {
            fn wham_rate_limit_reset_credits(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::Wham::wham_rate_limit_reset_credits(self)
            }

            fn wham_usage(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::Wham::wham_usage(self)
            }
        }
    }
}

#[cfg(feature = "boxed")]
pub mod boxed {
    use async_from::wasm_safe::AsyncTryIntoWasmSafe;
    use async_trait::async_trait;
    use wasm_not_send_sync::WasmNotSync;

    use crate::{ApiCommon, AsyncTryInto, wham::profiles::boxed::Profiles};

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    pub trait Wham: ApiCommon + Profiles {
        async fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;

        async fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl<C: super::r#async::wasm_safe::Wham + WasmNotSync> Wham for C {
        async fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::Wham::wham_rate_limit_reset_credits(self).await
        }

        async fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::Wham::wham_usage(self).await
        }
    }

    pub async fn rate_limit_reset_credits_response<R: AsyncTryInto<String>, E>(
        client: &dyn Wham<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.wham_rate_limit_reset_credits().await
    }

    pub async fn usage_response<R: AsyncTryInto<String>, E>(
        client: &dyn Wham<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.wham_usage().await
    }

    pub async fn rate_limit_reset_credits<
        R: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
        Re,
        E: From<Re> + From<<R as AsyncTryIntoWasmSafe<String>>::Error>,
    >(
        client: &dyn Wham<Response = R, ApiError = Re>,
    ) -> Result<String, E> {
        client
            .wham_rate_limit_reset_credits()
            .await?
            .async_try_into_wasm()
            .await
            .map_err(E::from)
    }

    pub async fn usage<
        R: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
        Re,
        E: From<Re> + From<<R as AsyncTryIntoWasmSafe<String>>::Error>,
    >(
        client: &dyn Wham<Response = R, ApiError = Re>,
    ) -> Result<String, E> {
        client
            .wham_usage()
            .await?
            .async_try_into_wasm()
            .await
            .map_err(E::from)
    }
}
