#!/bin/bash

cargo run install ruby 2.3.0
cargo run use ruby 2.3.0
cargo run uninstall ruby 2.3.0
result=$(readlink ~/.avm/ruby/bin)
if [ $? -eq 1 ]
then
  echo "Symlink to bin directory removed"
else
  echo "Symlink to bin directory still exists"
  rm -rf ~/.avm/
  exit 1
fi

rm -rf ~/.avm/
