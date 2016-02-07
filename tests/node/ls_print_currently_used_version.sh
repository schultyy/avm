#!/bin/bash

cargo run install node 4.1.2
cargo run install node 5.0.0
cargo run use node 4.1.2
cargo run ls node | grep "=> 4.1.2"

if [ $? -eq 0 ]
then
  echo "Prints currently used version"
  rm -rf ~/.avm/
else
  echo "Does not print currently used version"
  rm -rf ~/.avm/
  exit 1
fi
