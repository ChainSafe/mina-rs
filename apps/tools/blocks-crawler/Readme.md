# Block Crawler

A debugging tool which can be used to filter out blocks from mina google cloud storage mainnet dump which are not compatible with mina-rs json serde.

## Workflow:

### Step 1: 
Fetches blocks info from https://graphql.minaexplorer.com/ using graphql api query.

**Sample Query:**
```
query Fetch {
  blocks(limit: 1, sortBy: RECEIVEDTIME_DESC) {
    blockHeight
    stateHash
  }
}
```
**Sample Response:**
```
{
  "data": {
    "blocks": [
      {
        "blockHeight": 147571,
        "stateHash": "3NKwrze6FvGQCCF6L7Q2JLvwgnsm56hwSny9kUyjbSUr8oqu1MGp"
      }
    ]
  }
}
```

### Step 2: 
Use blockHeight and stateHash to download block json from mina google cloud storage

**Sample url**: https://storage.googleapis.com/mina_network_block_data/mainnet-147571-3NKwrze6FvGQCCF6L7Q2JLvwgnsm56hwSny9kUyjbSUr8oqu1MGp.json


### Step 3: 
Parse downloaded block json to **ExternalTransition** type

If parsing block json fails appropriate error message along with block url will be written to terminal
```
Fix Me(https://storage.googleapis.com/mina_network_block_data/mainnet-147571-3NKwrze6FvGQCCF6L7Q2JLvwgnsm56hwSny9kUyjbSUr8oqu1MGp.json): found 'Failed' expected 'Applied'
```

## How to Run: blocks-crawler 

USAGE:
    blocks-crawler [OPTIONS] --limit <LIMIT>

OPTIONS:
    -c, --concurrency <CONCURRENCY>    [default: 8]
    -h, --help                         Print help information
    -l, --limit <LIMIT>                

## How to enable logger:

To enable info level logger: 
```
RUST_LOG=info
```
To enable warn level logger: 
```
RUST_LOG=warn
```
To enable error level logger: 
```
RUST_LOG=error