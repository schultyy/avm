#!/bin/bash

cargo run install node 6.10.2

if [ $? -ne 0 ]; then
  echo "Compilation was not successful"
  rm -rf ~/.avm/
  exit 1
fi
if [ ! -d ~/.avm/node/6.10.2/ ]
then
  echo "Version 6.10.2 does not exist"
  rm -rf ~/.avm/
  exit 1
fi

rm -rf ~/.avm/
