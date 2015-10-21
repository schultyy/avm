#!/bin/bash

cargo run install 0.12.0
cargo run use 0.12.0
cargo run uninstall 0.12.0
result=$(readlink ~/.avm/node)
if [ $? -eq 1 ]
then
  echo "Symlink node removed"
else
  echo "Symlink node still exists"
  exit 1
fi

result=$(readlink ~/.avm/npm)
if [ $? -eq 1 ]
then
  echo "Symlink npm removed"
else
  echo "Symlink npm still exists"
  exit 1
fi
