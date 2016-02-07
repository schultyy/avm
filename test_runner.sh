#!/bin/bash

for file in tests/node/*.sh
do
  echo "#### Running $file ####"
  ./$file
done
