# What to build

A HTTP RESTful API that accept local requests and execute TCPStream requests with another server.
- Much be able to Recover from Errors and Shutdown.
- A Queue to make sure request much be execute.
- Simple, lightweight and least runtime cost at possible.

# Where to build

- A Microservice in a Containerizing Microservices Network. 

# How to build

- A Asyncious Rust API that get commands from other servicers and add to Queue.
- A Rust runner to execute commands from Queue and execute.

# Note on build

- Queue much be Recoverable.
