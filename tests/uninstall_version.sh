#!/bin/bash

cargo run install node 0.12.0
cargo run use node 0.12.0
cargo run uninstall node 0.12.0
result=$(readlink ~/.avm/bin)
if [ $? -eq 1 ]
then
  echo "Symlink to bin directory removed"
else
  echo "Symlink to bin directory still exists"
  rm -rf ~/.avm/
  exit 1
fi

rm -rf ~/.avm/
