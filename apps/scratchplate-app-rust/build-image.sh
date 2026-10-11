#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

IMAGE_NAME="${1:-scratchplate-app-rust:0.1.0}"
SKIP_TESTS="${SKIP_TESTS:-false}"

if [[ "${SKIP_TESTS}" != "true" ]]; then
  docker build --target test -t scratchplate-app-rust:test .
fi

docker build --target runtime -t "${IMAGE_NAME}" .
