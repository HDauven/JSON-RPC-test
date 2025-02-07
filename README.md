# Axum JSON-RPC Server
A lightweight JSON-RPC 2.0 server built with Axum and Yerpc, supporting both HTTP and WebSocket requests.

## Features
- JSON-RPC 2.0 compliant  
- Supports both HTTP & WebSocket  
- Built with Axum for performance  
- Modular architecture for scalability  

## Installation

### Clone the repository
```sh
git clone https://github.com/HDauven/JSON-RPC-test.git
cd JSON-RPC-test
```

### Build the project

```sh
cargo build --release
```

### Running the Server

```sh
cargo run
```

By default, the server runs on http://127.0.0.1:3000.

## API Endpoints

### HTTP JSON-RPC

Endpoint
```ssh
POST http://127.0.0.1:3000/rpc
```

Example Request
```json
{
  "jsonrpc": "2.0",
  "method": "get_block_height",
  "id": 1
}
```


Example Response
```json
{
  "jsonrpc": "2.0",
  "result": 123456,
  "id": 1
}
```

Test with cURL
```sh
curl -X POST http://127.0.0.1:3000/rpc \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"get_block_height","id":1}'
```

### WebSocket JSON-RPC

Endpoint
```sh
ws://127.0.0.1:3000/ws
```

Example Request
```json
{
  "jsonrpc": "2.0",
  "method": "get_balance",
  "params": { "address": "0x1234" },
  "id": 2
}
```

Example Response
```json
{
  "jsonrpc": "2.0",
  "result": { "balance": 1000 },
  "id": 2
}
```

Test with `wscat`
```sh
wscat -c ws://127.0.0.1:3000/ws
```

And send:
```json
{"jsonrpc": "2.0", "method": "get_block_height", "id": 1}
```
