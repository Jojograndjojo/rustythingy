#!/usr/bin/env bash
./do delete_compiler_container
echo "preparing building dependencies..."
./do build
wait
echo "building compiler image..."
./do build_compiler_image
wait
echo "building compiler container.."
./do build_compiler_container
wait
echo "compiling..."
./do compile
echo "copying binaries..."
wait
./do copy_binaries