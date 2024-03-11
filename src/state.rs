use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: String,
}

#[cw_serde]
pub struct Ddos {
    pub id: u128,
    pub user: String,
    pub is_approve: bool,
    pub traffic_scale: u8,
}

#[cw_serde]
pub struct SuperComputer {
    pub id: u128,
    pub user: String,
    pub is_approve: bool,
    pub cpu_value: u8,

}

#[cw_serde]
pub struct CyberSecurity {
    pub id: u128,
    pub user: String,
    pub is_approve: bool,
    pub web_security: bool,
    pub server_security: bool,
    pub ransomeware_research: bool,
    pub malware_research: bool,
}

#[cw_serde]
pub struct Vulnerability {
    pub id: u128,
    pub user: String,
    pub is_approve: bool,
    pub web_penetration: bool,
    pub server_penetration: bool,
    pub scada_penetration: bool,
    pub blockchain_penetration: bool,
    pub contract_penetration: bool,
}

#[cw_serde]
pub struct Blockchain {
    pub id: u128,
    pub user: String,
    pub approve_attack_prevention: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const DDOS_ID_SEQ: Item<u128> = Item::new("ddos_id_seq");
pub const SUPERCOMPUTER_ID_SEQ: Item<u128> = Item::new("supercomputer_id_seq");
pub const CYBERSECURITY_ID_SEQ: Item<u128> = Item::new("cybersecurity_id_seq");
pub const VULNERABILITY_ID_SEQ: Item<u128> = Item::new("vulnerability_id_seq");
pub const BLOCKCHAIN_ID_SEQ: Item<u128> = Item::new("blockchain_id_seq");

pub const DDOSSES: Map<&[u8], Ddos> = Map::new("ddoses"); // Addr as bytes &[u8] 
pub const SUPERCOMPUTERS: Map<&[u8], SuperComputer> = Map::new("supercomputers");
pub const CYBERSECURITIES: Map<&[u8], CyberSecurity> = Map::new("cybersecurities");
pub const VULNERABILITIES: Map<&[u8], Vulnerability> = Map::new("vulnerabilities");
pub const BLOCKCHAINS: Map<&[u8], Blockchain> = Map::new("blockchains");
