#!/usr/bin/env bash
#
# Interactive setup script for Daily Tracker.
#
# Builds the backend + frontend from a local source checkout, then installs
# only the resulting artifacts (binary + frontend dist) into INSTALL_DIR.
# Configuration lives in /etc/daily-tracker/. No source code is left on the
# server, so upgrades are a fresh clone-build-install cycle each time.
#
# Run as root: sudo bash deploy/setup.sh
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# ---------- Helpers ----------
prompt_default() {
    local label="$1" default="$2" val
    read -rp "  $label [$default]: " val </dev/tty
    echo "${val:-$default}"
}

prompt_required() {
    local label="$1" val=""
    while [[ -z "$val" ]]; do
        read -rp "  $label: " val </dev/tty
        [[ -z "$val" ]] && echo "    (required — please enter a value)"
    done
    echo "$val"
}

prompt_yes_no() {
    local label="$1" default="$2" val
    read -rp "  $label (y/n) [$default]: " val </dev/tty
    val="${val:-$default}"
    [[ "$val" =~ ^[Yy] ]] && echo "y" || echo "n"
}

if [[ $EUID -ne 0 ]]; then
    echo "This script must be run as root (use sudo)." >&2
    exit 1
fi

# ---------- Stable config locations ----------
SYSTEM_CONF_DIR="/etc/daily-tracker"
SYSTEM_CONF="$SYSTEM_CONF_DIR/config.env"
BACKEND_ENV="$SYSTEM_CONF_DIR/backend.env"

# ---------- Load previous answers if available ----------
DEF_INSTALL_DIR="/opt/daily-tracker"
DEF_SERVICE_USER="daily-tracker"
DEF_FRONTEND_DOMAIN=""
DEF_BACKEND_DOMAIN=""
DEF_ENABLE_HTTPS="n"

# Search the canonical path first, then a legacy location used by older
# versions of this script.
PREV_CONF=""
for candidate in "$SYSTEM_CONF" "$DEF_INSTALL_DIR/deploy/config.env"; do
    if [[ -f "$candidate" ]]; then
        PREV_CONF="$candidate"
        break
    fi
done
if [[ -n "$PREV_CONF" ]]; then
    # shellcheck disable=SC1090
    source "$PREV_CONF"
    DEF_INSTALL_DIR="${INSTALL_DIR:-$DEF_INSTALL_DIR}"
    DEF_SERVICE_USER="${SERVICE_USER:-$DEF_SERVICE_USER}"
    DEF_FRONTEND_DOMAIN="${FRONTEND_DOMAIN:-$DEF_FRONTEND_DOMAIN}"
    DEF_BACKEND_DOMAIN="${BACKEND_DOMAIN:-$DEF_BACKEND_DOMAIN}"
    DEF_ENABLE_HTTPS="${ENABLE_HTTPS:-$DEF_ENABLE_HTTPS}"
    echo "==> Found previous config at $PREV_CONF — defaults populated from it."
fi

# Pull existing secrets without ever displaying them. Check the canonical
# path first, then legacy backend/.env from older deployments.
read_env_var() {
    [[ -f "$1" ]] || return 0
    sed -n "s/^$2=//p" "$1" | head -n1
}
EXISTING_DB_URL=""
EXISTING_JWT_SECRET=""
for CANDIDATE in "$BACKEND_ENV" "$DEF_INSTALL_DIR/backend/.env"; do
    if [[ -f "$CANDIDATE" ]]; then
        EXISTING_DB_URL="$(read_env_var "$CANDIDATE" DATABASE_URL)"
        EXISTING_JWT_SECRET="$(read_env_var "$CANDIDATE" JWT_SECRET)"
        break
    fi
done

# ---------- Interactive configuration ----------
cat <<'BANNER'

==============================================
    Daily Tracker — Interactive Setup
==============================================
Answer the prompts below (press Enter to accept defaults in brackets).

BANNER

INSTALL_DIR=$(prompt_default "Installation directory (where compiled artifacts are stored)" "$DEF_INSTALL_DIR")
SERVICE_USER=$(prompt_default "Service user (system account that runs the backend)" "$DEF_SERVICE_USER")

if [[ -n "$EXISTING_DB_URL" ]]; then
    read -rp "  MySQL DATABASE_URL [keep existing]: " DATABASE_URL </dev/tty
    DATABASE_URL="${DATABASE_URL:-$EXISTING_DB_URL}"
else
    DATABASE_URL=$(prompt_required "MySQL DATABASE_URL (e.g. mysql://user:pass@localhost/daily_tracker)")
