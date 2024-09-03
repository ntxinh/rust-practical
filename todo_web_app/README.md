# Install `sqlx-cli`

```bash
# sudo apt install pkg-config
# sudo apt install libssl-dev
rustup update
cargo install sqlx-cli
# sqlx migrate add -r init
```

# Create a new project

```bash
cargo new todo_web_app
```

# Create `.env` file from `.env.sample` file

# Change to the project directory

```bash
cd todo_web_app
sqlx migrate run
cargo build
cargo run
```

http://localhost:8000

# References

- https://github.com/wpcodevo/rust-mysql-crud-sqlx