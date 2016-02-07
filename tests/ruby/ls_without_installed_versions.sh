#!/bin/bash
# The ls command is not supposed to print the following when there's
# nothing installed
#
# Listing all installed versions:
# (=>): current version
# =>  (system)

cargo run ls ruby | grep "(=>): current version"

if [ $? -eq 0 ]
then
  echo "Printed system ruby version even it doesn't exist"
  exit 1
fi
