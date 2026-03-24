# Daily Tracker

A full-stack web application designed to help you track your daily activities in 30-minute intervals. 

## Features
- **Hierarchical Topics**: Organize your tracked activities by topics and sub-topics.
- **Daily Time Tracking**: Record comments and associate specific topics with 30-minute time slots throughout the day.
- **User Authentication**: Secure user registration and login using JWT.
- **Data Isolation**: Each user has their own private set of topics and daily tracks.

## Technology Stack

### Backend
- **Language:** Rust
- **Framework:** Axum
- **Database ORM:** Diesel (MySQL)
- **Authentication:** JSON Web Tokens (JWT) & bcrypt

### Frontend
- **Framework:** React with Vite
- **Language:** TypeScript
- **API Client:** Generated via `openapi-typescript-codegen`

## Getting Started

### Prerequisites
- Node.js & npm
- Rust & Cargo
- MySQL Database

### Backend Setup
1. Navigate to the `backend` directory.
2. Ensure you have a `.env` file set up with your `DATABASE_URL` (pointing to your MySQL database) and a `JWT_SECRET` key.
3. Run migrations to initialize your database tables: 
   ```bash
   diesel setup
   diesel migration run
   ```
4. Start the Axum server: 
   ```bash
   cargo run
   ```

### Frontend Setup
1. Navigate to the `frontend` directory.
2. Install the required Node dependencies:
   ```bash
   npm install
   ```
3. Start the Vite development server:
   ```bash
   npm run dev
   ```

## API Documentation
The API schema is defined using OpenAPI. You can find the raw specification in `swagger.json` in the root directory.

If you make modifications to the backend's API signature, ensure you update `swagger.json` and optionally regenerate the frontend API client bindings:
```bash
cd frontend
npm run generate:api
```
