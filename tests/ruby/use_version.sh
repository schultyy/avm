#!/bin/bash

cargo run install ruby 2.3.0
cargo run use ruby 2.3.0

if [ $? -ne 0 ]; then
  echo "Didn't exit with status code 0"
  rm -rf ~/.avm/
  exit 1
fi

result=$(readlink ~/.avm/ruby/bin)
if [ $? -ne 0 ]
then
  echo "Link to version 2.3.0 does not exist"
  rm -rf ~/.avm/
  exit 1
fi

rm -rf ~/.avm/
