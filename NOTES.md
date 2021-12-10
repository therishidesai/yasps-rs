## Initial Design

- shm for pub/sub queues
- create queue and wakeup futex per topic when publisher registers topic
  - do all of these with helper functions
	- optional: request broker to see if topic exists
	- publisher will have to shm_open to create the shared memory fd
    - publisher will ftruncate the fd to set the size of the shared memory region
	- publisher will mmap the shm fd to begin accessing the data in its address space
	- publisher sets up the data structure of the wakeup futex and the queue 
- subscriber callbacks
  - do all of these with helper functions
	- optional: request broker to see if topic exists
	- shm open the file (topic name = file name) and get the fd
	- mmap the shm fd so it can start using it 
  - tokio spawn the actual callback and then await on the futex to be woken up when there is data
- broker is just a key value store of topics 
  - not required to run since the shared memory segments are just topic names
  - could use some other mechanism to manage topic names
