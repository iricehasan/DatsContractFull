#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult, Addr
};

use std::ops::Add;

use crate::error::ContractError;
use crate::msg::{DdosResponse, DdossesResponse, DdosLengthResponse, SuperComputerResponse, SuperComputersResponse, SuperLengthResponse, CyberLengthResponse, 
    CyberSecurityResponse, CyberSecuritiesResponse, VulnerabilityResponse, VulnerabilitiesResponse, VulnerabilityLengthResponse, BlockchainLengthResponse, 
    BlockchainResponse, BlockchainsResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Ddos, SuperComputer, CyberSecurity, Vulnerability, Blockchain, CONFIG, DDOS_ID_SEQ, SUPERCOMPUTER_ID_SEQ, CYBERSECURITY_ID_SEQ, 
    VULNERABILITY_ID_SEQ, BLOCKCHAIN_ID_SEQ, DDOSSES, SUPERCOMPUTERS, CYBERSECURITIES, VULNERABILITIES, BLOCKCHAINS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let owner = msg.owner;

    let config = Config {
        owner: owner.clone(),
    };

    CONFIG.save(deps.storage, &config)?;

    DDOS_ID_SEQ.save(deps.storage, &0u128)?;
    SUPERCOMPUTER_ID_SEQ.save(deps.storage, &0u128)?;
    CYBERSECURITY_ID_SEQ.save(deps.storage, &0u128)?;
    VULNERABILITY_ID_SEQ.save(deps.storage, &0u128)?;
    BLOCKCHAIN_ID_SEQ.save(deps.storage, &0u128)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SaveDDos {
            is_approve,
            traffic_scale,
        } => execute_save_ddos(deps, info, is_approve, traffic_scale),
        ExecuteMsg::SaveSuperComputer{
            is_approve,
            cpu_value,
        } => execute_save_supercomputer(deps, info, is_approve, cpu_value),
        ExecuteMsg::SaveCyberSecurity {
            is_approve,
            web_security,
            server_security,
            ransomeware_research,
            malware_research,
        } => execute_save_cybersecurity(deps, info, is_approve, web_security, server_security, ransomeware_research, malware_research),
        ExecuteMsg::SaveVulnerabilitiy {
            is_approve,
            web_penetration,
            server_penetration,
            scada_penetration,
            blockchain_penetration,
            contract_penetration,
        } => execute_save_vulnerability(deps, info, is_approve, web_penetration, server_penetration, scada_penetration, blockchain_penetration, contract_penetration,),
        ExecuteMsg::SaveBlockchain {
            approve_attack_prevention,
        } => execute_save_blockchain(deps, info, approve_attack_prevention),
    }
}

pub fn execute_save_ddos(
    deps: DepsMut,
    info: MessageInfo,
    is_approve: bool,
    traffic_scale: u8, 
) -> Result<Response, ContractError> {

    let user = info.sender.to_string().clone();

    let id = DDOS_ID_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_ddos = Ddos {
        id,
        user: user.clone(),
        is_approve,
        traffic_scale,
    };

    DDOSSES.save(deps.storage, info.sender.as_str().as_bytes(), &new_ddos)?;
    Ok(Response::new()
        .add_attribute("method", "execute_save_ddos")
        .add_attribute("ddos", id.to_string())
        .add_attribute("user", user.clone()))
}

pub fn execute_save_supercomputer(
    deps: DepsMut,
    info: MessageInfo,
    is_approve: bool,
    cpu_value: u8, 
) -> Result<Response, ContractError> {

    let user = info.sender.to_string().clone();

    let id = SUPERCOMPUTER_ID_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_supercomputer = SuperComputer {
        id,
        user: user.clone(),
        is_approve,
        cpu_value,
    };

    SUPERCOMPUTERS.save(deps.storage, info.sender.as_str().as_bytes(), &new_supercomputer)?;
    Ok(Response::new()
        .add_attribute("method", "execute_save_supercomputer")
        .add_attribute("supercomputer", id.to_string())
        .add_attribute("user", user.clone()))
}

