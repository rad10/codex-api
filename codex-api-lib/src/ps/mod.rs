pub mod plugins;

// Table of endpoint constants
pub const MODULE_PS: &str = "ps";
pub const ENDPOINT_MCP: &str = "mcp";

#[cfg(feature = "sync")]
pub mod sync {
    use crate::{ApiCommon, ps::plugins::sync::Plugins};

    pub trait Ps: ApiCommon + Plugins {
        fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;
    }

    #[inline]
    pub fn mcp_response<C: Ps>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.ps_mcp()
    }

    pub fn mcp<C: Ps, E>(client: &C) -> Result<String, E>
    where
        C::Response: TryInto<String>,
        E: From<C::ApiError> + From<<C::Response as TryInto<String>>::Error>,
    {
        client.ps_mcp()?.try_into().map_err(E::from)
    }
}

#[cfg(feature = "async")]
pub mod r#async {
    use crate::{ApiCommon, AsyncTryInto, ps::plugins::r#async::Plugins};

    #[allow(async_fn_in_trait)]
    pub trait Ps: ApiCommon + Plugins {
        async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[inline]
    pub fn mcp_response<C: Ps>(client: &C) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.ps_mcp()
    }

    pub async fn mcp<C: Ps, E>(client: &C) -> Result<String, E>
    where
        C::Response: AsyncTryInto<String>,
        E: From<C::ApiError> + From<<C::Response as AsyncTryInto<String>>::Error>,
    {
        client
            .ps_mcp()
            .await?
            .async_try_into()
            .await
            .map_err(E::from)
    }

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use async_from::thread_safe::AsyncTryIntoThreadSafe;

        use crate::plugins::r#async::thread_safe::Plugins;

        use super::{ApiCommon, AsyncTryInto};

        pub trait Ps: ApiCommon + Plugins {
            fn ps_mcp(&self) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn mcp_response<C: Ps>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.ps_mcp()
        }

        pub async fn mcp<C: Ps, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoThreadSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoThreadSafe<String>>::Error>,
        {
            client
                .ps_mcp()
                .await?
                .async_try_into_threaded()
                .await
                .map_err(E::from)
        }
    }

    #[cfg(feature = "threaded")]
    pub mod wasm_safe {
        use async_from::wasm_safe::AsyncTryIntoWasmSafe;

        use crate::{FutureNotSend, plugins::r#async::wasm_safe::Plugins};

        use super::{ApiCommon, AsyncTryInto};

        pub trait Ps: ApiCommon + Plugins {
            fn ps_mcp(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn mcp_response<C: Ps>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.ps_mcp()
        }

        pub async fn mcp<C: Ps, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoWasmSafe<String>>::Error>,
        {
            client
                .ps_mcp()
                .await?
                .async_try_into_wasm()
                .await
                .map_err(E::from)
        }

        // Blanket implementation based on arch
        #[cfg(not(target_arch = "wasm32"))]
        impl<T: super::thread_safe::Ps + Plugins> Ps for T {
            fn ps_mcp(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::Ps::ps_mcp(self)
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl<T: super::Ps> Ps for T {
            fn ps_mcp(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::Ps::ps_mcp(self)
            }
        }
    }
}

#[cfg(feature = "boxed")]
pub mod boxed {
    use async_from::wasm_safe::AsyncTryIntoWasmSafe;
    use async_trait::async_trait;
    use wasm_not_send_sync::WasmNotSync;

    use crate::{ApiCommon, AsyncTryInto, ps::plugins::boxed::Plugins};

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    pub trait Ps: ApiCommon + Plugins {
        async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl<C: super::r#async::wasm_safe::Ps + WasmNotSync + Plugins> Ps for C {
        async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::Ps::ps_mcp(self).await
        }
    }

    pub async fn mcp_response<R: AsyncTryInto<String>, E>(
        client: &dyn Ps<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.ps_mcp().await
    }

    pub async fn mcp<
        R: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
        Re,
        E: From<Re> + From<<R as AsyncTryIntoWasmSafe<String>>::Error>,
    >(
        client: &dyn Ps<Response = R, ApiError = Re>,
    ) -> Result<String, E> {
        client
            .ps_mcp()
            .await?
            .async_try_into_wasm()
            .await
            .map_err(E::from)
    }
}
