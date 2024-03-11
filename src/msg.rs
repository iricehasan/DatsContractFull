use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use crate::state::{Blockchain, CyberSecurity, Ddos, SuperComputer, Vulnerability};

pub struct InstantiateMsg {
    pub owner: String,
}

pub enum ExecuteMsg {
    SaveDDos {
        is_approve: bool,
        traffic_scale: u8,
    },

    SaveSuperComputer {
        is_approve: bool,
        cpu_value: u8,
    },

    SaveCyberSecurity {
        is_approve: bool,
        web_security: bool,
        server_security: bool,
        ransomeware_research: bool,
        malware_research: bool,
    },

    SaveVulnerabilitiy {
        is_approve: bool,
        web_penetration: bool,
        server_penetration: bool,
        scada_penetration: bool,
        blockchain_penetration: bool,
        contract_penetration: bool,
    },

    SaveBlockchain {
        approve_attack_prevention: bool,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(DdosResponse)]
    GetDdossByUser { user: Addr },
    #[returns(DdosLengthResponse)]
    GetDdosCount {},
    #[returns(DdossesResponse)]
    GetAllUserDdosSettings {},
    #[returns(SuperComputersResponse)]
    GetAllUserSuperComputerSettings {},
    #[returns(SuperComputerResponse)]
    GetSuperComputerByUser { user: Addr },
    #[returns(SuperLengthResponse)]
    GetSuperComputerCount {},
    #[returns(CyberSecuritiesResponse)]
    GetAllUserCyberSecuritySettings {},
    #[returns(CyberSecurityResponse)]
    GetCyberSecurityByUser { user: Addr },
    #[returns(CyberLengthResponse)]
    GetCyberSecurityCount {},
    #[returns(VulnerabilitiesResponse)]
    GetAllUserVulnerabilitySettings {},
    #[returns(VulnerabilityResponse)]
    GetVulnerabilityByUser { user: Addr },
    #[returns(VulnerabilityLengthResponse)]
    GetVulnerabilitiyCount {},
    #[returns(BlockchainResponse)]
    GetBlockchainByUser { user: Addr },
    #[returns(BlockchainLengthResponse)]
    GetBlockchainCount {},
}

#[cw_serde]
pub struct DdosResponse {
    pub id: u128,
    pub user: String,
    pub is_approve: bool,
    pub traffic_scale: u8,
}

#[cw_serde]
pub struct DdosLengthResponse {
    pub ddoslength: usize,
}

#[cw_serde]
pub struct DdossesResponse {
    pub ddosses: Vec<Ddos>,
}

#[cw_serde]
pub struct SuperComputerResponse {
    pub id: u128,
    pub user: String,
    pub is_approve: bool,
    pub cpu_value: u8,
}

#[cw_serde]
pub struct SuperComputersResponse {
    pub supercomputers: Vec<SuperComputer>,
}

#[cw_serde]
pub struct SuperLengthResponse {
    pub superlength: usize,
}

#[cw_serde]
pub struct CyberSecurityResponse {
    pub id: u128,
    pub user: String,
    pub is_approve: bool,
    pub web_security: bool,
    pub server_security: bool,
    pub ransomeware_research: bool,
    pub malware_research: bool,
}

#[cw_serde]
pub struct CyberSecuritiesResponse {
    pub cybersecurities: Vec<CyberSecurity>
}

#[cw_serde]
pub struct CyberLengthResponse {
    pub cyberlength: usize,
}

#[cw_serde]
pub struct VulnerabilityResponse {
    pub id: u128,
    pub user: String,
    pub is_approve: bool,
    pub server_penetration: bool,
    pub scada_penetration: bool,
    pub blockchain_penetration: bool,
    pub contract_penetration: bool,
}

#[cw_serde]
pub struct VulnerabilitiesResponse {
    pub vulnerabilities: Vec<Vulnerability>,
}

#[cw_serde]
pub struct VulnerabilityLengthResponse {
    pub vulnerability_length: usize,
}

#[cw_serde]
pub struct BlockchainResponse {
    pub id: u128,
    pub user: String,
    pub approve_attack_prevention: bool,
}

#[cw_serde]
pub struct BlockchainsResponse {
    pub blockchains: Vec<Blockchain>,
}

#[cw_serde]
pub struct BlockchainLengthResponse {
    pub blockchain_length: usize,
}
