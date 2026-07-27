// Table of endpoint constants
pub const MODULE_ACCOUNTS: &str = "accounts";
pub const ENDPOINT_SETTINGS: &str = "settings";

#[cfg(feature = "sync")]
pub mod sync {
    use uuid::Uuid;

    use crate::ApiCommon;

    pub trait Accounts: ApiCommon {
        /// Gets the settings for the given user's account
        fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;
    }

    #[inline]
    pub fn settings<C: Accounts>(client: &C, user_id: Uuid) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.account_settings(user_id)
    }
}

#[cfg(feature = "async")]
pub mod r#async {
    use uuid::Uuid;

    use crate::{ApiCommon, AsyncTryInto};

    #[allow(async_fn_in_trait)]
    pub trait Accounts: ApiCommon {
        /// Gets the settings for the given user's account
        async fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[inline]
    pub fn settings<C: Accounts>(
        client: &C,
        user_id: Uuid,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.account_settings(user_id)
    }

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use super::{ApiCommon, AsyncTryInto, Uuid};

        pub trait Accounts: ApiCommon {
            /// Gets the settings for the given user's account
            fn account_settings(
                &self,
                user_id: Uuid,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn settings<C: Accounts>(
            client: &C,
            user_id: Uuid,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.account_settings(user_id)
        }
    }

    #[cfg(feature = "threaded")]
    pub mod wasm_safe {
        use crate::FutureNotSend;

        use super::{ApiCommon, AsyncTryInto, Uuid};

        pub trait Accounts: ApiCommon {
            /// Gets the settings for the given user's account
            fn account_settings(
                &self,
                user_id: Uuid,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;
        }

        pub fn settings<C: Accounts>(
            client: &C,
            user_id: Uuid,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.account_settings(user_id)
        }

        // Blanket implementation based on arch
        #[cfg(not(target_arch = "wasm32"))]
        impl<T: super::thread_safe::Accounts> Accounts for T {
            fn account_settings(
                &self,
                user_id: Uuid,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::Accounts::account_settings(self, user_id)
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl<T: super::Accounts> Accounts for T {
            fn account_settings(
                &self,
                user_id: Uuid,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::Accounts::account_settings(self, user_id)
            }
        }
    }
}

#[cfg(feature = "boxed")]
pub mod boxed {
    use async_trait::async_trait;
    use uuid::Uuid;
    use wasm_not_send_sync::WasmNotSync;

    use crate::{ApiCommon, AsyncTryInto};

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    pub trait Accounts: ApiCommon {
        /// Gets the settings for the given user's account
        async fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl<C: super::r#async::wasm_safe::Accounts + WasmNotSync> Accounts for C {
        async fn account_settings(&self, user_id: Uuid) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::Accounts::account_settings(self, user_id).await
        }
    }

    pub async fn settings<R: AsyncTryInto<String>, E>(
        client: &dyn Accounts<Response = R, ApiError = E>,
        user_id: Uuid,
    ) -> Result<R, E> {
        client.account_settings(user_id).await
    }
}
