# Execute Functions
>> const save_d_dos = await datsClient.saveDDos({isApprove: true, trafficScale: 5}, "auto");
undefined
>> console.log(save_d_dos)
{
  logs: [ { msg_index: 0, log: '', events: [Array] } ],
  height: 8777456,
  transactionHash: 'BF1E952314E57934179F2731F0F1860A3CD01DD5045F8647CB20C03FEA8DF750',
  events: [
    { type: 'coin_spent', attributes: [Array] },
    { type: 'coin_received', attributes: [Array] },
    { type: 'transfer', attributes: [Array] },
    { type: 'message', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'message', attributes: [Array] },
    { type: 'execute', attributes: [Array] },
    { type: 'wasm', attributes: [Array] }
  ],
  gasWanted: 151491n,
  gasUsed: 125820n
}
undefined
>> const save_super_computer = await datsClient.saveSuperComputer({cpuValue: 1, isApprove: true}, "auto");
undefined
>> console.log(save_super_computer)
{
  logs: [ { msg_index: 0, log: '', events: [Array] } ],
  height: 8777458,
  transactionHash: '9BC76AB4B653E05C8BA942BCFAAAAA5CC4AFA1797A07D62365A721315128C4BE',
  events: [
    { type: 'coin_spent', attributes: [Array] },
    { type: 'coin_received', attributes: [Array] },
    { type: 'transfer', attributes: [Array] },
    { type: 'message', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'message', attributes: [Array] },
    { type: 'execute', attributes: [Array] },
    { type: 'wasm', attributes: [Array] }
  ],
  gasWanted: 152173n,
  gasUsed: 126306n
}
undefined
>> const save_cyber_security = await datsClient.saveCyberSecurity({isApprove: true, malwareResearch: true, ransomewareResearch: false, serverSecurity: true, webSecurity: false}, "auto");
undefined
>> console.log(save_cyber_security)
{
  logs: [ { msg_index: 0, log: '', events: [Array] } ],
  height: 8777464,
  transactionHash: '439C56BAB163567477679C4D288497FD95559127131EB57711D8C8981DC2864A',
  events: [
    { type: 'coin_spent', attributes: [Array] },
    { type: 'coin_received', attributes: [Array] },
    { type: 'transfer', attributes: [Array] },
    { type: 'message', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'message', attributes: [Array] },
    { type: 'execute', attributes: [Array] },
    { type: 'wasm', attributes: [Array] }
  ],
  gasWanted: 156887n,
  gasUsed: 129673n
}
undefined
>> const save_vulnerabilitiy = await datsClient.saveVulnerabilitiy({blockchainPenetration: false, contractPenetration: true, isApprove: true, scadaPenetration: true, serverPenetration: true, webPenetration: true}, "auto")
undefined
>> console.log(save_vulnerability)
{
  logs: [ { msg_index: 0, log: '', events: [Array] } ],
  height: 8777465,
  transactionHash: '0FDC02D8BE1E04A66DADF2D4C537AB847461CE1AEB4AA6FA3B4DE589583213D7',
  events: [
    { type: 'coin_spent', attributes: [Array] },
    { type: 'coin_received', attributes: [Array] },
    { type: 'transfer', attributes: [Array] },
    { type: 'message', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'message', attributes: [Array] },
    { type: 'execute', attributes: [Array] },
    { type: 'wasm', attributes: [Array] }
  ],
  gasWanted: 158907n,
  gasUsed: 131116n
}
undefined
>> const save_blockchain = await datsClient.saveBlockchain({approveAttackPrevention: true}, "auto");
undefined
>> console.log(save_blockchain)
{
  logs: [ { msg_index: 0, log: '', events: [Array] } ],
  height: 8777467,
  transactionHash: 'B17FAAE48FE92106A1336B2895B23E5D178119B05FF0D030BA9DD2379DEB49B6',
  events: [
    { type: 'coin_spent', attributes: [Array] },
    { type: 'coin_received', attributes: [Array] },
    { type: 'transfer', attributes: [Array] },
    { type: 'message', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'tx', attributes: [Array] },
    { type: 'message', attributes: [Array] },
    { type: 'execute', attributes: [Array] },
    { type: 'wasm', attributes: [Array] }
  ],
  gasWanted: 151899n,
  gasUsed: 126110n
}
undefined

