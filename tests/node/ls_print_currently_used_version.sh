#!/bin/bash

cargo run install node 6.10.2
cargo run install node 7.7.4
cargo run use node 6.10.2
cargo run ls node | grep "=> 6.10.2"

if [ $? -eq 0 ]
then
  echo "Prints currently used version"
  rm -rf ~/.avm/
else
  echo "Does not print currently used version"
  rm -rf ~/.avm/
  exit 1
fi
