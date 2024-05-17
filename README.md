# Database template

This is a template project for a database written in Rust. It is split up into multiple sub-projects:
- `core` is a library project that contains the core database code. The idea is that the database itself is just a library that can be used by other projects, but you will also provide a standalone server (a database management system) that uses this `core` library
- `server` is an executable for a server that provides API access (e.g. through HTTP or some other protocol) that lets other applications talk to your database 