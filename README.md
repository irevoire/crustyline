# Crustyline

The first aim of this project is to learn rust.
The second aim of this project is to know what I'll eat this week.

This project should be deployed at https://wl.irevoire.ovh


## How do I start

### Required software

You'll need rust and cargo. You should install everything from rustup : https://doc.rust-lang.org/cargo/getting-started/installation.html

### Building

```
cargo build
```

### Running

```
cargo run
```

### Docker

#### Pulling the image:
```
docker pull reg.irevoire.ovh/crustyline
```

#### Running the image:
```
docker run --restart=always -d --name crustyline reg.irevoire.ovh/crustyline
```
