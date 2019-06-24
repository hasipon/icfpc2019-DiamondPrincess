import sys
import json
from time import time

blockchaininfo = json.loads(sys.stdin.read())
with open('../block') as f:
    block = f.read()

if block != str(blockchaininfo['block']):
    with open('../block', 'w') as f:
        f.write(str(blockchaininfo['block']))
    with open('../update_block', 'w') as f:
        pass

if time() - blockchaininfo['block_ts'] > 850:
    with open('../submit', 'w') as f:
        f.write(str(blockchaininfo['block']))
