pub(crate) mod types;
pub use types::*;

use super::TimestampMs;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum Event<'a, R>
where
    R: PhyRxTx,
{
    TxRequest(TxConfig, &'a [u8]),
    RxRequest(RfConfig),
    CancelRx,
    PhyEvent(R::PhyEvent),
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum Response<R>
where
    R: PhyRxTx,
{
    Idle,
    Txing,
    Rxing,
    TxDone(TimestampMs),
    RxDone(RxQuality),
    PhyResponse(R::PhyResponse),
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum Error<R>
where
    R: PhyRxTx,
{
    TxRequestDuringTx,
    TxRequestDuringRx,
    RxRequestDuringTx,
    RxRequestDuringRx,
    CancelRxWhileIdle,
    CancelRxDuringTx,
    PhyError(R::PhyError),
}

use core::fmt;

pub trait PhyRxTx {
    #[cfg(not(feature = "defmt"))]
    type PhyEvent: fmt::Debug;
    #[cfg(feature = "defmt")]
    type PhyEvent: fmt::Debug + defmt::Format;

    #[cfg(not(feature = "defmt"))]
    type PhyError: fmt::Debug;
    #[cfg(feature = "defmt")]
    type PhyError: fmt::Debug + defmt::Format;

    #[cfg(not(feature = "defmt"))]
    type PhyResponse: fmt::Debug;
    #[cfg(feature = "defmt")]
    type PhyResponse: fmt::Debug + defmt::Format;

    fn get_mut_radio(&mut self) -> &mut Self;

    // we require mutability so we may decrypt in place
    fn get_received_packet(&mut self) -> &mut [u8];
    fn handle_event(&mut self, event: Event<Self>) -> Result<Response<Self>, Error<Self>>
    where
        Self: core::marker::Sized;
}
