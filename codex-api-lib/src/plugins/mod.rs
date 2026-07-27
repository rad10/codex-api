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
    pub fn featured<C: Plugins>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.plugins_featured()
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
    pub fn featured<'a, C: Plugins>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.plugins_featured()
    }
}

#[cfg(feature = "boxed")]
pub mod boxed {
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
    impl<C: super::r#async::Plugins + WasmNotSync> Plugins for C {
        async fn plugins_featured(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::Plugins::plugins_featured(self).await
        }
    }

    pub async fn events<R: AsyncTryInto<String>, E>(
        client: &dyn Plugins<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.plugins_featured().await
    }
}
