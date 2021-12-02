## Initial Design

- shm for pub/sub queues
- new publishers creates new queues
- create eventfd per topic
- subscriber callbacks
	- request publisher info from broker
		- this is the eventfd and the read end of the queue data structure
	- similar to tokio::spawn write the callback and have it await on data from the pub queue
	- the await is actually on an event from the eventfd underlying the system
- central broker to hold topic info (eventfd's, shm, etc.)
	- Pros:
		- easy implementation
	- Cons:
		- single point of failure
- block publisher until last subscriber has read from buffer
	- put eventfd in EFD_NONBLOCKING and EFD_SEMAPHORE
	- publisher sets eventfd to number of subscribers
	- each read by a subscriber will decrement the eventfd by 1
	- publisher will block on next write until eventfd is 0
		- could just have the publisher still write to the queue and once eventfd is ready update the pointer for the top and signal the eventfd
## Questions/Future Ideas
- How to handle waiting on subscribers before overwriting parts of the buffer?
- turn this into an async executor in rust
