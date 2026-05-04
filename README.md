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

## Production Deployment

The `deploy/` directory contains an interactive installer for Ubuntu/Debian VPS hosts.

### Prerequisites
- Ubuntu/Debian with root (sudo) access
- A MySQL instance reachable from the VPS
- DNS records pointing your frontend (and optionally backend) domain(s) at the VPS
- Optional: Let's Encrypt certificates at `/etc/letsencrypt/live/<domain>/` if you want HTTPS

### Install
1. Clone this repository onto the server (any location — the installer can relocate it):
   ```bash
   git clone <repo-url> daily-tracker
   cd daily-tracker
   ```
2. Run the interactive setup script as root:
   ```bash
   sudo bash deploy/setup.sh
   ```
3. Answer the prompts:
   - **Installation directory** — where the app code will live (default `/opt/daily-tracker`)
   - **Service user** — system account that runs the backend (default `daily-tracker`)
   - **MySQL `DATABASE_URL`** — e.g. `mysql://user:pass@localhost/daily_tracker`
   - **Frontend domain name** — e.g. `app.example.com`
   - **Backend domain name** — defaults to the frontend domain for a single-domain setup; set a different value (e.g. `api.example.com`) to serve the API on its own host
   - **JWT secret** — leave blank to auto-generate a 32-byte random value
   - **HTTPS** — enable to produce an nginx config that uses Let's Encrypt certs

   The script installs system packages (Rust, Node.js, Diesel CLI, nginx), builds the backend and frontend, writes `backend/.env`, runs migrations, installs the systemd unit, configures nginx, and starts the services. Your answers are saved to `deploy/config.env` so the installer's choices are reused by `deploy.sh`.

### Subsequent deploys
From the installation directory:
```bash
sudo bash deploy/deploy.sh
```
This pulls the latest code, rebuilds, reruns migrations, and restarts the backend and nginx.

### Useful commands
```bash
systemctl status daily-tracker-backend     # service status
journalctl -u daily-tracker-backend -f     # follow backend logs
nginx -t && systemctl reload nginx         # test + reload nginx config
```

## API Documentation
The API schema is defined using OpenAPI. You can find the raw specification in `swagger.json` in the root directory.

If you make modifications to the backend's API signature, ensure you update `swagger.json` and optionally regenerate the frontend API client bindings:
```bash
cd frontend
npm run generate:api
```
