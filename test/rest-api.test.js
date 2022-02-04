require("regenerator-runtime/runtime");

const config = require('./config');
const contract = require('./rest-api-test-utils');
const utils = require('./utils');
const path = require('path');
const fs = require('fs').promises;

async function getAddress(accountName) {
    const file = path.join(config.ACCOUNTS_DIR, accountName);
    const address = await fs.readFile(file, 'utf-8');
    return address
}

let { near, root_contract_id } = {};



describe("Deploy contract", () => {

    beforeAll(async () => {
        root_contract_id = await getAddress("root");
        near = new contract(root_contract_id);
        // const contractName = await near.deploy("dtoken.wasm");
        // console.log({contractName});
    })

    test("Parameter root_contract_id installed", async () => {
        expect(root_contract_id).not.toBe(undefined);
    });

    test("Contract was successfully installed", async () => {
        expect(near).toMatchObject(new contract(root_contract_id));
    });

})

describe("Dtoken common methods", () => {
    let { dtoken_contract_id, root_user } = {};

    beforeAll(async () => {
        dtoken_contract_id = await getAddress("dtoken");
        root_user = await getAddress("root");
        dtoken_near = new contract(dtoken_contract_id);
    })

    test("Total_supplies: [get_total_supplies, set_total_supplies]", async () => {

        const total_supplies = await dtoken_near.view("get_total_supplies", {},
            {account_id: dtoken_contract_id });

        expect(total_supplies).toBeGreaterThanOrEqual(0);

        await dtoken_near.call("set_total_supplies", {amount: total_supplies + 1 },
            {account_id: dtoken_contract_id });

        const new_total_supplies = await dtoken_near.view("get_total_supplies", {});

        expect(new_total_supplies).toBeGreaterThanOrEqual(0);
        expect(total_supplies).toBeLessThan(new_total_supplies);
        expect(total_supplies).toBe(new_total_supplies - 1);
    });

    test("[Negative value test]: Total_supplies", async () => {
        const NegativeValue = await dtoken_near.call("set_total_supplies", { amount: -1 },
            {account_id: dtoken_contract_id });

        expect(NegativeValue.type).toBe('FunctionCallError');
    })

    test("[Private method test]: Total_supplies", async () => {
        const PrivateMethodCheck = await dtoken_near.call("set_total_supplies", { amount: 10 },
            {account_id: root_user });

        expect(PrivateMethodCheck.type).toBe('FunctionCallError');
    })

    test("Total_borrows: [get_total_borrows, set_total_borrows]", async () => {

        const total_borrows = await dtoken_near.view("get_total_borrows", {},
            {account_id: dtoken_contract_id });

        expect(total_borrows).toBeGreaterThanOrEqual(0);

        await dtoken_near.call("set_total_borrows", {amount: total_borrows + 1 },
            {account_id: dtoken_contract_id });

        const new_total_borrows = await dtoken_near.view("get_total_borrows", {});

        expect(new_total_borrows).toBeGreaterThanOrEqual(0);
        expect(total_borrows).toBeLessThan(new_total_borrows);
        expect(total_borrows).toBe(new_total_borrows - 1);
    });

    test("[Negative value test]: Total_borrows", async () => {
        const NegativeValue = await dtoken_near.call("set_total_borrows", { amount: -1 },
            {account_id: dtoken_contract_id });

        expect(NegativeValue.type).toBe('FunctionCallError');
    })

    test("[Private method test]: Total_borrows", async () => {
        const PrivateMethodCheck = await dtoken_near.call("set_total_borrows", { amount: 50 },
            {account_id: root_user });

        expect(PrivateMethodCheck.type).toBe('FunctionCallError');
    })
    

})