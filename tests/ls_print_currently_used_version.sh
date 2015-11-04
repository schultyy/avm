#!/bin/bash

cargo run install 4.1.2
cargo run install 5.0.0
cargo run use 4.1.2
cargo run ls | grep "=> 4.1.2"

if [ $? -eq 0 ]
then
  echo "Prints currently used version"
  rm -rf ~/.avm/
else
  echo "Does not print currently used version"
  rm -rf ~/.avm/
  exit 1
fi
