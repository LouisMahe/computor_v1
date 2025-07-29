#!/bin/bash

docker build -t rust-builder .

docker create --name temp-builder rust-builder

docker cp temp-builder:/app/target/release/computor_v1 ./computor_v1

docker rm temp-builder
