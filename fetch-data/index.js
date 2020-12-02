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
              const value = Math.floor(Math.random() * Math.floor(100));
              const result = api.createType('i128', value).toHex();
              // Respond to the request with a dummy result
              api.tx.aresModule.callback(parseInt(id), result).signAndSend(operatorAccount, async ({ events = [], status }) => {
                  if (status.isFinalized) {
                    const updatedResult = await api.query.aresModule.oracleResults("btcusdt");
                    console.log(`Result is now ${updatedResult}`);
                    //process.exit();
                  }
                });
              console.log(`Operator answered to request ${id} with ${value}`);
          }
      });
    });

    await registerOperatorIfNeeded(api, operatorAccount);


    // Then simulate a call from alice
    await api.tx.aresModule.initiateRequest(operatorAccount.address, "btcusdt", "1", "").signAndSend(aliceAccount);
    console.log(`Request sent`);
}

main().catch(console.error)
