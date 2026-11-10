# Taskr - Task Management API

A simple and fast REST API for managing tasks, built with [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/).

## Features

- **Fast & Async**: Built on Tokio for high-performance async I/O
- **Lightweight**: Minimal dependencies using the Axum web framework
- **RESTful API**: Simple endpoints for task management
- **Health Checks**: Built-in health check endpoint for monitoring

## Getting Started

### Prerequisites

- Rust 1.56 or later
- Cargo

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd taskr
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run
```

The server will start on `http://localhost:3500` by default.

### Configuration

You can configure the port using the `PORT` environment variable:

```bash
PORT=8080 cargo run
```

Or on Windows:
```powershell
$env:PORT=8080; cargo run
```

## API Endpoints

### Root Endpoint
- **GET** `/`
  - Returns: `"taskr is alive"`

### Health Check
- **GET** `/health`
  - Returns: `"ok"`
  - Use this endpoint to check if the server is running

### List Tasks
- **GET** `/tasks`
  - Query Parameters:
    - `done` (optional): Filter tasks by completion status
  - Returns: List of tasks filtered by the provided parameters
  - Examples:
    ```
    GET /tasks
    GET /tasks?done=true
    GET /tasks?done=false
    ```

### Get Task by ID
- **GET** `/tasks/{id}`
  - Path Parameters:
    - `id` (required): The task ID (u32)
  - Returns: Task details for the specified ID
  - Example:
    ```
    GET /tasks/1
    ```

### Create Task
- **POST** `/tasks`
  - Request Body:
    ```json
    {
      "title": "Task title"
    }
    ```
  - Returns: `"task created"`

## Project Structure

```
taskr/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Project manifest
├── Cargo.lock           # Dependency lock file
└── README.md            # This file
```

## Data Structures

### Task
```rust
struct Task {
    id: u32,
    title: String,
    done: bool,
}
```

### CreateTask (Request Body)
```rust
struct CreateTask {
    title: String,
}
```

## Dependencies

- **axum** (0.8.9): Web framework for building HTTP services
- **tokio** (1.x): Async runtime with full features
- **serde** (1.x): Serialization/deserialization framework

## Development

### Running Tests
```bash
cargo test
```

### Formatting Code
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Future Enhancements

- [ ] Persistent storage (database integration)
- [ ] Task update/delete endpoints
- [ ] Request validation
- [ ] Error handling middleware
- [ ] Authentication/Authorization
- [ ] Comprehensive test coverage
- [ ] Docker support

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]

## Support

For issues and questions, please open an issue on the repository.
