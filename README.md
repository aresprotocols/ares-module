# ares-module
It's the pallets repo for Ares Protocol 

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

### aggregate-ares is ares data warehouse 
  * it fetch huobi and binance and okex price
  * privide api for oracle visite and use. 
