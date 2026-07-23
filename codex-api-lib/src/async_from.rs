use std::convert::Infallible;

use wasm_not_send_sync::WasmNotSend;

use crate::FutureNotSend;

pub trait AsyncFrom<T>: Sized {
    fn from(value: T) -> impl FutureNotSend<Output = Self>;
}

pub trait AsyncTryFrom<T>: Sized {
    type Error;

    fn try_from(value: T) -> impl FutureNotSend<Output = Result<Self, Self::Error>>;
}

pub trait AsyncInto<T>: Sized {
    fn into(self) -> impl FutureNotSend<Output = T>;
}

pub trait AsyncTryInto<T>: Sized {
    type Error;

    fn try_into(self) -> impl FutureNotSend<Output = Result<T, Self::Error>>;
}

// Blanket implementations

impl<T, U: AsyncTryFrom<T>> AsyncTryInto<U> for T {
    type Error = U::Error;

    fn try_into(self) -> impl FutureNotSend<Output = Result<U, Self::Error>> {
        U::try_from(self)
    }
}

impl<T, U: AsyncInto<T> + WasmNotSend> AsyncTryFrom<U> for T {
    type Error = Infallible;

    async fn try_from(value: U) -> Result<Self, Self::Error> {
        Ok(value.into().await)
    }
}

impl<T, U: AsyncFrom<T> + WasmNotSend> AsyncInto<U> for T {
    fn into(self) -> impl FutureNotSend<Output = U> {
        U::from(self)
    }
}

// Converting std into async by blanket
impl<T: WasmNotSend, U: From<T>> AsyncFrom<T> for U {
    async fn from(value: T) -> Self {
        U::from(value)
    }
}
