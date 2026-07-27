pub mod directory;

// Table of endpoint constants
pub const MODULE_CONNECTORS: &str = "connectors";

#[cfg(feature = "sync")]
pub mod sync {
    use crate::{ApiCommon, connectors::directory::sync::Directory};

    pub trait Connectors: ApiCommon + Directory {}

    impl<T: ApiCommon + Directory> Connectors for T {}
}

#[cfg(feature = "async")]
pub mod r#async {
    use crate::{ApiCommon, connectors::directory::r#async::Directory};

    pub trait Connectors: ApiCommon + Directory {}

    impl<T: ApiCommon + Directory> Connectors for T {}

    pub mod thread_safe {
        use crate::connectors::directory::r#async::thread_safe::Directory;

        use super::ApiCommon;
        pub trait Connectors: ApiCommon + Directory {}

        impl<T: ApiCommon + Directory> Connectors for T {}
    }

    pub mod wasm_safe {
        use crate::connectors::directory::r#async::wasm_safe::Directory;

        use super::ApiCommon;
        pub trait Connectors: ApiCommon + Directory {}

        impl<T: ApiCommon + Directory> Connectors for T {}
    }
}

#[cfg(feature = "boxed")]
pub mod boxed {
    use crate::{ApiCommon, connectors::directory::boxed::Directory};

    pub trait Connectors: ApiCommon + Directory {}

    impl<T: ApiCommon + Directory> Connectors for T {}
}
