#!/bin/bash

echo '# start'

mkdir task_output
mkdir puzzle_output
mkdir task_err_output
mkdir puzzle_err_output



for x in ./task_programs/*; do
  if [[ -f $x/main ]]; then
    name=$(basename $x)
    mkdir task_output/$name
    timeout -sKILL 900 $x/main ./task.desc task_output/$name/output > task_err_output/${name}.out 2>task_err_output/${name}.err &
  fi
done

for x in ./puzzle_programs/*; do
  if [[ -f $x/main ]]; then
    name=$(basename $x)
    timeout -sKILL 900 $x/main ./puzzle.cond puzzle_output/$name > puzzle_err_output/${name}.out 2>puzzle_err_output/${name}.err &
  fi
done

