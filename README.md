# SQLx MySQL Demo

A simple Rust application demonstrating MySQL database connectivity using SQLx. This project implements basic TODO list management functionality.

## Features

- Add new TODO tasks
- Mark tasks as completed
- List all TODO tasks

## Technology Stack

- Rust 2024 Edition
- SQLx 0.8.5
- MySQL 8.0.41
- Docker & Docker Compose
- tokio (async runtime)

## Setup

### Prerequisites

- Rust installed
- Docker & Docker Compose installed

### Steps

1. Clone the repository

```bash
git clone https://github.com/yourusername/sqlx-mysql-demo.git
cd sqlx-mysql-demo
```

2. Set up environment variables

Create a `.env` file with the following content:

```
DATABASE_URL=mysql://testuser:testpassword@localhost:3306/testdb
```

3. Start the MySQL container

```bash
docker-compose up -d
```

4. Run migrations (they are run automatically during container initialization, but just to be sure)

```bash
sqlx migrate run
```

## Usage

### Add a task

```bash
cargo run -- add "Description of the new task"
```

### Mark a task as done

```bash
cargo run -- done <task_id>
```

### List all tasks

```bash
cargo run
```

## Testing

To run the tests:

```bash
cargo test
```

Tests automatically run migrations and use fixtures to test each functionality.

## Project Structure

- `src/main.rs` - Main application code
- `migrations/` - SQL migration files
- `src/fixtures/` - Test fixtures

## License

MIT
