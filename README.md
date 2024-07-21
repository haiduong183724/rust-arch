## Install diesel cli 
```
sudo apt-get install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
```


## Create a migration 

```bash
diesel setup
diesel migration generate create_user > /dev/null 2>&1
```

## Runa migration 
```bash
export DATABASE_URL=postgres://useer:password@localhost:6432/rust-arch
diesel migration run
```
## Run and watch
```bash
cargo watch -x "run --release"
```

