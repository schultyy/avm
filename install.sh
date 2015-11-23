#!/bin/bash

function has_rust {
  which cargo > /dev/null
  if [ $? -eq 1 ]
  then
    >&2 echo "fatal: Rust is not installed"
    >&2 echo "Please visit https://www.rust-lang.org/downloads.html to install Rust"
    exit 1
  fi
}

function has_git {
  which git > /dev/null
  if [ $? -eq 1 ]
  then
    >&2 echo "fatal: git is not installed"
    exit 1
  fi
}

function create_folder {
  mkdir -p ~/.avm/
}

function download_source {
  git clone git@github.com:schultyy/avm.git /tmp/avm
}

function cleanup {
  echo "Cleaning up..."
  rm -rf /tmp/avm
}

function compile_avm {
  cd /tmp/avm/
  cargo build --release
  if [ $? -ne 0 ]
  then
    >&2 echo "fatal: compilation failed"
    cleanup
    >&2 echo "fatal: exiting"
    exit 1
  fi
  sudo cp target/release/avm /usr/local/bin/avm
  if [ $? -ne 0 ]
  then
    >&2 echo "fatal: Could copy binary to /usr/local/bin"
    cleanup
    >&2 echo "fatal: exiting"
    exit 1
  fi
}

echo "Installing avm"
has_rust
has_git
create_folder
download_source
compile_avm
cleanup

echo "Installation finished"
echo "Add this 'export PATH=~/.avm/:~/.avm/bin:\$PATH' to your bash configuration file"