fi

if [[ -n "$DEF_FRONTEND_DOMAIN" ]]; then
    FRONTEND_DOMAIN=$(prompt_default "Frontend domain name (e.g. app.example.com)" "$DEF_FRONTEND_DOMAIN")
else
    FRONTEND_DOMAIN=$(prompt_required "Frontend domain name (e.g. app.example.com)")
fi

BACKEND_DOMAIN=$(prompt_default "Backend domain name (use same as frontend for single-domain setup)" "${DEF_BACKEND_DOMAIN:-$FRONTEND_DOMAIN}")

echo ""
if [[ -n "$EXISTING_JWT_SECRET" ]]; then
    read -rsp "  JWT_SECRET (leave blank to keep existing): " JWT_SECRET </dev/tty
    echo ""
    JWT_SECRET="${JWT_SECRET:-$EXISTING_JWT_SECRET}"
    echo "    Keeping existing JWT_SECRET."
else
    read -rsp "  JWT_SECRET (leave blank to auto-generate): " JWT_SECRET </dev/tty
    echo ""
    if [[ -z "$JWT_SECRET" ]]; then
        JWT_SECRET="$(openssl rand -hex 32)"
        echo "    Generated a random 32-byte JWT_SECRET."
    fi
fi

ENABLE_HTTPS=$(prompt_yes_no "Configure HTTPS in nginx (requires Let's Encrypt certs at /etc/letsencrypt/live/<domain>/)" "$DEF_ENABLE_HTTPS")

echo ""
echo "----------------------------------------------"
echo "Summary:"
echo "    Install dir:       $INSTALL_DIR"
echo "    Service user:      $SERVICE_USER"
echo "    MySQL URL:         [hidden]"
echo "    Frontend domain:   $FRONTEND_DOMAIN"
echo "    Backend domain:    $BACKEND_DOMAIN"
echo "    HTTPS:             $ENABLE_HTTPS"
echo "----------------------------------------------"
read -rp "Proceed with installation? (y/N): " CONFIRM </dev/tty
[[ "$CONFIRM" =~ ^[Yy] ]] || { echo "Aborted."; exit 1; }

# ---------- Install system packages ----------
echo "==> Installing system packages"
apt-get update
apt-get install -y build-essential pkg-config libssl-dev nginx curl git openssl rsync

# MySQL client dev headers: Ubuntu uses libmysqlclient-dev; Debian ships
# libmariadb-dev (wire-compatible, works with Diesel).
if apt-cache show libmysqlclient-dev 2>/dev/null | grep -q '^Package:'; then
    apt-get install -y libmysqlclient-dev
else
    echo "==> libmysqlclient-dev unavailable — installing libmariadb-dev instead"
    apt-get install -y libmariadb-dev
fi

# Install Rust if not present
if ! command -v cargo &>/dev/null; then
    echo "==> Installing Rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi
[[ -f "$HOME/.cargo/env" ]] && source "$HOME/.cargo/env"
export PATH="$HOME/.cargo/bin:/root/.cargo/bin:$PATH"

# Install / upgrade Node.js. Vite requires >= 20.19, and setup_lts.x has been
# known to serve Node 18 on some mirrors, so pin to NodeSource 22.x explicitly
# and reinstall if the existing major is too old.
NODE_MIN_MAJOR=20
NODE_CURRENT_MAJOR=0
if command -v node &>/dev/null; then
    NODE_CURRENT_MAJOR=$(node -v | sed 's/^v//; s/\..*//')
fi
if (( NODE_CURRENT_MAJOR < NODE_MIN_MAJOR )); then
    echo "==> Installing Node.js 22.x (found v${NODE_CURRENT_MAJOR:-none}, need >= $NODE_MIN_MAJOR)"
    curl -fsSL https://deb.nodesource.com/setup_22.x | bash -
    apt-get install -y nodejs
fi

# Install Diesel CLI if not present
if ! command -v diesel &>/dev/null; then
    echo "==> Installing Diesel CLI"
    cargo install diesel_cli --no-default-features --features mysql
fi

# ---------- Service user ----------
if ! id "$SERVICE_USER" &>/dev/null; then
    echo "==> Creating service user: $SERVICE_USER"
    useradd --system --no-create-home --shell /usr/sbin/nologin "$SERVICE_USER"
fi

# ---------- Write system config ----------
mkdir -p "$SYSTEM_CONF_DIR"
chmod 755 "$SYSTEM_CONF_DIR"

