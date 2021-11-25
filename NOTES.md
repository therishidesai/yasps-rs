## Initial Design

- shm for pub/sub queues
- new publishers creates new queues
- create eventfd per topic
- subscribers use mio to poll on eventfd's
	- use SourceFD/RawFD to use eventfd's with mio
	- trigger callback when message received
- central broker to hold topic info (eventfd's, shm, etc.)
	- Pros:
		- easy implementation
	- Cons:
		- single point of failure

## Questions/Future Ideas
- How to handle waiting on subscribers before overwriting parts of the buffer?
- turn this into an async executor in rust
