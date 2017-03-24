#!/bin/bash

cargo run install node 6.10.2
cargo run use node 6.10.2
cargo run uninstall node 6.10.2
result=$(readlink ~/.avm/node/bin)
if [ $? -eq 1 ]
then
  echo "Symlink to bin directory removed"
else
  echo "Symlink to bin directory still exists"
  rm -rf ~/.avm/
  exit 1
fi

rm -rf ~/.avm/
