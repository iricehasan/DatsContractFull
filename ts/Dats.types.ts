/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.7.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export interface InstantiateMsg {
  owner: string;
}
export type ExecuteMsg = {
  save_d_dos: {
    is_approve: boolean;
    traffic_scale: number;
  };
} | {
  save_super_computer: {
    cpu_value: number;
    is_approve: boolean;
  };
} | {
  save_cyber_security: {
    is_approve: boolean;
    malware_research: boolean;
    ransomeware_research: boolean;
    server_security: boolean;
    web_security: boolean;
  };
} | {
  save_vulnerabilitiy: {
    blockchain_penetration: boolean;
    contract_penetration: boolean;
    is_approve: boolean;
    scada_penetration: boolean;
    server_penetration: boolean;
    web_penetration: boolean;
  };
} | {
  save_blockchain: {
    approve_attack_prevention: boolean;
  };
};
export type QueryMsg = {
  get_ddoss_by_user: {
    user: Addr;
  };
} | {
  get_ddos_count: {};
} | {
  get_all_user_ddos_settings: {};
} | {
  get_all_user_super_computer_settings: {};
} | {
  get_super_computer_by_user: {
    user: Addr;
  };
} | {
  get_super_computer_count: {};
} | {
  get_all_user_cyber_security_settings: {};
} | {
  get_cyber_security_by_user: {
    user: Addr;
  };
} | {
  get_cyber_security_count: {};
} | {
  get_all_user_vulnerability_settings: {};
} | {
  get_vulnerability_by_user: {
    user: Addr;
  };
} | {
  get_vulnerabilitiy_count: {};
} | {
  get_blockchain_by_user: {
    user: Addr;
  };
} | {
  get_blockchain_count: {};
};
export type Addr = string;
export interface CyberSecuritiesResponse {
  cybersecurities: CyberSecurity[];
}
export interface CyberSecurity {
  id: number;
  is_approve: boolean;
  malware_research: boolean;
  ransomeware_research: boolean;
  server_security: boolean;
  user: string;
  web_security: boolean;
}
export interface DdossesResponse {
  ddosses: Ddos[];
}
export interface Ddos {
  id: number;
  is_approve: boolean;
  traffic_scale: number;
  user: string;
}
export interface SuperComputersResponse {
  supercomputers: SuperComputer[];
}
export interface SuperComputer {
  cpu_value: number;
  id: number;
  is_approve: boolean;
  user: string;
}
export interface VulnerabilitiesResponse {
  vulnerabilities: Vulnerability[];
}
export interface Vulnerability {
  blockchain_penetration: boolean;
  contract_penetration: boolean;
  id: number;
  is_approve: boolean;
  scada_penetration: boolean;
  server_penetration: boolean;
  user: string;
  web_penetration: boolean;
}
export interface BlockchainResponse {
  approve_attack_prevention: boolean;
  id: number;
  user: string;
}
export interface BlockchainLengthResponse {
  blockchain_length: number;
}
export interface CyberSecurityResponse {
  id: number;
  is_approve: boolean;
  malware_research: boolean;
  ransomeware_research: boolean;
  server_security: boolean;
  user: string;
  web_security: boolean;
}
export interface CyberLengthResponse {
  cyberlength: number;
}
export interface DdosLengthResponse {
  ddoslength: number;
}
export interface DdosResponse {
  id: number;
  is_approve: boolean;
  traffic_scale: number;
  user: string;
}
export interface SuperComputerResponse {
  cpu_value: number;
  id: number;
  is_approve: boolean;
  user: string;
}
export interface SuperLengthResponse {
  superlength: number;
}
export interface VulnerabilityLengthResponse {
  vulnerability_length: number;
}
export interface VulnerabilityResponse {
  blockchain_penetration: boolean;
  contract_penetration: boolean;
  id: number;
  is_approve: boolean;
  scada_penetration: boolean;
  server_penetration: boolean;
  user: string;
}