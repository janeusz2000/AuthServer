# Authentication Server

## Introduction
This project is an authentication server built with Rust using the Actix web framework. It provides a robust and efficient way to handle user authentication and session management in web applications.

## Features
- User registration and login
- Session management with Actix Session
- Request throttling with Actix Limitation
- Environment configuration for various parameters like database, server address, port, etc.
- Debug mode for testing and development
- XML API endpoint for custom XML requests
- Comprehensive logging with colored output

## Prerequisites
- Rust (latest version)
- Docker
- PostgreSQL database service

## Installation

### Setting up PostgreSQL
This application requires a running PostgreSQL service. Ensure you have PostgreSQL installed and running on your machine or network.

### Building with Docker
1. Clone the repository:
   ```bash
   git clone [your-repository-url]
   cd [project-directory]
   docker build -t auth_server .
   ```

## Usage

### Running the Server
- Run the server using Docker:
  ```bash
  docker run -p 8080:8080 auth_server
  ```
The server will be available at localhost:8080


### Environment Variables
You can set various environment variables to configure the server:
- `AUTH_SERVER_ADDRESS` - Address for the server (default: localhost)
- `AUTH_SERVER_PORT` - Port for the server (default: 8080)
- `DATABASE_ADDRESS` - Database address (default: localhost)
- `DATABASE_USER` - Database user (default: root)
- `DATABASE_PASSWORD` - Database password (default: password)
- `DATABASE_PORT` - Database port (default: 3306)
- `DATABASE_NAME` - Database name (default: user_database)

### API Endpoints
- `POST /auth/register`: Register a new user
- `POST /auth/login`: Login an existing user
- `GET /auth/logout`: Logout a user
- `POST /auth/refresh`: Refresh the authentication token
- `POST /xml-api/send_xml`: Custom XML request endpoint

## Contributing
Contributions to this project are welcome. Please follow these steps to contribute:
1. Fork the repository
2. Create a new branch for your feature
3. Commit your changes
4. Push to the branch
5. Open a pull request