# Query Functions
>> const queryGetAllUserDdosSettings = await datsClient.getAllUserDdosSettings();
undefined
>> console.log(queryGetAllUserDdosSettings);
{
  ddosses: [
    {
      id: '1',
      user: 'juno1tuu594kmqax6k8crqtjaqrypux3z5aq9e97xx0',
      is_approve: true,
      traffic_scale: 5
    }
  ]
}
undefined
>> const queryGetDdosCount = await datsClient.getDdosCount();
undefined
>> console.log(queryGetDdosCount);
{ ddoslength: 1 }
undefined
>> const queryGetDdosByUser = await datsClient.getDdossByUser({user: senderAddress});
undefined
>> console.log(queryGetDdosByUser);
{
  id: '1',
  user: 'juno1tuu594kmqax6k8crqtjaqrypux3z5aq9e97xx0',
  is_approve: true,
  traffic_scale: 5
}
undefined
>> const queryGetAllUserSuperComputerSettings = await datsClient.getAllUserSuperComputerSettings();
undefined
>> console.log(queryGetAllUserSuperComputerSettings);
{
  supercomputers: [
    {
      id: '1',
      user: 'juno1tuu594kmqax6k8crqtjaqrypux3z5aq9e97xx0',
      is_approve: true,
      cpu_value: 1
    }
  ]
}
undefined
>> const queryGetSuperComputerByUser = await datsClient.getSuperComputerByUser({user: senderAddress});
undefined
>> console.log(queryGetSuperComputerByUser);
{
  id: '1',
  user: 'juno1tuu594kmqax6k8crqtjaqrypux3z5aq9e97xx0',
  is_approve: true,
  cpu_value: 1
}
undefined
>> const queryGetSuperComputerCount = await datsClient.getSuperComputerCount();
undefined
>> console.log(queryGetSuperComputerCount);
{ superlength: 1 }
undefined
>> const queryGetCyberSecurityByUser = await datsClient.getCyberSecurityByUser({user: senderAddress});
undefined
>> console.log(queryGetCyberSecurityByUser);
{
  id: '1',
  user: 'juno1tuu594kmqax6k8crqtjaqrypux3z5aq9e97xx0',
  is_approve: true,
  web_security: false,
  server_security: true,
  ransomeware_research: false,
  malware_research: true
}
undefined
>> const queryGetCyberSecurityCount = await datsClient.getCyberSecurityCount();
undefined
>> console.log(queryGetCyberSecurityCount);
{ cyberlength: 1 }
undefined
>> const queryGetAllUserVulnerabilitySettings = await datsClient.getAllUserVulnerabilitySettings();
undefined
>> console.log(queryGetAllUserVulnerabilitySettings);
{
  vulnerabilities: [
    {
      id: '1',
      user: 'juno1tuu594kmqax6k8crqtjaqrypux3z5aq9e97xx0',
      is_approve: true,
      web_penetration: true,
      server_penetration: true,
      scada_penetration: true,
      blockchain_penetration: false,
      contract_penetration: true
    }
  ]
}
undefined
>> const queryGetVulnerabilityByUser = await datsClient.getVulnerabilityByUser({user: senderAddress});
undefined
>> console.log(queryGetVulnerabilityByUser);
{
  id: '1',
  user: 'juno1tuu594kmqax6k8crqtjaqrypux3z5aq9e97xx0',
  is_approve: true,
  server_penetration: true,
  scada_penetration: true,
  blockchain_penetration: false,
  contract_penetration: true
}
undefined
>> const queryGetVulnerabilityCount = await datsClient.getVulnerabilitiyCount();
undefined
>> console.log(queryGetVulnerabilityCount);
{ vulnerability_length: 1 }
undefined
>> const queryGetBlockchainByUser = await datsClient.getBlockchainByUser({user: senderAddress});
undefined
>> console.log(queryGetBlockchainByUser);
{
  id: '1',
  user: 'juno1tuu594kmqax6k8crqtjaqrypux3z5aq9e97xx0',
  approve_attack_prevention: true
}
undefined
>> const queryGetBlockchainCount = await datsClient.getBlockchainCount();
undefined
>> console.log(queryGetBlockchainCount);
{ blockchain_length: 1 }
undefined