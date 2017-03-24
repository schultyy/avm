#!/bin/bash

cargo run install node 6.10.2
cargo run use node 6.10.2
old_npm_version=$(npm -v)
npm install -g npm
new_npm_version=$(npm -v)

if [ new_npm_version == old_npm_version ]
then
  echo "fatal: did not successfully upgrade npm"
  rm -rf ~/.avm
  exit 1
else
  echo "Upgraded npm successfully"
  rm -rf ~/.avm
fi
