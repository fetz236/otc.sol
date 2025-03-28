# otc.sol

Trade solana OTC. Project to facilitate high volume solana trades over the counter. 

## Setup

1. Make sure PostgreSQL is installed and running
2. Create a test database:

```bash
createdb otc_sol_test
```

3. Set up environment variables:

```bash
export DATABASE_URL="postgres://YOUR_USERNAME@localhost:5432/otc_sol_test"
export TEST_DATABASE_URL="postgres://YOUR_USERNAME@localhost:5432/otc_sol_test"
```

4. Run database migrations:

```bash
diesel migration run
```

## Running Tests

Run the tests with:

```bash
cargo test -- --test-threads=1
```

Note: Make sure you have the diesel CLI installed. If not, install it with:

```bash
cargo install diesel_cli --no-default-features --features postgres
```
