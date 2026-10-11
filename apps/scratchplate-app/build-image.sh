#!/usr/bin/env bash
# Builds the scratchplate-app container image with Jib into the local Docker daemon.
#
# Usage: ./build-image.sh [image]        (default image: scratchplate-app:<project version>)
#
# Requires Maven and a JDK 25+. Tests run as part of `package`; set SKIP_TESTS=true to skip them.

set -euo pipefail

cd "$(dirname "$0")"

SKIP_TESTS=${SKIP_TESTS:-false}
MVN_ARGS=(-B package "-DskipTests=${SKIP_TESTS}")
if [[ $# -gt 0 ]]; then
  MVN_ARGS+=("-Dimage.name=$1")
fi

mvn "${MVN_ARGS[@]}" jib:dockerBuild
