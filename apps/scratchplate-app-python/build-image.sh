#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

VERSION="$(< VERSION)"
DEFAULT_IMAGE="scratchplate-app-python:${VERSION}"
IMAGE_NAME="${1:-$DEFAULT_IMAGE}"
SKIP_TESTS="${SKIP_TESTS:-false}"

if [[ "$SKIP_TESTS" != "true" ]]; then
  docker build --target test -t scratchplate-app-python-test:${VERSION} .
fi

docker build \
  --build-arg APP_VERSION="$VERSION" \
  --target final \
  -t "$IMAGE_NAME" \
  .
