#!/bin/bash

cargo run autoselect

if [ $? -eq 0 ]; then
  echo "Should error with exit code != 0 when package.json is not there"
  exit 1
fi
