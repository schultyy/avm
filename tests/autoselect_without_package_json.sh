#!/bin/bash

cargo run autoselect

if [ $? -ne 0 ]; then
  echo "Should exit with status code != 0"
  exit 1
fi
