pub const MODULE_PLUGINS: &str = "plugins";
pub const ENDPOINT_INSTALLED: &str = "installed";
pub const ENDPOINT_LIST: &str = "list";
pub const ENDPOINT_SUGGESTED: &str = "suggested";

#[cfg(feature = "sync")]
pub mod sync {
    use crate::ApiCommon;

    pub trait Plugins: ApiCommon {
        fn ps_plugins_installed(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;

        fn ps_plugins_list(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;

        fn ps_plugins_suggested(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;
    }

    #[inline]
    pub fn installed<C: Plugins>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.ps_plugins_installed()
    }

    #[inline]
    pub fn list<C: Plugins>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.ps_plugins_list()
    }

    #[inline]
    pub fn suggested<C: Plugins>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.ps_plugins_suggested()
    }
}

#[cfg(feature = "async")]
pub mod r#async {
    use crate::{ApiCommon, AsyncTryInto};

    #[allow(async_fn_in_trait)]
    pub trait Plugins: ApiCommon {
        async fn ps_plugins_installed(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;

        async fn ps_plugins_list(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;

        async fn ps_plugins_suggested(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[inline]
    pub fn installed<C: Plugins>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.ps_plugins_installed()
    }

    #[inline]
    pub fn list<C: Plugins>(client: &C) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.ps_plugins_list()
    }

    #[inline]
    pub fn suggested<C: Plugins>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.ps_plugins_suggested()
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
        async fn ps_plugins_installed(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;

        async fn ps_plugins_list(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;

        async fn ps_plugins_suggested(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl<C: super::r#async::Plugins + WasmNotSync> Plugins for C {
        async fn ps_plugins_installed(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::Plugins::ps_plugins_installed(self).await
        }

        async fn ps_plugins_list(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::Plugins::ps_plugins_list(self).await
        }

        async fn ps_plugins_suggested(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::Plugins::ps_plugins_suggested(self).await
        }
    }

    pub async fn installed<R: AsyncTryInto<String>, E>(
        client: &dyn Plugins<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.ps_plugins_installed().await
    }

    pub async fn list<R: AsyncTryInto<String>, E>(
        client: &dyn Plugins<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.ps_plugins_list().await
    }

    pub async fn suggested<R: AsyncTryInto<String>, E>(
        client: &dyn Plugins<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.ps_plugins_suggested().await
    }
}
