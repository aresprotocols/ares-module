# ares-module
It's the pallets repo for Ares Protocol 

[`Example Vedio`](https://www.google.com/sorry/index?continue=https://www.youtube.com/watch%3Fv%3DHlYhsHFKzJw&q=EgQvOaYCGIXIrP8FIhkA8aeDS0MbBewQM_cNNJgAufzC6Rm8niLWMgFy)

### substrate-node-template is substrate node
   *  `pallet-ares` deal aggregator register and unregister
   *  `pallet-ares` oracle request and result callback 
   *   add `pallet-ares` test code
   
### substrate-front-end-template 
  * the front end displays the registration and results of events on the chain
  * query the oracle price and data warehouse price

### fetch-data is ares oracle scanner
  * listen for Oracle event requests
  * fetch aggregate price and return to oracle
  
  [Learn More](https://github.com/aresprotocols/ares-module/tree/main/fetch-data)


### aggregate-ares is ares data warehouse 
  * it fetch huobi and binance and okex price
  * privide api for oracle visite and use. 

[Learn More](https://github.com/aresprotocols/ares-module/tree/main/aggregate-ares)

#### getPrice
Suppot `BTC`,`ETH`, `DOT`

http://141.164.45.97:8080/ares/api/getPartyPrice/btcusdt
```
{"msg":"success","code":0,"data":{"market":null,"symbol":"btcusdt","price":18319.72,"nodes":null,"sn":null,"systs":1607528442761,"ts":1607528442761}}
```

## Build

### 1. Start Node
Enter `substrate-node-template`
```
make build
```
then start
```
./target/release/node-template --dev --tmp
```

### 2. Start Front
Enter `substrate-front-end-template`
run
```
yarn start
```

### 3. Start Aggregator
Enter `fetch-data` run
```
npm index.js
```

you can use `Start Front` send `register`, `unregister`,`initial_request`,`feed_data` action with `node-template`
