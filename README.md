# HTTP-server
**HTTP Server in Rust**
<br>
This project implements a basic HTTP/1.1 server in Rust. The server focuses on core HTTP functionality.
<br><br>
**Project Overview**
<br>
The server consists of three main components:
<br>
1. TCP Listener<br>
2. HTTP Parser<br>
3. Request Handler<br>

**HTTP/1.1 Implementation**
<br>
Our server implements a subset of HTTP/1.1 protocol, focusing on:<br>
**Request:** Method, Path, Query String, Protocol Version<br>
**Response:** Protocol, Status Code, Body<br>

**Server Architecture**
<br>
1. TCP Listener: Listens for new TCP connections, reads and writes bytes.<br>
2. HTTP Parser: Parses incoming bytes into HTTP data structures.<br>
3. Request Handler: Implements routing logic and generates HTTP responses.
<br>

**Features**
<br>
1. Single-threaded server handling one request at a time<br>
2. Support for main HTTP methods (GET, POST, PUT, etc.)<br>
3. Basic routing capabilities<br>
4. Response generation based on request parameters<br><br>
