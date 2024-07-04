# Database template

This is a template project for a database written in Rust. It is split up into multiple sub-projects:
- `core` is a library project that contains the core database code. The idea is that the database itself is just a library that can be used by other projects, but you will also provide a standalone server (a database management system) that uses this `core` library
- `server` is an executable for a server that provides API access (e.g. through HTTP or some other protocol) that lets other applications talk to your database 


The Server can be started using docker-compose or podman-compose
```
$ docker-compose up
$ podman-compose up
```
If you want to __remove__ the server container, use the appropriate command based on whether you are using Docker or Podman:
```
$ docker-compose rm -s -f
$ podman-compose down -v --remove-orphans
```

```
podman-compose up --build --force-recreate
```