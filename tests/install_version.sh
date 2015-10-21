#!/bin/bash

cargo run install 4.1.2

if [ $? -ne 0 ]; then
  echo "Didn't exit with status code 0"
  rm -rf ~/.avm/
  exit 1
fi
if [ ! -d ~/.avm/4.1.2/ ]
then
  echo "Version 4.1.2 does not exist"
  rm -rf ~/.avm/
  exit 1
fi

rm -rf ~/.avm/
