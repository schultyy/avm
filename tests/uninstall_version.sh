#!/bin/bash

cargo run install 0.12.0
cargo run use 0.12.0
cargo run uninstall 0.12.0
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
