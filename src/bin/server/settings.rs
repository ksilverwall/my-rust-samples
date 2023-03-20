use std::env;


pub struct DatabaseSettings {
    pub db_host: String,
    pub db_port: String,
}

impl DatabaseSettings {
    pub fn get_url(&self) -> String {
        return format!(
            "postgresql://app:appPassword@{}:{}/lt",
            self.db_host, self.db_port
        );
    }
}

pub struct EthereumSettings {
    pub node_url: String,
    pub abi_file: String,
    pub contract_address: String,
}

pub struct Settings {
    pub ethereum: EthereumSettings,
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn load() -> Result<Self, String> {
        let database = DatabaseSettings {
            db_host: env::var("DB_HOST").unwrap_or("localhost".to_string()),
            db_port: env::var("DB_PORT").unwrap_or("5432".to_string()),
        };

        let ethereum = EthereumSettings {
            node_url: env::var("NODE_URL").unwrap_or("http://localhost:8545".to_string()),
            abi_file: env::var("ABI_FILE").map_err(|e| format!("{e:}: ABI_FILE"))?,
            contract_address: env::var("CONTRACT_ADDRESS")
                .map_err(|e| format!("{e:}: CONTRACT_ADDRESS"))?,
        };

        let settings = Settings {
            database: database,
            ethereum: ethereum,
        };

        Ok(settings)
    }
}
