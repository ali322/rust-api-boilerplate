# Rust API Boilerplate

fundamental api services with [Rocket](https://rocket.rs/)

## Prerequirements

install [Postgresql] in docker

```bash
docker run --name postgres -p 5438:5432 -v ${PWD}/postgresql_data:/var/lib/postgresql/data -e POSTGRES_PASSWORD=postgres -d postgres
```

## Get Started

```bash
cargo run
```