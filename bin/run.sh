#!/bin/bash

convert="python utils/export.py"
drawsolution="python utils/draw.py"
validate="python utils/validate.py"

if [ -n "$HEURISTIC_RUN" ]; then
  runheuristic=$HEURISTIC_RUN
else
  runheuristic="./target/release/mor-proj"
fi

if [ -n "$HEURISTIC_CONFIG" ]; then
  config="-c $HEURISTIC_CONFIG"
else
  config=""
fi

execute_staff() {
  file=$1
  filebase=${file%%.*}
  ext=${file#*.}

  if [ $ext = 'm' ]; then
    file=$($convert $file)
  fi

  solution=${filebase}_out.json
  $runheuristic $file -o $solution $config

  if [ $? -ne 0 ]; then
    return
  fi

  if [ ! -e $solution ]; then
    return
  fi

  $drawsolution $solution
  $validate $solution
}


for file in $*; do
  execute_staff $file
done
