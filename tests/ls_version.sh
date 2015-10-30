#!/bin/bash
# The ls command is not supposed to print any other directories
# then version directories

mkdir -p ~/.avm/bin

cargo run ls | grep bin

if [ $? -eq 0 ]
then
  echo "ls printed out bin directory"
  rm -rf ~/.avm/
  exit 1
fi
rm -rf ~/.avm/
