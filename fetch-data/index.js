const { ApiPromise, Keyring, WsProvider } = require('@polkadot/api');
const { cryptoWaitReady }  = require('@polkadot/util-crypto');

const PHRASE = 'entire material egg meadow latin bargain dutch coral blood melt acoustic thought';

async function fundOperatorAccountIfNeeded(api, aliceAccount, operatorAccount) {
    return new Promise(async (resolve) => {

        let { data: { free: previousFree }, nonce: previousNonce } = await api.query.system.account(operatorAccount.address);

        if (previousFree.isZero()) {
            await api.tx.balances.transfer(operatorAccount.address, 1000000000000000).signAndSend(aliceAccount, async ({ status }) => {
                if (status.isFinalized) {
                    resolve();
                }
            });
            console.log('Operator funded');
        } else {
            resolve();
        }
    });
}

async function registerOperatorIfNeeded(api, operatorAccount) {
    // Register the operator, this is supposed to be initiated once by the operator itself
    return new Promise(async (resolve) => {
        const operator = await api.query.aresModule.operators(operatorAccount.address);
        if(operator.isFalse) {
            await api.tx.aresModule.registerOperator().signAndSend(operatorAccount, async ({ status }) => {
                if (status.isFinalized) {
                    console.log('Operator registered');
                    resolve();
                }
            });
        } else {
            resolve();
        }
    });
}
function sentRes(url,data,method,fn){
    data=data||null;
    if(data==null){
        var content=require('querystring').stringify(data);
    }else{
        var content = JSON.stringify(data); //json format
    }

    var parse_u=require('url').parse(url,true);
    var isHttp=parse_u.protocol=='http:';
    var options={
        host:parse_u.hostname,
        port:parse_u.port||(isHttp?80:443),
        path:parse_u.path,
        method:method,
        headers:{
            'Content-Type':'application/json',
            'Content-Length':Buffer.byteLength(content,"utf8"),
            'Tracking-Api-Key':'YOUR API KEY'
        }
    };
    var req = require(isHttp?'http':'https').request(options,function(res){
        var _data='';
        res.on('data', function(chunk){
            _data += chunk;
        });
        res.on('end', function(){
            fn!=undefined && fn(_data);
        });
    });
    req.write(content);
    req.end();
}

async function main() {
    await cryptoWaitReady();

    // Connect to the local chain
    const wsProvider = new WsProvider('ws://localhost:9944');
    const api = await ApiPromise.create({
        provider: wsProvider,
        types: {
            SpecIndex: "Vec<u8>",
            RequestIdentifier: "u64",
            DataVersion: "u64"
        }
    });


    const Bytes2HexString =(arr)=> {
        for(j = 0; j < arr.length; j++) {
            if(arr[j].indexOf("SpecIndex")>-1)
            {
                var str=arr[j].split(":")[1].trim()

                var curCharCode;
                var resultStr = [];
                // var i=0;

                for(var i = 2; i < str.length;i = i + 2) {
                    curCharCode = parseInt(str.substr(i, 2), 16); // ASCII Code Value
                    resultStr.push(String.fromCharCode(curCharCode));
                }
                return resultStr.join("");

            }
        }
        return "error";
    }



    // Add an account, straight from mnemonic
    const keyring = new Keyring({ type: 'sr25519' });
    const operatorAccount = keyring.addFromUri(PHRASE);
    console.log(`Imported operator with address ${operatorAccount.address}`);

    // Make sure this operator has some funds
    const aliceAccount = keyring.addFromUri('//Alice');
    console.log(`alice ${aliceAccount.address}`);

    await fundOperatorAccountIfNeeded(api, aliceAccount, operatorAccount);

    const result = await api.query.aresModule.oracleResults("btcusdt");
    console.log(`BTCUSDT is currently ${result}`);


    // Listen for ares.OracleRequest events
    api.query.system.events((events) => {
        events.forEach(record => {
            // extract the phase, event and the event types
            const { event, phase} = record;
            const types = event.typeDef;

            // show what we are busy with
            const eventName = `${event.section}:${
                event.method
            }:: (phase=${phase.toString()})`;

            if (event.section == "aresModule" && event.method == "OracleRequest") {
                // loop through each of the parameters, displaying the type and data
                const params = event.data.map(
                    (data, index) => `${types[index].type}: ${data.toString()}`
                );

                console.log(`${params}`);
                console.log(eventName);

                const id = event.data[2].toString();
                var postData = null;
                //get aggregator url and loading
                var url      =`http://141.164.45.97:8080/ares/api/getPartyPrice/${Bytes2HexString(params)}`;
                sentRes(url,postData,"GET",function(data){
                    var price=JSON.parse(data).data.price;
                    const result = api.createType('i128', price).toHex();
                    // Respond to the request with a dummy result
                    api.tx.aresModule.callback(parseInt(id), result).signAndSend(operatorAccount, async ({ events = [], status }) => {
                        if (status.isFinalized) {
                            const updatedResult = await api.query.aresModule.oracleResults("btcusdt");
                            console.log(`Result is now ${updatedResult}`);
                            //process.exit();
                        }
                    });
                    console.log(data);
                    console.log(`Operator answered to request ${id} with ${price}`);
                });
            }

        });
    });

    await registerOperatorIfNeeded(api, operatorAccount);


    // Then simulate a call from alice
    await api.tx.aresModule.initiateRequest(operatorAccount.address, "btcusdt", "1", "").signAndSend(aliceAccount);
    console.log(`Request sent`);
}

main().catch(console.error)
