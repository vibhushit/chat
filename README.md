# Chat Application

This is a chat application built using Rust. It allows multiple clients to connect to a server, join chat rooms, and exchange messages in real-time.

## Features

- Asynchronous communication using `async-std` and `tokio`.
- JSON serialization for message exchange using `serde` and `serde_json`.
- Multiple clients can join the same chat room and see each other's messages.

## Dependencies

- `async-std`: For asynchronous I/O operations.
- `tokio`: For managing asynchronous tasks and synchronization.
- `serde` and `serde_json`: For serializing and deserializing data to/from JSON.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your system.

### Installation

Clone the repository:

```bash
git clone https://github.com/vibhushit/chat
cd chat
```

### Running the Application

#### Step 1: Run the Server

Open a terminal and start the server by running:

```bash
cargo run --bin server localhost:8080
```

#### Step 2: Run Client 1

In another terminal, start the first client:

```bash
cargo run --bin client localhost:8080
```

#### Step 3: Run Client 2

In yet another terminal, start the second client:

```bash
cargo run --bin client localhost:8080
```

## Usage

Once the clients are running, you will see options displayed in each client's terminal:

```
Options:
join CHAT
post CHAT MESSAGE
```

### Commands

- **Join a Chat Room**:  
  To join a chat room named `TEST`, type:
  ```bash
  join TEST
  ```

- **Post a Message**:  
  To post a message in the `TEST` chat room, type:
  ```bash
  post TEST "hi from client 1"
  ```

Messages sent by one client will be visible to all other clients who have joined the same chat room.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
