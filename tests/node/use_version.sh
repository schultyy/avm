#!/bin/bash

cargo run install node 4.1.2
cargo run use node 4.1.2

if [ $? -ne 0 ]; then
  echo "Didn't exit with status code 0"
  rm -rf ~/.avm/
  exit 1
fi

result=$(readlink ~/.avm/node/bin)
if [ $? -ne 0 ]
then
  echo "Link to version 4.1.2 does not exist"
  rm -rf ~/.avm/
  exit 1
fi

rm -rf ~/.avm/
