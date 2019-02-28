#!/bin/bash
set -e

BUILD_DIR=./build/
SRC_DIR=./src/
DOCKER_TAG="rustythingy"

function task_build {
    mkdir -p ${BUILD_DIR}/src
    cp -r ${SRC_DIR}/* ${BUILD_DIR}/src
    cp ./Cargo.toml ${BUILD_DIR}
}

function task_build_compiler_image {
    docker build -t ${DOCKER_TAG} ${BUILD_DIR}
}

function task_build_compiler_container {
    docker run --name ${DOCKER_TAG} -dt ${DOCKER_TAG}
}

function task_compile {
    docker exec -e CARGO_INCREMENTAL=1 ${DOCKER_TAG} cargo build --release --target armv7-unknown-linux-gnueabihf
}

function task_copy_binaries {
    docker cp ${DOCKER_TAG}:/target/armv7-unknown-linux-gnueabihf/release/rustythingy ${BUILD_DIR}
}

function task_delete_compiler_container {
    docker stop ${DOCKER_TAG}
    docker rm ${DOCKER_TAG}
}

function task_usage {
  echo "Usage: $0 build | build_compiler_image | build_compiler_container | compile | copy_binaries | delete_compiler_container"
}

CMD=${1:-}
shift || true
case ${CMD} in
  build) task_build ;;
  build_compiler_image) task_build_compiler_image ;;
  build_compiler_container) task_build_compiler_container ;;
  compile) task_compile ;;
  copy_binaries) task_copy_binaries ;;
  delete_compiler_container) task_delete_compiler_container ;;
  *) task_usage ;;
esac