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
    pub fn rate_limit_reset_credits<C: Wham>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.wham_rate_limit_reset_credits()
    }

    #[inline]
    pub fn usage<C: Wham>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.wham_usage()
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
    pub fn rate_limit_reset_credits<C: Wham>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.wham_rate_limit_reset_credits()
    }

    #[inline]
    pub fn usage<C: Wham>(client: &C) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.wham_usage()
    }
}

#[cfg(feature = "boxed")]
pub mod boxed {
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
    impl<C: super::r#async::Wham + WasmNotSync> Wham for C {
        async fn wham_rate_limit_reset_credits(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::Wham::wham_rate_limit_reset_credits(self).await
        }

        async fn wham_usage(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::Wham::wham_usage(self).await
        }
    }

    pub async fn rate_limit_reset_credits<R: AsyncTryInto<String>, E>(
        client: &dyn Wham<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.wham_rate_limit_reset_credits().await
    }

    pub async fn usage<R: AsyncTryInto<String>, E>(
        client: &dyn Wham<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.wham_usage().await
    }
}
