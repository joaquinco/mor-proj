#!/bin/bash


convert="python utils/export.py"
runheuristic="./target/release/mor-proj"
drawsolution="python utils/draw.py"

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
    echo "Error running metaheuristic"
    return
  fi

  if [ ! -e $solution ]; then
    exit 0
  fi

  $drawsolution $solution
}


for file in $*; do
  execute_staff $file
done
