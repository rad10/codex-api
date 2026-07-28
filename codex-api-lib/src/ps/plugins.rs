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
    pub fn installed_response<C: Plugins>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.ps_plugins_installed()
    }

    #[inline]
    pub fn list_response<C: Plugins>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.ps_plugins_list()
    }

    #[inline]
    pub fn suggested_response<C: Plugins>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.ps_plugins_suggested()
    }

    pub fn installed<C: Plugins, E>(client: &C) -> Result<String, E>
    where
        C::Response: TryInto<String>,
        E: From<C::ApiError> + From<<C::Response as TryInto<String>>::Error>,
    {
        client.ps_plugins_installed()?.try_into().map_err(E::from)
    }

    pub fn list<C: Plugins, E>(client: &C) -> Result<String, E>
    where
        C::Response: TryInto<String>,
        E: From<C::ApiError> + From<<C::Response as TryInto<String>>::Error>,
    {
        client.ps_plugins_list()?.try_into().map_err(E::from)
    }

    pub fn suggested<C: Plugins, E>(client: &C) -> Result<String, E>
    where
        C::Response: TryInto<String>,
        E: From<C::ApiError> + From<<C::Response as TryInto<String>>::Error>,
    {
        client.ps_plugins_suggested()?.try_into().map_err(E::from)
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
    pub fn installed_response<C: Plugins>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.ps_plugins_installed()
    }

    #[inline]
    pub fn list_response<C: Plugins>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.ps_plugins_list()
    }

    #[inline]
    pub fn suggested_response<C: Plugins>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.ps_plugins_suggested()
    }

    pub async fn installed<C: Plugins, E>(client: &C) -> Result<String, E>
    where
        C::Response: AsyncTryInto<String>,
        E: From<C::ApiError> + From<<C::Response as AsyncTryInto<String>>::Error>,
    {
        client
            .ps_plugins_installed()
            .await?
            .async_try_into()
            .await
            .map_err(E::from)
    }

    pub async fn list<C: Plugins, E>(client: &C) -> Result<String, E>
    where
        C::Response: AsyncTryInto<String>,
        E: From<C::ApiError> + From<<C::Response as AsyncTryInto<String>>::Error>,
    {
        client
            .ps_plugins_list()
            .await?
            .async_try_into()
            .await
            .map_err(E::from)
    }

    pub async fn suggested<C: Plugins, E>(client: &C) -> Result<String, E>
    where
        C::Response: AsyncTryInto<String>,
        E: From<C::ApiError> + From<<C::Response as AsyncTryInto<String>>::Error>,
    {
        client
            .ps_plugins_suggested()
            .await?
            .async_try_into()
            .await
            .map_err(E::from)
    }

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use async_from::thread_safe::AsyncTryIntoThreadSafe;

        use super::{ApiCommon, AsyncTryInto};

        pub trait Plugins: ApiCommon {
            fn ps_plugins_installed(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;

            fn ps_plugins_list(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;

            fn ps_plugins_suggested(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn installed_response<C: Plugins>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.ps_plugins_installed()
        }

        #[inline]
        pub fn list_response<C: Plugins>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.ps_plugins_list()
        }

        #[inline]
        pub fn suggested_response<C: Plugins>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.ps_plugins_suggested()
        }

        pub async fn installed<C: Plugins, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoThreadSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoThreadSafe<String>>::Error>,
        {
            client
                .ps_plugins_installed()
                .await?
                .async_try_into_threaded()
                .await
                .map_err(E::from)
        }

        pub async fn list<C: Plugins, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoThreadSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoThreadSafe<String>>::Error>,
        {
            client
                .ps_plugins_list()
                .await?
                .async_try_into_threaded()
                .await
                .map_err(E::from)
        }

        pub async fn suggested<C: Plugins, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoThreadSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoThreadSafe<String>>::Error>,
        {
            client
                .ps_plugins_suggested()
                .await?
                .async_try_into_threaded()
                .await
                .map_err(E::from)
        }
    }

    #[cfg(feature = "threaded")]
    pub mod wasm_safe {
        use async_from::wasm_safe::AsyncTryIntoWasmSafe;

        use crate::FutureNotSend;

        use super::{ApiCommon, AsyncTryInto};

        pub trait Plugins: ApiCommon {
            fn ps_plugins_installed(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;

            fn ps_plugins_list(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;

            fn ps_plugins_suggested(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn installed_response<C: Plugins>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.ps_plugins_installed()
        }

        #[inline]
        pub fn list_response<C: Plugins>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.ps_plugins_list()
        }

        #[inline]
        pub fn suggested_response<C: Plugins>(
            client: &C,
        ) -> impl FutureNotSend<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.ps_plugins_suggested()
        }

        pub async fn installed<C: Plugins, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoWasmSafe<String>>::Error>,
        {
            client
                .ps_plugins_installed()
                .await?
                .async_try_into_wasm()
                .await
                .map_err(E::from)
        }

        pub async fn list<C: Plugins, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoWasmSafe<String>>::Error>,
        {
            client
                .ps_plugins_list()
                .await?
                .async_try_into_wasm()
                .await
                .map_err(E::from)
        }

        pub async fn suggested<C: Plugins, E>(client: &C) -> Result<String, E>
        where
            C::Response: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
            E: From<C::ApiError> + From<<C::Response as AsyncTryIntoWasmSafe<String>>::Error>,
        {
            client
                .ps_plugins_suggested()
                .await?
                .async_try_into_wasm()
                .await
                .map_err(E::from)
        }

        // Blanket implementation based on arch
        #[cfg(not(target_arch = "wasm32"))]
        impl<T: super::thread_safe::Plugins> Plugins for T {
            fn ps_plugins_installed(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::Plugins::ps_plugins_installed(self)
            }

            fn ps_plugins_list(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::Plugins::ps_plugins_list(self)
            }

            fn ps_plugins_suggested(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::Plugins::ps_plugins_suggested(self)
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl<T: super::Plugins> Plugins for T {
            fn ps_plugins_installed(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::Plugins::ps_plugins_installed(self)
            }

            fn ps_plugins_list(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::Plugins::ps_plugins_list(self)
            }

            fn ps_plugins_suggested(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::Plugins::ps_plugins_suggested(self)
            }
        }
    }
}

#[cfg(feature = "boxed")]
pub mod boxed {
    use async_from::wasm_safe::AsyncTryIntoWasmSafe;
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
    impl<C: super::r#async::wasm_safe::Plugins + WasmNotSync> Plugins for C {
        async fn ps_plugins_installed(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::Plugins::ps_plugins_installed(self).await
        }

        async fn ps_plugins_list(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::Plugins::ps_plugins_list(self).await
        }

        async fn ps_plugins_suggested(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::Plugins::ps_plugins_suggested(self).await
        }
    }

    pub async fn installed_response<R: AsyncTryInto<String>, E>(
        client: &dyn Plugins<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.ps_plugins_installed().await
    }

    pub async fn list_response<R: AsyncTryInto<String>, E>(
        client: &dyn Plugins<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.ps_plugins_list().await
    }

    pub async fn suggested_response<R: AsyncTryInto<String>, E>(
        client: &dyn Plugins<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.ps_plugins_suggested().await
    }

    pub async fn installed<
        R: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
        Re,
        E: From<Re> + From<<R as AsyncTryIntoWasmSafe<String>>::Error>,
    >(
        client: &dyn Plugins<Response = R, ApiError = Re>,
    ) -> Result<String, E> {
        client
            .ps_plugins_installed()
            .await?
            .async_try_into_wasm()
            .await
            .map_err(E::from)
    }

    pub async fn list<
        R: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
        Re,
        E: From<Re> + From<<R as AsyncTryIntoWasmSafe<String>>::Error>,
    >(
        client: &dyn Plugins<Response = R, ApiError = Re>,
    ) -> Result<String, E> {
        client
            .ps_plugins_list()
            .await?
            .async_try_into_wasm()
            .await
            .map_err(E::from)
    }

    pub async fn suggested<
        R: AsyncTryInto<String> + AsyncTryIntoWasmSafe<String>,
        Re,
        E: From<Re> + From<<R as AsyncTryIntoWasmSafe<String>>::Error>,
    >(
        client: &dyn Plugins<Response = R, ApiError = Re>,
    ) -> Result<String, E> {
        client
            .ps_plugins_suggested()
            .await?
            .async_try_into_wasm()
            .await
            .map_err(E::from)
    }
}
