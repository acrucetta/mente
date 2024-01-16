
## Building a Proxy Server in C

The general architecture for a proxy server is:

You have a user, that's you. You go into a website and click on facebook.com. Then facebook.com sends a TCP / UDP request to the facebook.com server to request that information and show it in the browser.

If you had a proxy server you would build a small program on your localhost that captures those requests, reads them, and forwards them to the server. It acts as the intermediary between the browser and the server requests.

You can either handle HTTP requests or HTTPS requests.

HTTP requests are easy, because you can read all the data sent from the client (e.g., websites you want to block) and decide whether you want to return a 403 error (forbidden) or if you want to send the request to the server and serve whatever content the client is asking for.

With HTTPS requests it becomes more complicated. 

HTTPS servers encrypt the data so that no person in the middle can know who's sending what. Many browsers now default to HTTPS only connections.

HTTPS secures HTTP by using TLS over TCP on port 443. TLS provides security to any TCP connection. TLS is the successor the Secure Socket Layer (SSL). Generally when a server and a client interact they have to negotiate which protocol to use (SSL or TLS).

Since HTTPS is HTTP protected by some security protocols the information is sent as:

HTTP Connection
TCP -> HTTP -> HTML -> TEXT

HTTPS Connection
TCP -> TLS -> HTTP -> HTML -> TEXT

---

Since I'm working in C, we need to handle sockets to create the network connections between the client and the server.

The high level steps for a proxy are:
1. Create a socket
2. Bind the socket to an address
3. Listen for connections
4. Accept a connection
5. Read from the connection
6. Handle proxy logic
7. Send the request to the server
8. Receive the response from the server
9. Send the response back to the client

References
[1] https://stackoverflow.com/questions/516323/https-connections-over-proxy-servers
[2] Hands on Network Programming with C
