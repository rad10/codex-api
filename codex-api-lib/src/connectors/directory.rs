pub const MODULE_DIRECTORY: &str = "directory";
pub const ENDPOINT_LIST: &str = "list";
pub const ENDPOINT_LIST_WORKSPACE: &str = "list_workspace";

#[cfg(feature = "sync")]
pub mod sync {
    use crate::ApiCommon;

    pub trait Directory: ApiCommon {
        fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;

        fn connectors_directory_list_workspace(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: TryInto<String>;
    }

    #[inline]
    pub fn list<C: Directory>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.connectors_directory_list()
    }

    #[inline]
    pub fn list_workspace<C: Directory>(client: &C) -> Result<C::Response, C::ApiError>
    where
        C::Response: TryInto<String>,
    {
        client.connectors_directory_list_workspace()
    }
}

#[cfg(feature = "async")]
pub mod r#async {
    use crate::{ApiCommon, AsyncTryInto};

    #[allow(async_fn_in_trait)]
    pub trait Directory: ApiCommon {
        async fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;

        async fn connectors_directory_list_workspace(
            &self,
        ) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[inline]
    pub fn list<C: Directory>(client: &C) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.connectors_directory_list()
    }

    #[inline]
    pub fn list_workspace<C: Directory>(
        client: &C,
    ) -> impl Future<Output = Result<C::Response, C::ApiError>>
    where
        C::Response: AsyncTryInto<String>,
    {
        client.connectors_directory_list_workspace()
    }

    #[cfg(feature = "threaded")]
    pub mod thread_safe {
        use super::{ApiCommon, AsyncTryInto};

        pub trait Directory: ApiCommon {
            fn connectors_directory_list(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;

            fn connectors_directory_list_workspace(
                &self,
            ) -> impl Future<Output = Result<Self::Response, Self::ApiError>> + Send
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn list<C: Directory>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.connectors_directory_list()
        }

        #[inline]
        pub fn list_workspace<C: Directory>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>> + Send
        where
            C::Response: AsyncTryInto<String>,
        {
            client.connectors_directory_list_workspace()
        }
    }

    #[cfg(feature = "threaded")]
    pub mod wasm_safe {
        use crate::FutureNotSend;

        use super::{ApiCommon, AsyncTryInto};

        pub trait Directory: ApiCommon {
            fn connectors_directory_list(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;

            fn connectors_directory_list_workspace(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>;
        }

        #[inline]
        pub fn list<C: Directory>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.connectors_directory_list()
        }

        #[inline]
        pub fn list_workspace<C: Directory>(
            client: &C,
        ) -> impl Future<Output = Result<C::Response, C::ApiError>>
        where
            C::Response: AsyncTryInto<String>,
        {
            client.connectors_directory_list_workspace()
        }

        // Blanket implementation based on arch
        #[cfg(not(target_arch = "wasm32"))]
        impl<T: super::thread_safe::Directory> Directory for T {
            fn connectors_directory_list(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::Directory::connectors_directory_list(self)
            }

            fn connectors_directory_list_workspace(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::thread_safe::Directory::connectors_directory_list_workspace(self)
            }
        }

        #[cfg(target_arch = "wasm32")]
        impl<T: super::Directory> Directory for T {
            fn connectors_directory_list(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::Directory::connectors_directory_list(self)
            }

            fn connectors_directory_list_workspace(
                &self,
            ) -> impl FutureNotSend<Output = Result<Self::Response, Self::ApiError>>
            where
                Self::Response: AsyncTryInto<String>,
            {
                super::Directory::connectors_directory_list_workspace(self)
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
    pub trait Directory: ApiCommon {
        async fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;

        async fn connectors_directory_list_workspace(
            &self,
        ) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>;
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl<C: super::r#async::wasm_safe::Directory + WasmNotSync> Directory for C {
        async fn connectors_directory_list(&self) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::Directory::connectors_directory_list(self).await
        }

        async fn connectors_directory_list_workspace(
            &self,
        ) -> Result<Self::Response, Self::ApiError>
        where
            Self::Response: AsyncTryInto<String>,
        {
            super::r#async::wasm_safe::Directory::connectors_directory_list_workspace(self).await
        }
    }

    pub async fn list<R: AsyncTryInto<String>, E>(
        client: &dyn Directory<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.connectors_directory_list().await
    }

    pub async fn list_workspace<R: AsyncTryInto<String>, E>(
        client: &dyn Directory<Response = R, ApiError = E>,
    ) -> Result<R, E> {
        client.connectors_directory_list_workspace().await
    }
}
