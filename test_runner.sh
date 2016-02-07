#!/bin/bash

for file in tests/**/*.sh
do
  echo "#### Running $file ####"
  ./$file
done
