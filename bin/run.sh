#!/bin/bash

convert="python utils/export.py"
drawsolution="python utils/draw.py"

if [ -n "$HEURISTIC_RUN" ]; then
  runheuristic=$HEURISTIC_RUN
else
  runheuristic="./target/release/mor-proj"
fi

execute_staff() {
  file=$1
  filebase=${file%%.*}
  ext=${file#*.}

  if [ $ext = 'm' ]; then
    file=$($convert $file)
  fi

  solution=${filebase}_out.json
  $runheuristic $file -o $solution

  if [ $? -ne 0 ]; then
    return
  fi

  if [ ! -e $solution ]; then
    return
  fi

  $drawsolution $solution
}


for file in $*; do
  execute_staff $file
done
