#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
FRONTEND_DIR="$ROOT_DIR/frontend"
STATIC_DIR="$ROOT_DIR/static"

echo "Building frontend using direnv..."


cd "$FRONTEND_DIR"
direnv exec . bash -c '
  echo "[frontend] Installing dependencies..."
  [ -d node_modules ] || npm install

  echo "[frontend] Building frontend..."
  npm run build
'

cd "$ROOT_DIR"

echo "[backend] Updating static directory..."
rm -rf "$STATIC_DIR"
mkdir -p "$STATIC_DIR"
cp -r "$FRONTEND_DIR/dist/"* "$STATIC_DIR/"

echo "Done."
