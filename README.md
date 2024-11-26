# GateKeeper

## Goal of the Project

Build a high-performance API Gateway in Rust that acts as a central hub for managing, monitoring, and forwarding client requests to backend microservices. The gateway provides secure authentication (API keys with optional OAuth2 and JWT support), rate limiting, caching, and request routing. It includes detailed logging and analytics, a developer interface for generating and managing API keys, and a dashboard for visualizing key statistics—all designed for scalability and handling thousands of concurrent requests.


- **PostgreSQL and Redis status:**
    
    - [![Start Postgres and Redis Services](https://github.com/LunnosMp4/GateKeeper/actions/workflows/docker-services.yml/badge.svg)](https://github.com/LunnosMp4/GateKeeper/actions/workflows/docker-services.yml)

## Backend

The backend is implemented in Rust using the Actix-web framework. It includes the following features:

- User authentication and registration
- Admin dashboard for managing users
- **PostgreSQL** for database management
- **Redis** for caching and rate limiting
- **GraphQL** API for querying data
- **REST API** for user management
- Middleware for API key validation, rate limiting, and logging

### Main Dependencies

- `actix-web`
- `async-graphql`
- `sqlx`
- `redis`
- `dotenv`

### Setup

1. Install Rust and Cargo.
2. Clone the repository.
3. Navigate to the `backend` directory.
4. Create a `.env` file and set the required environment variables (`DATABASE_URL`, `REDIS_URL`, etc.).
5. Run `cargo build` to build the project.
6. Run `cargo run` to start the server.

### Database Configuration

The following PostgreSQL tables are used in this project:

#### Table: `users`

Stores user information and authentication details.

```sql
CREATE TABLE public.users (
    id integer NOT NULL DEFAULT nextval('public.users_id_seq'::regclass),
    name character varying(100) NOT NULL,
    email character varying(100) NOT NULL,
    api_key text,
    permission smallint DEFAULT 0 NOT NULL,
    password_hash text DEFAULT ''::text NOT NULL
);
```

#### Table: `api_usage`

Tracks API usage, including details of the request and response.

```sql
CREATE TABLE public.api_usage (
    id integer NOT NULL DEFAULT nextval('public.api_usage_id_seq'::regclass),
    user_id integer NOT NULL,
    api_key character varying NOT NULL,
    request_path character varying NOT NULL,
    request_method character varying NOT NULL,
    request_time timestamp without time zone DEFAULT now() NOT NULL,
    request_ip character varying NOT NULL,
    status_code integer NOT NULL
);
```


### API

The backend server includes both a GraphQL and REST API. To get an API key, you need to register as a user and log in.

#### 1. GraphQL

The GraphQL API is available at the `/api/graphql` endpoint. You can access the GraphQL playground at the `/playground` endpoint.

#### 2. REST API

The REST API is available at the `/api/v1` endpoint.

## Frontend

The frontend is implemented in **Vue.js**. It includes the following features:

- User login and registration
- Display user information
- Admin page for managing users
- API key refresh functionality
- Complete API key statistics:
  - Total requests
  - Requests per minute
  - Requests per hour
  - Requests per day
  - Requests per week
  - All requests made by the user (with path, method, timestamp and status code)

### Main Dependencies

- `vue`
- `vue-router`

### Setup

1. Install Node.js and npm.
2. Navigate to the `frontend` directory.
3. Run `npm install` to install the dependencies.
4. Run `npm run serve` to start the development server.
5. Access the application at `http://localhost:3000`.

## Docker

The project includes Docker configurations for both the backend and frontend.

### Setup

1. Install Docker.
2. Navigate to the backend directory of the project.
3. Run `docker-compose up` to build and start the containers.

For the moment the frontend is not included in the Docker configuration.

Starting Docker containers will start Rust, PostgreSQL and Redis services as well a `http://localhost:8080`.

## File Structure

```
.
├── backend
│   ├── src
│   │   ├── main.rs
│   │   ├── routes
│   │   ├── middlewares
│   │   ├── models
│   │   ├── utils
│   │   └── graphql
│   ├── Dockerfile
│   ├── docker-compose.yml
│   ├── .env
│   └── Cargo.toml
├── frontend
│   ├── src
│   │   ├── main.js
│   │   ├── router
│   │   ├── views
│   │   └── components
│   └── package.json
└── README.md
```

## Environment Variables

Create a `.env` file in the `backend` directory with the following variables:

```
DATABASE_URL=your_database_url
REDIS_URL=your_redis_url
RUST_LOG=actix_web=debug
```

3. Access the application at `http://localhost:8080` for the backend and `http://localhost:3000` for the frontend.

## Documentation
The documentation for this project does not exist yet, but it will be added in the future with all the details about the project and how to contribute.

## License

This project is licensed under the MIT License.