pub fn execute_save_cybersecurity(
    deps: DepsMut,
    info: MessageInfo,
    is_approve: bool,
    web_security: bool, 
    server_security: bool,
    ransomeware_research: bool,
    malware_research: bool,
) -> Result<Response, ContractError> {

    let user = info.sender.to_string().clone();

    let id = CYBERSECURITY_ID_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_cybersecurity = CyberSecurity {
        id,
        user: user.clone(),
        is_approve,
        web_security,
        server_security,
        ransomeware_research,
        malware_research,
    };

    CYBERSECURITIES.save(deps.storage, info.sender.as_str().as_bytes(), &new_cybersecurity)?;
    Ok(Response::new()
        .add_attribute("method", "execute_save_cybersecurity")
        .add_attribute("cybersecurity", id.to_string())
        .add_attribute("user", user.clone()))
}

pub fn execute_save_vulnerability(
    deps: DepsMut,
    info: MessageInfo,
    is_approve: bool,
    web_penetration: bool, 
    server_penetration: bool,
    scada_penetration: bool,
    blockchain_penetration: bool,
    contract_penetration: bool,
) -> Result<Response, ContractError> {

    let user = info.sender.to_string().clone();

    let id = VULNERABILITY_ID_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_vulnerability = Vulnerability {
        id,
        user: user.clone(),
        is_approve,
        web_penetration,
        server_penetration,
        scada_penetration,
        blockchain_penetration,
        contract_penetration,
    };

    VULNERABILITIES.save(deps.storage, info.sender.as_str().as_bytes(), &new_vulnerability)?;
    Ok(Response::new()
        .add_attribute("method", "execute_save_vulnerability")
        .add_attribute("vulnerability", id.to_string())
        .add_attribute("user", user.clone()))
}

pub fn execute_save_blockchain(
    deps: DepsMut,
    info: MessageInfo,
    approve_attack_prevention: bool,
) -> Result<Response, ContractError> {

    let user = info.sender.to_string().clone();

    let id = BLOCKCHAIN_ID_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let new_blockchain = Blockchain {
        id,
        user: user.clone(),  
        approve_attack_prevention,
    };

   BLOCKCHAINS.save(deps.storage, info.sender.as_str().as_bytes(), &new_blockchain)?;
    Ok(Response::new()
        .add_attribute("method", "execute_save_blockchain")
        .add_attribute("blockchain", id.to_string())
        .add_attribute("user", user.clone()))
}

// later on maybe add functionality to get contract owner from config
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetDdossByUser { user } => to_json_binary(&query_get_ddos_by_user(deps, user)?),
        QueryMsg::GetDdosCount {} => to_json_binary(&query_get_ddos_count(deps)?),
        QueryMsg::GetAllUserDdosSettings {} => to_json_binary(&query_get_all_user_ddos_settings(deps)?),
        QueryMsg::GetAllUserSuperComputerSettings {} => to_json_binary(&query_get_all_user_supercomputer_settings(deps)?),
        QueryMsg::GetSuperComputerByUser { user } => to_json_binary(&query_get_supercomputer_by_user(deps, user)?),
        QueryMsg::GetSuperComputerCount {} => to_json_binary(&query_get_supercomputer_count(deps)?),
        QueryMsg::GetAllUserCyberSecuritySettings {} => to_json_binary(&query_get_all_user_cybersecurity_settings(deps)?),
        QueryMsg::GetCyberSecurityByUser { user } => to_json_binary(&query_get_cybersecurity_by_user(deps, user)?),
        QueryMsg::GetCyberSecurityCount {} => to_json_binary(&query_get_cybersecurity_count(deps)?),
        QueryMsg::GetAllUserVulnerabilitySettings {} => to_json_binary(&query_get_all_user_vulnerability_settings(deps)?),
        QueryMsg::GetVulnerabilityByUser { user } => to_json_binary(&query_get_vulnerability_by_user(deps, user)?),
        QueryMsg::GetVulnerabilitiyCount {} => to_json_binary(&query_get_vulnerability_count(deps)?),
        QueryMsg::GetBlockchainByUser { user } => to_json_binary(&query_get_blockchain_by_user(deps, user)?),
        QueryMsg::GetBlockchainCount {} => to_json_binary(&query_get_blockchain_count(deps)?),
    }
}

