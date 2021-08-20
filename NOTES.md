initial design:


publisher:
	- shm_open an fd
	- mmap on the shm fd
	- create eventfd for updates
	- 
