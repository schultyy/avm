#!/bin/bash
# The ls command is not supposed to print the following when there's
# nothing installed
#
# Listing all installed versions:
# (=>): current version
# =>  (system)

cargo run ls | grep "=>  (system)"

if [ $? -eq 0 ]
then
  echo "Printed system node version even it doesn't exist"
  exit 1
fi