echo "==> Writing $BACKEND_ENV"
cat > "$BACKEND_ENV" <<EOF
DATABASE_URL=$DATABASE_URL
JWT_SECRET=$JWT_SECRET
BIND_ADDR=127.0.0.1:8080
CORS_ORIGIN=https://$FRONTEND_DOMAIN
EOF
chown root:root "$BACKEND_ENV"
chmod 600 "$BACKEND_ENV"

echo "==> Writing $SYSTEM_CONF"
cat > "$SYSTEM_CONF" <<EOF
# Generated by setup.sh — do not edit by hand.
INSTALL_DIR="$INSTALL_DIR"
SERVICE_USER="$SERVICE_USER"
FRONTEND_DOMAIN="$FRONTEND_DOMAIN"
BACKEND_DOMAIN="$BACKEND_DOMAIN"
ENABLE_HTTPS="$ENABLE_HTTPS"
EOF
chown root:root "$SYSTEM_CONF"
chmod 600 "$SYSTEM_CONF"

# ---------- Build backend ----------
echo "==> Building backend (release)"
cd "$SOURCE_REPO_DIR/backend"
cargo build --release

# ---------- Database migrations ----------
# diesel.toml lives in crates/db_model and points at ./migrations there.
# Migrations run from the source tree and are not copied to the server.
echo "==> Running database migrations"
cd "$SOURCE_REPO_DIR/backend/crates/db_model"
DATABASE_URL="$DATABASE_URL" diesel migration run

# ---------- Build frontend ----------
# Vite bakes VITE_API_BASE_URL into the bundle at build time. When frontend
# and backend share a domain, use a relative URL so the browser matches the
# page protocol — avoids mixed-content blocking when fronted by HTTPS even if
# ENABLE_HTTPS=n here.
echo "==> Building frontend"
cd "$SOURCE_REPO_DIR/frontend"
if [[ "$BACKEND_DOMAIN" == "$FRONTEND_DOMAIN" ]]; then
    API_BASE_URL="/api/v1"
else
    PROTO=$([[ "$ENABLE_HTTPS" == "y" ]] && echo https || echo http)
    API_BASE_URL="$PROTO://$BACKEND_DOMAIN/api/v1"
fi
cat > .env.production <<EOF
VITE_API_BASE_URL=$API_BASE_URL
EOF
npm ci
npm run build

# ---------- Install artifacts ----------
echo "==> Installing artifacts to $INSTALL_DIR"
mkdir -p "$INSTALL_DIR/bin" "$INSTALL_DIR/frontend"

# Backend binary: cargo's bin name uses underscores; rename to the dashed
# form that matches the systemd unit and CLI conventions.
install -m 0755 -o root -g root \
    "$SOURCE_REPO_DIR/backend/target/release/daily_tracker_backend" \
    "$INSTALL_DIR/bin/daily-tracker-backend"

# rsync --delete wipes anything previously in $INSTALL_DIR/frontend, so
# leftover source files from older deployments (src/, node_modules/, etc.)
# are removed in one shot.
rsync -a --delete --chmod=Du=rwx,Dgo=rx,Fu=rw,Fgo=r \
    "$SOURCE_REPO_DIR/frontend/dist/" "$INSTALL_DIR/frontend/"
chown -R root:root "$INSTALL_DIR/frontend"

# Make the path traversable for nginx (www-data) and the service user.
chmod 755 "$INSTALL_DIR" "$INSTALL_DIR/bin" "$INSTALL_DIR/frontend"

# ---------- Systemd service ----------
echo "==> Installing systemd service"
sed -e "s|@INSTALL_DIR@|$INSTALL_DIR|g" \
    -e "s|@SERVICE_USER@|$SERVICE_USER|g" \
    "$SOURCE_REPO_DIR/deploy/daily-tracker-backend.service" \
    > /etc/systemd/system/daily-tracker-backend.service
systemctl daemon-reload
systemctl enable daily-tracker-backend

# ---------- Nginx ----------
echo "==> Configuring nginx"
NGINX_CONF="/etc/nginx/sites-available/daily-tracker"

WRITE_NGINX=y
if [[ -f "$NGINX_CONF" ]]; then
    echo "    $NGINX_CONF already exists."
    WRITE_NGINX=$(prompt_yes_no "Overwrite existing nginx config" "n")
fi

if [[ "$WRITE_NGINX" != "y" ]]; then
    echo "    Keeping existing nginx config — skipping write, symlink, and nginx restart."
fi

