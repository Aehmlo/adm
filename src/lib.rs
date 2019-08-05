use futures::{Sink, Stream};

/// Encodes information which may be retrieved from an effector.
pub trait Notification {}

/// A device capable of retrieving information about its environment.
///
/// A sensor may be thought of as an asynchronous notification source.
pub trait Sensor<N>: Stream<Item = N>
where
    N: Notification,
{
}

/// A command which may be sent to an effector.
pub trait Message {}

/// A device which can act to change its environment.
///
/// A sensor may be thought of as an asynchronous message sink.
pub trait Effector<M, E>: Sink<SinkItem = M, SinkError = E>
where
    M: Message,
{
}
