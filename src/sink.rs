use futures::Sink;

/// A command which may be sent to an effector.
pub trait Message {}

/// A device which can act to change its environment.
///
/// An effector may be thought of as an asynchronous message sink.
pub trait Effector<M>: Sink<M>
where
    M: Message,
{
    type Error: From<<Self as Sink<M>>::Error>;
}
