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

The server will start at [YOUR_DOMAIN]:[YOUR_PORT].

## Configuration

The application configuration is located in `src/config.rs`.

- **Base Directory**: Files are stored in [YOUR_STORAGE_PATH] by default.
- **CORS**: Configured to allow specific origins (default: `*`).

## API Endpoints

### 1. Upload File

**POST** `/api/upload`

Uploads a file to the storage. Supports `multipart/form-data`.

**Form Fields:**

- `path` (text, optional): The relative path where the file should be saved (e.g., `images/user1`).
- `file` (file, required): The file to upload.

**Example (curl):**

```bash
curl -X POST [YOUR_DOMAIN]:[YOUR_PORT]/api/upload \
  -F "path=documents/work" \
  -F "file=@./my-document.pdf"
```

### 2. Delete File/Directory

**DELETE** `/api/delete`

Recursively deletes a file or directory.

**Query Parameters:**

- `path` (required): The relative path to delete.

**Example:**

```bash
curl -X DELETE "[YOUR_DOMAIN]:[YOUR_PORT]/api/delete?path=documents/work"
```

### 3. List Files

**GET** `/api/list`
_(Implementation details depend on `src/handlers/list.rs`)_

### 4. Get Size

**GET** `/api/size`
_(Implementation details depend on `src/handlers/size.rs`)_

## Project Structure

- `src/main.rs`: Application entry point and server setup.
- `src/config.rs`: Configuration struct and loader.
- `src/routes.rs`: API route definitions.
- `src/handlers/`: Request handlers for upload, delete, etc.
- `src/utils/`: Utility functions for file system operations and path safety.

## License

This project is open-source and available under the MIT License.
