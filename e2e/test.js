const nearAPI = require('near-api-js');
const config = require('./config/config')
const nearConfig = require('./config/nearConfig')

let account;

beforeAll(async () => {
    const keyPair = nearAPI.utils.KeyPair.fromString(config.clientPrivateKey)
    const keyStore = new nearAPI.keyStores.InMemoryKeyStore()

    keyStore.setKey(nearConfig.networkId, config.clientId, keyPair)
    
    const connect = await nearAPI.connect({
        deps: {
            keyStore,
        },
        ...nearConfig,
    });

    account = await connect.account(config.clientId)
});

it('test owner_id', async () => {
    const owner = await account.viewFunction(nearConfig.contractName, 'get_owner');

    expect(owner).toEqual(nearConfig.contractName);
});

it('test turn right', async () => {
    const initial_direction = await account.viewFunction(nearConfig.contractName, 'direction');

    expect(initial_direction).toEqual("South");

    await account.functionCall({
        methodName: 'turn_right',
        contractId: nearConfig.contractName
    })

    const direction = await account.viewFunction(nearConfig.contractName, 'direction');

    expect(direction).toEqual("West");
});