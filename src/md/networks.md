
## Computer Networking: A Top Down Approach

### Chapter 1

The internet is a computer network that connects hundreds of millions of computing devices.

These devices are called hosts or end systems. They're connected by communication links and packet switches.

Different links can transmit data at different rates. Measured in bits/second. When one system sends data to another. It adds headers and bytes to each segment. This is known as packets.

End systems access the internet through Internet Service Providers. Each ISP network runs the IP protocol and conforms to naming and address conventions.

These systems run protocols; including the Transmission Control Protocol and the Internet Protocol.

The internet is also an infrastructure for providing services to distributed applications.

The internet is built of protocols, they defined the format and order of messages exchanged by two or more entities, as well as the actions taken on the transmission and/or receipt of a message or other event.

End systems can be divided into clients and servers. A client program is a running one end systems that requests and receives a service from a server program. They are distributed applications.

THe physical medium includes how the bits will travel from one system to the other. They can be guided or unguided. In guided ones, we have a solid medium (optic cable). In unguided the waves propagate in the atmosphere (e.g., wireless LAN or digital satellite)

Packet Switching: Information is divided into "packets" and sent via multiple connections to the destination. E.g., API calls.

Circuit switching: A physical connection is made between the host and the destination. E.g., phone lines.

Packet switching is more efficient. It uses statistical multiplexing, i.e., allows multiple data streams to share the same transmission channel based on the users' data demand. Instead of reserving capacity it allocates it to the users ready to send data.

### Application Layer

It's designed by the application developer and dictates how the application is structured over various end systems.

We can have: client-server or peer to peer architecture. 

Client-server has an always-on host called the server serving many requests from other hosts called clients. Clients don't talk with each other. The server has a fixed address. E.g., the web, FTP, e-mail.

In a P2P architecture, there is minimal or no reliance on always-on-infrastructure servers. The applications communicated with each other. Some examples are BitTorrent, LimeWire, Skype.

The process that initiates the communication is the client. The process that waits to be contacted is the server.

A process sends message into, and receives messages through a socket. A socket is the interface between the app layer and the transport layer of a host. Also referred to as the API between the application and the network.

What are the services provided by the transport layer protocol:
- Reliable data transfer
- Throughput: how many bits/second can it provide. Particularly useful for bandwidth-sensitive applications, i.e., media apps. Elastic applications can make use of as much or as little throughput as available. (e.g., email, file transfers)
- Timing: every bit the sender pumps arrives no more than 100 ms later.
- Security: a protocol can encrypt all data transmitted by the sending process. 

Transport services provided by the internet:

TCP Services
- TCP has the client and the server exchange info before the app level messages begin to flow. i.e, the handshake. When the app finishes it must tear down the connection.
- It sends all the data without error in the proper order.
- Includes congestion control mechanisms. It can throttle the transmission rate. (not as good for videos)

UDP Services
- No frills, lightweight transfer protocol, provides minimal services. Is connectionless, no handshakes.
