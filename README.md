 # Rust Database

 Rust Database is a Rust-based project designed to emulate a MySQL-like database system. It includes:

 - **core**: A library providing fundamental database functionality for use in other projects.
 - **server**: An executable that exposes API endpoints (e.g., HTTP) to interact with the database, acting as a full-fledged database management system.
 - **client**: An example implementation showcasing how to interact with the database as a library.

 ## Getting Started

 ### Running the Server

 To start the server locally:

 ```bash
 $ cargo run --package server
 ```

 Access the server at [http://localhost:3000](http://localhost:3000).

 ### Running the Client

 The client's binary code serves as a reference implementation and can be started using:

 ```bash
 $ cargo run --package client --bin client
 ```

 ### Docker/Podman Usage

 To run the server using Docker or Podman, use `docker-compose` or `podman-compose`:

 ```bash
 $ docker-compose up
 # or
 $ podman-compose up
 ```

 To remove the server container:

 ```bash
 $ docker-compose rm -s -f
 # or
 $ podman-compose down -v --remove-orphans
 ```

 ### Screenshots

 ![Webpage Screenshot](server_webpage_screenshot.png)

 ### Documentation

 For detailed documentation:

 - **core**: `cargo doc --open --package core`
 - **server**: `cargo doc --open --package server`
 - **client**: `cargo doc --open --package client`