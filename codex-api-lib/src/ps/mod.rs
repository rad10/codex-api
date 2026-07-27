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
    pub fn mcp<C: Ps>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.ps_mcp()
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
    pub fn mcp<C: Ps>(client: &C) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.ps_mcp()
    }

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use crate::plugins::r#async::thread_safe::Plugins;

        use super::{ApiCommon, AsyncTryInto};

        pub trait Ps: ApiCommon + Plugins {
            fn ps_mcp(&self) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn mcp<C: Ps>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.ps_mcp()
        }
    }

    #[cfg(feature = "threaded")]
    pub mod wasm_safe {
        use crate::{FutureNotSend, plugins::r#async::wasm_safe::Plugins};

        use super::{ApiCommon, AsyncTryInto};

        pub trait Ps: ApiCommon + Plugins {
            fn ps_mcp(&self) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn mcp<C: Ps>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.ps_mcp()
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

    pub async fn mcp<R: AsyncTryInto<String>, E>(
        client: &dyn Ps<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.ps_mcp().await
    }
}