/* 
pub fn query_get_ddos(
    deps: Deps,
    env: Env,
    user: String,
) -> StdResult<Binary> {

}
*/

pub fn query_get_ddos_by_user(
    deps: Deps,
    user: Addr,
) -> StdResult<DdosResponse> {
    let ddos = DDOSSES.load(deps.storage, user.as_str().as_bytes())?;
    Ok(DdosResponse {
        id: ddos.id,
        user: ddos.user,
        is_approve: ddos.is_approve,
        traffic_scale: ddos.traffic_scale,
    })
}

pub fn query_get_all_user_ddos_settings(
    deps: Deps,
) -> StdResult<DdossesResponse> {
    let ddosses: StdResult<Vec<_>> = DDOSSES
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let result = DdossesResponse {
        ddosses: ddosses?.into_iter().map(|d| d.1).collect(),
    };
    Ok(result)
}

pub fn query_get_ddos_count(
    deps: Deps, 
) -> StdResult<DdosLengthResponse> {
    let query_ddosses: StdResult<Vec<_>> = DDOSSES
    .range(deps.storage, None, None, Order::Ascending)
    .collect();

    let ddosses: Vec<Ddos> = query_ddosses?.into_iter().map(|d| d.1).collect();
    let ddoslength: usize = ddosses.len();
    Ok( DdosLengthResponse {ddoslength})
}

/* 
pub fn query_get_supercomputer(
    deps: Deps,
    env: Env,
    user: String,
) -> StdResult<Binary> {

}
*/

pub fn query_get_supercomputer_by_user(
    deps: Deps,
    user: Addr,
) -> StdResult<SuperComputerResponse> {
    let supercomputer = SUPERCOMPUTERS.load(deps.storage, user.as_str().as_bytes())?;
    Ok(SuperComputerResponse {
        id: supercomputer.id,
        user: supercomputer.user,
        is_approve: supercomputer.is_approve,
        cpu_value: supercomputer.cpu_value,
    })

}

// change to support admin only query
pub fn query_get_all_user_supercomputer_settings(
    deps: Deps,
) -> StdResult<SuperComputersResponse> {
    let supercomputers: StdResult<Vec<_>> = SUPERCOMPUTERS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let result = SuperComputersResponse {
        supercomputers: supercomputers?.into_iter().map(|s| s.1).collect(),
    };
    Ok(result)
}

pub fn query_get_cybersecurity_by_user(
    deps: Deps,
    user: Addr,
) -> StdResult<CyberSecurityResponse> {
    let cybersecurities = CYBERSECURITIES.load(deps.storage, user.as_str().as_bytes())?;
    Ok(CyberSecurityResponse {
        id: cybersecurities.id,
        user: cybersecurities.user,
        is_approve: cybersecurities.is_approve,
        web_security: cybersecurities.web_security,
        server_security: cybersecurities.server_security,
        ransomeware_research: cybersecurities.ransomeware_research,
        malware_research: cybersecurities.malware_research,
    })

}

pub fn query_get_all_user_cybersecurity_settings(
    deps: Deps,
) -> StdResult<CyberSecuritiesResponse> {
    let cybersecurities: StdResult<Vec<_>> = CYBERSECURITIES
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let result = CyberSecuritiesResponse {
        cybersecurities: cybersecurities?.into_iter().map(|s| s.1).collect(),
    };
    Ok(result)
}

