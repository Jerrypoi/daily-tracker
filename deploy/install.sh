#!/usr/bin/env bash
#
# Bootstrap installer for Daily Tracker.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/Jerrypoi/daily-tracker/main/deploy/install.sh | sudo bash
#
# Clones the repo into a temp dir and hands off to deploy/setup.sh, which
# prompts for install dir / domain / DB URL / etc. and stages everything.
#
# Override the source via env vars:
#   REPO_URL=...  (default: git@github.com:Jerrypoi/daily-tracker.git)
#   BRANCH=...    (default: main)
#
# SSH note: this script runs as root, so root must have an SSH key authorized
# to read the repo (e.g. add a deploy key to /root/.ssh/id_ed25519, or run
# `sudo -E` after `ssh-add`-ing your key, etc.).
set -euo pipefail

REPO_URL="${REPO_URL:-git@github.com:Jerrypoi/daily-tracker.git}"
BRANCH="${BRANCH:-main}"

if [[ $EUID -ne 0 ]]; then
    echo "This installer must be run as root (use sudo)." >&2
    exit 1
fi

if ! command -v git &>/dev/null; then
    echo "==> Installing git"
    apt-get update
    apt-get install -y git
fi

WORK_DIR="$(mktemp -d -t daily-tracker-install-XXXXXX)"
trap 'rm -rf "$WORK_DIR"' EXIT

# Pre-seed github.com's host key so the SSH clone doesn't hang on the
# interactive "Are you sure you want to continue connecting?" prompt when
# root has never connected to github before.
mkdir -p /root/.ssh
chmod 700 /root/.ssh
if ! ssh-keygen -F github.com -f /root/.ssh/known_hosts &>/dev/null; then
    echo "==> Adding github.com to /root/.ssh/known_hosts"
    ssh-keyscan -t rsa,ecdsa,ed25519 github.com >> /root/.ssh/known_hosts 2>/dev/null
fi

echo "==> Cloning $REPO_URL ($BRANCH) into $WORK_DIR"
git clone --branch "$BRANCH" "$REPO_URL" "$WORK_DIR/repo"

# setup.sh is interactive and reads prompts from /dev/tty, which works fine
# under `curl | bash` because /dev/tty is the controlling terminal even when
# stdin is the pipe.
bash "$WORK_DIR/repo/deploy/setup.sh"
