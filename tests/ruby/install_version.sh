#!/bin/bash

cargo run install ruby 2.3.0

if [ $? -ne 0 ]; then
  echo "Compilation was not successful"
  rm -rf ~/.avm/
  exit 1
fi
if [ ! -d ~/.avm/ruby/2.3.0/ ]
then
  echo "Version 2.3.0 does not exist"
  rm -rf ~/.avm/
  exit 1
fi

rm -rf ~/.avm/
