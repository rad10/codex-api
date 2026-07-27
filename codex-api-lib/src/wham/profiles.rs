pub const MODULE_PROFILES: &str = "profiles";
pub const ENDPOINT_ME: &str = "me";

#[cfg(feature = "sync")]
pub mod sync {
    use crate::ApiCommon;

    pub trait Profiles: ApiCommon {
        fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;
    }

    #[inline]
    pub fn me<C: Profiles>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.wham_profiles_me()
    }
}

#[cfg(feature = "async")]
pub mod r#async {
    use crate::{ApiCommon, AsyncTryInto};

    #[allow(async_fn_in_trait)]
    pub trait Profiles: ApiCommon {
        async fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[inline]
    pub fn me<'a, C: Profiles>(client: &C) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.wham_profiles_me()
    }
}

#[cfg(feature = "boxed")]
pub mod boxed {
    use async_trait::async_trait;
    use wasm_not_send_sync::WasmNotSync;

    use crate::{ApiCommon, AsyncTryInto};

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    pub trait Profiles: ApiCommon {
        async fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl<C: super::r#async::Profiles + WasmNotSync> Profiles for C {
        async fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::Profiles::wham_profiles_me(self).await
        }
    }

    pub async fn me<R: AsyncTryInto<String>, E>(
        client: &dyn Profiles<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.wham_profiles_me().await
    }
}
