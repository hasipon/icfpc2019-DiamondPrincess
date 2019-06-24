#!/bin/bash

echo '# start'
touch task.sol
touch puzzle.desc

for i in task_output/*
do
    if [ -d "$i" ]
    then
        /home/ubuntu/bin/run-checker ./task.desc $i/output > $i/score
        echo "$i:`cat $i/score`"
    fi
done

python3 prepare_submit.py
