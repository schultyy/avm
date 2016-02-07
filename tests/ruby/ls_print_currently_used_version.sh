#!/bin/bash

cargo run install ruby 2.2.0
cargo run install ruby 2.3.0
cargo run use ruby 2.2.0
cargo run ls ruby | grep "=> 2.2.0"

if [ $? -eq 0 ]
then
  echo "Prints currently used version"
  rm -rf ~/.avm/
else
  echo "Does not print currently used version"
  rm -rf ~/.avm/
  exit 1
fi
