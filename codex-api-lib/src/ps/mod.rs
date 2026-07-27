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
    pub fn mcp<'a, C: Ps>(client: &C) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.ps_mcp()
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
    impl<C: super::r#async::Ps + WasmNotSync> Ps for C {
        async fn ps_mcp(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::Ps::ps_mcp(self).await
        }
    }

    pub async fn mcp<R: AsyncTryInto<String>, E>(
        client: &dyn Ps<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.ps_mcp().await
    }
}
