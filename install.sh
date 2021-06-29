#!/bin/bash

cargo build --release
cp ./target/release/bloodstarve $1/py2json