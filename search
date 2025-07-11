#!/usr/bin/env bash
set -euo pipefail

: "${SEARCH_SESSION_TOKEN:=here_the_token}" # Replace here_the_token with your LinkedIn session token
export SEARCH_SESSION_TOKEN

if ! command -v cargo >/dev/null; then
  echo "Error: cargo is not installed. Please install Rust's cargo tool." >&2
  exit 1
fi

OTW=false
args=()
while (( "$#" )); do
  case "$1" in
    -otw|--open-to-work)
      OTW=true
      shift
      ;;
    *)
      args+=("$1")
      shift
      ;;
  esac
done

exec cargo run -q --release "${args[@]:-}" "$OTW"