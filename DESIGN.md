This document is a best-effort attempt to document the reasoning behind `adm`'s design decisions.
When possible, an attempt will be made to document alternatives, tradeoffs, and miscellaneous thoughts.

`adm`'s design philosophy is intended to enable flexibility and modularity through abstraction.
Sensors are simply signal sources; effectors, signal sinks.
The entire `adm` "framework" may be thought of as simply a transform stream that converts an input stream (notifications) to an output stream (messages), then sends that output stream to target devices as appropriate.
In the future, it is likely that this will be modularized even further, allowing for third-party crates to sit "in the middle" and be involved in this process.