# Lamplace Storage

Lamplace Storage is a lightweight, high-performance file storage service built with Rust and Actix-web. It provides a simple HTTP API for uploading, managing, and deleting files with built-in path safety mechanisms.

## Features

- 🚀 **High Performance**: Built on top of Actix-web and Tokio for asynchronous I/O.
- 📂 **File Management**: Upload files to specific directories, delete files/folders recursively.
- 🔒 **Path Safety**: Prevents directory traversal attacks using sanitized paths.
- 📊 **Storage Insights**: Check file/directory sizes and list contents (endpoints available).

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd template_storage
   ```

2. Run the server:
   ```bash
   cargo run
   ```

The server will start at `http://127.0.0.1:8080` by default.

## Configuration

The application configuration is located in `src/config.rs` and `src/constants.rs`.

You can configure the following via environment variables (create a `.env` file):

- **DOMAIN**: Server domain (default: `127.0.0.1`)
- **PORT**: Server port (default: `8080`)
- **ALLOW_DOMAIN**: CORS allowed origins (default: `*`)

Files are stored in the `uploads/` directory by default.

## API Endpoints

### 1. API Information

**GET** `/`

Returns API status and available endpoints.

**Example:**

```bash
curl http://127.0.0.1:8080/
```

**Response:**

```json
{
  "status": "running",
  "endpoints": {
    "GET /": "API information",
    "GET /uploads/": "Display files in storage",
    "POST /api/upload": "Upload files (form-data: path, file)",
    "POST /api/list": "List files (JSON: {path, limit?})",
    "POST /api/size": "Get size (JSON: {path})",
    "DELETE /api/delete": "Delete path (JSON: {path})"
  }
}
```

### 2. Upload File

**POST** `/api/upload`

Uploads a file to the storage. Supports `multipart/form-data`.

**Form Fields:**

- `path` (text, optional): The relative path where the file should be saved (e.g., `images/user1`).
- `file` (file, required): The file to upload.

**Example:**

```bash
curl -X POST http://127.0.0.1:8080/api/upload \
  -F "path=documents/work" \
  -F "file=@./my-document.pdf"
```

### 3. List Files

**POST** `/api/list`

Lists files and directories in a specified path.

**Request Body (JSON):**

- `path` (string, required): The relative path to list.
- `limit` (number, optional): Maximum number of items to return.

**Example:**

```bash
curl -X POST http://127.0.0.1:8080/api/list \
  -H "Content-Type: application/json" \
  -d '{"path": "documents", "limit": 10}'
```

**Response:**

```json
["file1.pdf", "file2.txt", "subfolder"]
```

### 4. Get Size

**POST** `/api/size`

Gets the total size of a file or directory (recursive).

**Request Body (JSON):**

- `path` (string, required): The relative path to check.

**Example:**

```bash
curl -X POST http://127.0.0.1:8080/api/size \
  -H "Content-Type: application/json" \
  -d '{"path": "documents"}'
```

**Response:**

```json
{
  "bytes": 1048576
}
```

### 5. Delete File/Directory

**DELETE** `/api/delete`

Recursively deletes a file or directory.

**Query Parameters:**

- `path` (required): The relative path to delete.

**Example:**

```bash
curl -X DELETE "http://127.0.0.1:8080/api/delete?path=documents/work"
```

### 6. Browse Files

**GET** `/uploads/{path}`

Serves static files from the uploads directory. Access uploaded files directly via browser or download.

## Project Structure

- `src/main.rs`: Application entry point and server setup.
- `src/config.rs`: Configuration struct and loader.
- `src/routes.rs`: API route definitions.
- `src/handlers/`: Request handlers for upload, delete, etc.
- `src/utils/`: Utility functions for file system operations and path safety.

## License

This project is open-source and available under the MIT License.
