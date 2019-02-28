#!/bin/bash
set -e

BUILD_DIR=./build/
SRC_DIR=./src/

function task_build {

    mkdir  ${BUILD_DIR}/src
    cp -r ${SRC_DIR}/* ${BUILD_DIR}/src
    cp ./Cargo.toml ${BUILD_DIR}
}

function task_usage {
  echo "Usage: $0 build"
}

CMD=${1:-}
shift || true
case ${CMD} in
  build) task_build ;;
  *) task_usage ;;
esac