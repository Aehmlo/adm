use futures::Stream;

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