write_ssl_lines() {
    local domain="$1"
    cat <<EOF
    ssl_certificate     /etc/letsencrypt/live/$domain/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/$domain/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
EOF
}

write_api_proxy_block() {
    cat <<'EOF'
    location /api/ {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
EOF
}

if [[ "$WRITE_NGINX" == "y" ]]; then
{
    if [[ "$ENABLE_HTTPS" == "y" ]]; then
        cat <<EOF
server {
    listen 80;
    server_name $FRONTEND_DOMAIN;
    return 301 https://\$host\$request_uri;
}

server {
    listen 443 ssl http2;
    server_name $FRONTEND_DOMAIN;

$(write_ssl_lines "$FRONTEND_DOMAIN")

    root $INSTALL_DIR/frontend;
    index index.html;

    location /assets/ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    location / {
        try_files \$uri \$uri/ /index.html;
    }
EOF
        if [[ "$FRONTEND_DOMAIN" == "$BACKEND_DOMAIN" ]]; then
            write_api_proxy_block
        fi
        echo "}"

        if [[ "$FRONTEND_DOMAIN" != "$BACKEND_DOMAIN" ]]; then
            cat <<EOF

server {
    listen 80;
    server_name $BACKEND_DOMAIN;
    return 301 https://\$host\$request_uri;
}

server {
    listen 443 ssl http2;
    server_name $BACKEND_DOMAIN;

$(write_ssl_lines "$BACKEND_DOMAIN")

$(write_api_proxy_block)
}
EOF
        fi
    else
        cat <<EOF
server {
    listen 80;
    server_name $FRONTEND_DOMAIN;

    root $INSTALL_DIR/frontend;
    index index.html;

    location /assets/ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    location / {
        try_files \$uri \$uri/ /index.html;
    }
EOF
        if [[ "$FRONTEND_DOMAIN" == "$BACKEND_DOMAIN" ]]; then
            write_api_proxy_block
        fi
        echo "}"

        if [[ "$FRONTEND_DOMAIN" != "$BACKEND_DOMAIN" ]]; then
            cat <<EOF

server {
    listen 80;
    server_name $BACKEND_DOMAIN;

$(write_api_proxy_block)
}
EOF
        fi
    fi
} > "$NGINX_CONF"

    ln -sf "$NGINX_CONF" /etc/nginx/sites-enabled/daily-tracker
    rm -f /etc/nginx/sites-enabled/default
    nginx -t
    systemctl restart nginx
    systemctl enable nginx
fi

# ---------- (Re)start backend ----------
systemctl restart daily-tracker-backend

cat <<EOF

==> Setup complete!

    Install dir:    $INSTALL_DIR
    Backend bin:    $INSTALL_DIR/bin/daily-tracker-backend
    Frontend root:  $INSTALL_DIR/frontend
    Backend env:    $BACKEND_ENV
    Deploy config:  $SYSTEM_CONF
    Backend logs:   /var/log/daily-tracker/backend-YYYY-MM-DD.log (UTC, daily)
                    /var/log/daily-tracker/current.log -> latest
    Frontend URL:   $( [[ "$ENABLE_HTTPS" == "y" ]] && echo "https" || echo "http" )://$FRONTEND_DOMAIN
    Backend URL:    $( [[ "$ENABLE_HTTPS" == "y" ]] && echo "https" || echo "http" )://$BACKEND_DOMAIN/api/v1

    Backend status: systemctl status daily-tracker-backend
    Live logs:      tail -F /var/log/daily-tracker/current.log
    Journald:       journalctl -u daily-tracker-backend -f

EOF

# ---------- Legacy cleanup hint ----------
LEGACY_PATHS=()
for p in \
    "$INSTALL_DIR/backend" \
    "$INSTALL_DIR/deploy" \
    "$INSTALL_DIR/Cargo.toml" \
    "$INSTALL_DIR/Cargo.lock" \
    "$INSTALL_DIR/Makefile" \
    "$INSTALL_DIR/.git" \
    "$INSTALL_DIR/swagger.json" \
    "$INSTALL_DIR/README.md"
do
    [[ -e "$p" ]] && LEGACY_PATHS+=("$p")
done
if (( ${#LEGACY_PATHS[@]} > 0 )); then
    cat <<EOF
==> Legacy source files detected from a previous deployment style.
    These are no longer used and can be safely removed:

$(for p in "${LEGACY_PATHS[@]}"; do echo "      $p"; done)

    To clean up:
      sudo rm -rf$(printf ' %q' "${LEGACY_PATHS[@]}")

EOF
fi
