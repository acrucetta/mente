
## Notes on Network Programming in C


### Chap 02: Questions

What is a socket?
- A socket is a connection between two devices through the internet
- An abstraction that represents on endpoint of a comms link

What is a connectionless protocol? What is a connection-oriented protocol?
- Connectionless -> sends data independently 
- Connection-oriented -> sends data in the context of a larger stream 

Is UDP a connectionless or connection-oriented protocol?
- Connectionless

Is TCP a connectionless or connection-oriented protocol?
- Connection-oriented

What types of applications generally benefit from using the UDP protocol?
- Videogames, video calls (better real-time performance)

What types of applications generally benefit from using the TCP protocol?
- Banking applications, databases, emails

Does TCP guarantee that data will be transmitted successfully?
- Yes

What does the bind() function do?
- It attaches a socket to a given IP address. Its usage is required by the server

What does the accept() function do?
- Accepts the connection from the client. It will block until a new TCP has been connected, then returns the socket for this new connection

In a TCP connection, does the client or the server send application data first?”
- The client asks for data, the server sends it 
