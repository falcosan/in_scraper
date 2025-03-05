#!/usr/bin/env bash

set -euo pipefail

echo "Starting dependency update process..."

if ! command -v cargo &> /dev/null; then
  echo "Error: cargo is not installed or not in PATH" >&2
  exit 1
fi

echo "Running cargo upgrade..."
if ! cargo upgrade -i allow; then
  echo "Error: cargo upgrade failed" >&2
  exit 1
fi

echo "Running cargo update..."
if cargo update; then
  echo "Dependency update completed successfully"
else
  echo "Error: cargo update failed" >&2
  exit 1
fi