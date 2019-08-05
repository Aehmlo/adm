This document is a best-effort attempt to document the reasoning behind `adm`'s design decisions.
When possible, an attempt will be made to document alternatives, tradeoffs, and miscellaneous thoughts.

`adm`'s design philosophy is intended to enable flexibility and modularity through abstraction.
Sensors are simply signal sources; effectors, signal sinks.
The entire `adm` "framework" may be thought of as simply a transform stream that converts an input stream (notifications) to an output stream (messages), then sends that output stream to target devices as appropriate.
In the future, it is likely that this will be modularized even further, allowing for third-party crates to sit "in the middle" and be involved in this process.

## `Error` as an associated type

The initial version of the `Effector` trait was spelled `Effector<M, E>`, where `M` was the message type and `E` the error type.
However, this possessed not only ergonomic but *semantic* shortcomings.
By making the error type an orthogonal generic parameter, the early `Effector` trait allowed multiple implementations with the same message type *but different error types*.
This doesn't make a lot of sense; for a given message type, it's desirable to have a simple error type that can occur.
As such, this was quickly revised, and `Error` is now an associated type on `Effector`, coupling the message and error types together.
The migration to an associated type has the additional benefit of clearer compiler errors when the error type is forgotten.
To make sure that the error can actually be constructed from the underlying `Stream` implementation, the `Error` type must implement `From<<Self as Sink>::SinkError>`.
