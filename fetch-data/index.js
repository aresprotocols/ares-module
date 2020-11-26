const { ApiPromise, Keyring, WsProvider } = require('@polkadot/api');
const { cryptoWaitReady }  = require('@polkadot/util-crypto');

const PHRASE = 'entire material egg meadow latin bargain dutch coral blood melt acoustic thought';

async function fundOperatorAccountIfNeeded(api, aliceAccount, operatorAccount) {
  return new Promise(async (resolve) => {

    let { data: { free: previousFree }, nonce: previousNonce } = await api.query.system.account(operatorAccount.address);

    if (previousFree.isZero()) {
        await api.tx.balances.transfer(operatorAccount.address, 130000000).signAndSend(aliceAccount, async ({ status }) => {
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

    const result = await api.query.templateModule.result();
    console.log(`Result is currently ${result}`);

    // Listen for ares.OracleRequest events
    api.query.system.events((events) => {
        events.forEach(({ event })  => {
          //console.log(`\t${event.section}:${event.method}`);
          if (event.section == "aresModule" && event.method == "OracleRequest") {
            const id = event.data[2].toString();
            const value = Math.floor(Math.random() * Math.floor(100));
            const result = api.createType('i128', value).toHex(true);
            // Respond to the request with a dummy result
            api.tx.aresModule.callback(parseInt(id), result).signAndSend(operatorAccount, async ({ events = [], status }) => {
                if (status.isFinalized) {
                  const updatedResult = await api.query.example.result();
                  console.log(`Result is now ${updatedResult}`);
                  process.exit();
                }
              });
            console.log(`Operator answered to request ${id} with ${value}`);
        }
      });
    });


    // Then simulate a call from alice
    await api.tx.templateModule.sendRequest(operatorAccount.address, "").signAndSend(aliceAccount);
    console.log(`Request sent`);
}

main().catch(console.error)