pub fn query_get_supercomputer_count(
    deps: Deps, 
) -> StdResult<SuperLengthResponse> {
    let query_supercomputers: StdResult<Vec<_>> = SUPERCOMPUTERS
    .range(deps.storage, None, None, Order::Ascending)
    .collect();

    let supercomputers: Vec<SuperComputer> = query_supercomputers?.into_iter().map(|s| s.1).collect();
    let superlength: usize = supercomputers.len();
    Ok(SuperLengthResponse {superlength})

}


pub fn query_get_cybersecurity_count(
    deps: Deps, 
) -> StdResult<CyberLengthResponse> {
    let query_cybersecurities: StdResult<Vec<_>> = CYBERSECURITIES
    .range(deps.storage, None, None, Order::Ascending)
    .collect();

    let cybersecurities: Vec<CyberSecurity> = query_cybersecurities?.into_iter().map(|c| c.1).collect();
    let cyberlength: usize = cybersecurities.len();
    Ok( CyberLengthResponse {cyberlength})

}

pub fn query_get_vulnerability_by_user(
    deps: Deps,
    user: Addr,
) -> StdResult<VulnerabilityResponse> {
    let vulnerability = VULNERABILITIES.load(deps.storage, user.as_str().as_bytes())?;

    Ok(VulnerabilityResponse { 
        id: vulnerability.id,
        user: vulnerability.user,
        is_approve: vulnerability.is_approve,
        server_penetration: vulnerability.server_penetration,
        scada_penetration: vulnerability.scada_penetration,
        blockchain_penetration: vulnerability.blockchain_penetration,
        contract_penetration: vulnerability.contract_penetration,
    })

}

// change for msg.sender
pub fn query_get_all_user_vulnerability_settings(
    deps: Deps,
) -> StdResult<VulnerabilitiesResponse> {
    let vulnerabilities: StdResult<Vec<_>> = VULNERABILITIES
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let result = VulnerabilitiesResponse {
        vulnerabilities: vulnerabilities?.into_iter().map(|v| v.1).collect(),
    };
    Ok(result)
}

pub fn query_get_vulnerability_count(
    deps: Deps, 
) -> StdResult<VulnerabilityLengthResponse> {
    let query_vulnerabilities: StdResult<Vec<_>> = VULNERABILITIES
    .range(deps.storage, None, None, Order::Ascending)
    .collect();

    let vulnerabilities: Vec<Vulnerability> = query_vulnerabilities?.into_iter().map(|v| v.1).collect();
    let vulnerability_length: usize = vulnerabilities.len();

    Ok(VulnerabilityLengthResponse {vulnerability_length})
}

pub fn query_get_blockchain_by_user(
    deps: Deps,
    user: Addr,
) -> StdResult<BlockchainResponse> {
    let blockchain = BLOCKCHAINS.load(deps.storage, user.as_str().as_bytes())?;

    Ok(BlockchainResponse { 
        id: blockchain.id,
        user: blockchain.user,
        approve_attack_prevention: blockchain.approve_attack_prevention,
    })
}

pub fn query_get_all_user_blockchain_settings(
    deps: Deps,
) -> StdResult<BlockchainsResponse> {
    let blockchains: StdResult<Vec<_>> = BLOCKCHAINS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let result = BlockchainsResponse {
        blockchains: blockchains?.into_iter().map(|b| b.1).collect(),
    };
    Ok(result)
}

pub fn query_get_blockchain_count(
    deps: Deps,
) -> StdResult<BlockchainLengthResponse> {
    let query_blockchains: StdResult<Vec<_>> = BLOCKCHAINS
    .range(deps.storage, None, None, Order::Ascending)
    .collect();

    let blockchains: Vec<Blockchain> = query_blockchains?.into_iter().map(|b| b.1).collect();
    let blockchain_length: usize = blockchains.len();

    Ok(BlockchainLengthResponse {blockchain_length})
}

