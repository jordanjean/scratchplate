#!/usr/bin/env bash
# Builds the scratchplate-app container image with Jib and makes it available in the local Docker daemon.
#
# Usage: ./build-image.sh [image]        (default image: scratchplate-app:<project version>)
#
# - With a local JDK 25+: runs `./mvnw package jib:dockerBuild`.
# - Without it: runs Maven/Jib inside a JDK 25 container (`jib:buildTar`) and `docker load`s the tarball.
# Tests run as part of `package`; set SKIP_TESTS=true to skip them.

set -euo pipefail

cd "$(dirname "$0")"

MAVEN_IMAGE=${MAVEN_IMAGE:-maven:3.9-eclipse-temurin-25}
SKIP_TESTS=${SKIP_TESTS:-false}
MVN_ARGS=(-B package "-DskipTests=${SKIP_TESTS}")
if [[ $# -gt 0 ]]; then
  MVN_ARGS+=("-Dimage.name=$1")
fi

has_jdk() {
  command -v javac >/dev/null 2>&1 || return 1
  local version
  version=$(javac -version 2>&1 | grep -oE '[0-9]+' | head -1)
  [[ ${version:-0} -ge 25 ]]
}

if has_jdk; then
  echo ">> Building with local JDK (jib:dockerBuild)"
  ./mvnw "${MVN_ARGS[@]}" jib:dockerBuild
else
  echo ">> No local JDK 25 found, building inside ${MAVEN_IMAGE} (jib:buildTar)"
  mkdir -p "${HOME}/.m2"
  docker run --rm \
    --user "$(id -u):$(id -g)" \
    -e HOME=/tmp \
    -e MAVEN_OPTS="-Duser.home=/tmp" \
    -v "${PWD}:/workspace" \
    -v "${HOME}/.m2:/tmp/.m2" \
    -w /workspace \
    "${MAVEN_IMAGE}" \
    ./mvnw "${MVN_ARGS[@]}" jib:buildTar -Djib.baseImageCache=/tmp/.m2/jib-cache
  docker load -i target/jib-image.tar
fi

