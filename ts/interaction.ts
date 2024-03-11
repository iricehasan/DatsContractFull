import { DatsClient } from "/Users/macbookpro/dats/DatsContractFull/ts/Dats.client"; // Replace with the actual path to the file
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice, calculateFee, StdFee } from "@cosmjs/stargate";
import fs from "fs";


// Define the sender's private key

// const privateKey = "your_private_key_here";

// Create a signer object using the private key
// const wallet = await DirectSecp256k1Wallet.fromKey(privateKey);

// const mnemonic

const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix: "juno", 
});

// Initialize a CosmWasm client with the signer
const client = await SigningCosmWasmClient.connectWithSigner("https://juno-testnet-rpc.polkachu.com/", wallet, {
    gasPrice: GasPrice.fromString("0.025ujunox"),
});

// Define the sender's address and the contract address
const [account] = await wallet.getAccounts();
const senderAddress = account.address;
console.log(senderAddress);


const wasm = fs.readFileSync("/Users/macbookpro/dats/DatsContractFull/artifacts/dats_contract_full.wasm");
const result = await client.upload(senderAddress, wasm, "auto");

// instantiate
const codeId = result.codeId;

const instantiateMsg = {
    owner: senderAddress,
  };

//Instantiate the contract
const instantiateResponse = await client.instantiate(senderAddress, codeId, instantiateMsg, "Dats Contract Full", "auto");
console.log(instantiateResponse);


const contractAddress = instantiateResponse.contractAddress;
// const contractAddress = "juno1z7jgc6wpmyahxp3v280v6lj0ya2efdcp26xzclnynmxapqpcchcsucq72k";

// Create an instance of client
const datsClient = new DatsClient(client, senderAddress, contractAddress);


// execute functions

const save_d_dos = await datsClient.saveDDos({isApprove: true, trafficScale: 5}, "auto");

const save_super_computer = await datsClient.saveSuperComputer({cpuValue: 1, isApprove: true}, "auto");

const save_cyber_security = await datsClient.saveCyberSecurity({isApprove: true, malwareResearch: true, ransomewareResearch: false, serverSecurity: true, webSecurity: false}, "auto");

const save_vulnerability = await datsClient.saveVulnerabilitiy({blockchainPenetration: false, contractPenetration: true, isApprove: true, scadaPenetration: true, serverPenetration: true, webPenetration: true}, "auto")

const save_blockchain = await datsClient.saveBlockchain({approveAttackPrevention: true}, "auto");


// query functions

const queryGetAllUserDdosSettings = await datsClient.getAllUserDdosSettings();
console.log(queryGetAllUserDdosSettings);

const queryGetDdosCount = await datsClient.getDdosCount();
console.log(queryGetDdosCount);

const queryGetDdosByUser = await datsClient.getDdossByUser({user: senderAddress});
console.log(queryGetDdosByUser);


const queryGetAllUserSuperComputerSettings = await datsClient.getAllUserSuperComputerSettings();
console.log(queryGetAllUserSuperComputerSettings);

const queryGetSuperComputerByUser = await datsClient.getSuperComputerByUser({user: senderAddress});
console.log(queryGetSuperComputerByUser);

const queryGetSuperComputerCount = await datsClient.getSuperComputerCount();
console.log(queryGetSuperComputerCount);

const queryGetAllUserCyberSecuritySettings = await datsClient.getAllUserCyberSecuritySettings();
console.log(queryGetAllUserCyberSecuritySettings);

const queryGetCyberSecurityByUser = await datsClient.getCyberSecurityByUser({user: senderAddress});
console.log(queryGetCyberSecurityByUser);

const queryGetCyberSecurityCount = await datsClient.getCyberSecurityCount();
console.log(queryGetCyberSecurityCount);

const queryGetAllUserVulnerabilitySettings = await datsClient.getAllUserVulnerabilitySettings();
console.log(queryGetAllUserVulnerabilitySettings);

const queryGetVulnerabilityByUser = await datsClient.getVulnerabilityByUser({user: senderAddress});
console.log(queryGetVulnerabilityByUser);

const queryGetVulnerabilityCount = await datsClient.getVulnerabilitiyCount();
console.log(queryGetVulnerabilityCount);

const queryGetBlockchainByUser = await datsClient.getBlockchainByUser({user: senderAddress});
console.log(queryGetBlockchainByUser);

const queryGetBlockchainCount = await datsClient.getBlockchainCount();
console.log(queryGetBlockchainCount);