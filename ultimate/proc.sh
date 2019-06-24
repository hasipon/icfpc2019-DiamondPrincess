#!/bin/bash -x

date
cd lambda-client
python3 lambda-cli.py getblockchaininfo | python3 ../proc.py
# echo '{"block": 0, "block_subs": 0, "block_ts": 0.0, "total_subs": 0}' | python3 ../proc.py

cd ..
if [[ -e ./update_block ]]; then
  block_id=$(cat ./block)
  if [[ ! -e ./lambda-client/blocks/$block_id/.done ]]; then	
    exit	
  fi
  echo '# update'
  rm ./update_block
  mkdir ./workspace/$block_id
  cp ./scripts/* ./workspace/$block_id
  cp ./lambda-client/blocks/$block_id/* ./workspace/$block_id
  rsync -av ./task_programs ./workspace/$block_id
  rsync -av ./puzzle_programs ./workspace/$block_id
  cd ./workspace/$block_id
  ./start.sh

elif [[ -e ./submit ]]; then
  if  [[ -e ./submitted ]]; then
    if [[ $(cat ./submit) = $(cat ./submitted) ]]; then
      exit
    fi
  fi

  echo '# submit'
  mv ./submit ./submitted
  block_id=$(cat ./block)
  cd ./workspace/$block_id
  ./prepare_submit.sh
  cd ../../lambda-client
  python3 lambda-cli.py submit $block_id ../workspace/$block_id/task.sol ../workspace/$block_id/puzzle.desc
fi
