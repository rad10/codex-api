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
    pub fn me<C: Profiles>(client: &C) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.wham_profiles_me()
    }

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use super::{ApiCommon, AsyncTryInto};

        pub trait Profiles: ApiCommon {
            fn wham_profiles_me(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn me<C: Profiles>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.wham_profiles_me()
        }
    }

    #[cfg(feature = "threaded")]
    pub mod wasm_safe {
        use crate::FutureNotSend;

        use super::{ApiCommon, AsyncTryInto};

        pub trait Profiles: ApiCommon {
            fn wham_profiles_me(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn me<C: Profiles>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.wham_profiles_me()
        }

        // Blanket implementation based on arch
        #[cfg(not(target_arch = "wasm32"))]
        impl<T: super::thread_safe::Profiles> Profiles for T {
            fn wham_profiles_me(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::Profiles::wham_profiles_me(self)
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl<T: super::Profiles> Profiles for T {
            fn wham_profiles_me(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::Profiles::wham_profiles_me(self)
            }
        }
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
    impl<C: super::r#async::wasm_safe::Profiles + WasmNotSync> Profiles for C {
        async fn wham_profiles_me(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::Profiles::wham_profiles_me(self).await
        }
    }

    pub async fn me<R: AsyncTryInto<String>, E>(
        client: &dyn Profiles<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.wham_profiles_me().await
    }
}
