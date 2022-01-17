use crate::config::Configuration;
use crate::config::DefaultConfiguration;

static DEFAULT_CONFIGURATION: DefaultConfiguration = DefaultConfiguration {};

#[test]
fn test_env() {
    for test_case in build_usize_test_cases() {
        test_case.assert_default();
        test_case.assert_env_key();
    }

    for test_case in build_u64_test_cases() {
        test_case.assert_default();
        test_case.assert_env_key();
    }
}

fn build_usize_test_cases() -> Vec<USizeEnvValue> {
    vec![
        USizeEnvValue {
            expected_default: 60 * 60 * 1000,
            env_key: String::from("SAFE_INFO_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.safe_info_cache_duration()),
        },
        USizeEnvValue {
            expected_default: 60 * 60 * 1000,
            env_key: String::from("ADDRESS_INFO_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.address_info_cache_duration()),
        },
        USizeEnvValue {
            expected_default: 60 * 60 * 24 * 1000,
            env_key: String::from("TOKEN_INFO_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.token_info_cache_duration()),
        },
        USizeEnvValue {
            expected_default: 60 * 60 * 1000,
            env_key: String::from("CHAIN_INFO_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.chain_info_cache_duration()),
        },
        USizeEnvValue {
            expected_default: 60 * 60 * 12 * 1000,
            env_key: String::from("EXCHANGE_API_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.exchange_api_cache_duration()),
        },
        USizeEnvValue {
            expected_default: 60 * 60 * 1000,
            env_key: String::from("REQUEST_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.request_cache_duration()),
        },
        USizeEnvValue {
            expected_default: 60 * 15 * 1000,
            env_key: String::from("ABOUT_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.about_cache_duration()),
        },
        USizeEnvValue {
            expected_default: 60 * 1000,
            env_key: String::from("BALANCES_REQUEST_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.balances_cache_duration()),
        },
        USizeEnvValue {
            expected_default: 60 * 60 * 1000,
            env_key: String::from("SAFE_APP_MANIFEST_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.safe_app_manifest_cache_duration()),
        },
        USizeEnvValue {
            expected_default: 60 * 1000,
            env_key: String::from("OWNERS_FOR_SAFES_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.owners_for_safes_cache_duration()),
        },
        USizeEnvValue {
            expected_default: 60 * 60 * 1000,
            env_key: String::from("BALANCES_CORE_REQUEST_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.balances_core_request_cache_duration()),
        },
        USizeEnvValue {
            expected_default: 10 * 1000,
            env_key: String::from("TOKEN_PRICE_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.token_price_cache_duration()),
        },
        USizeEnvValue {
            expected_default: DEFAULT_CONFIGURATION.request_cache_duration(),
            env_key: String::from("TX_QUEUED_CACHE_DURATION"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.tx_queued_cache_duration()),
        },
    ]
}

fn build_u64_test_cases() -> Vec<U64EnvValue> {
    vec![
        U64EnvValue {
            expected_default: 1000,
            env_key: String::from("INTERNAL_CLIENT_CONNECT_TIMEOUT"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.internal_client_connect_timeout()),
        },
        U64EnvValue {
            expected_default: 3000,
            env_key: String::from("SAFE_APP_INFO_REQUEST_TIMEOUT"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.safe_app_info_request_timeout()),
        },
        U64EnvValue {
            expected_default: 30000,
            env_key: String::from("TRANSACTION_REQUEST_TIMEOUT"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.transaction_request_timeout()),
        },
        U64EnvValue {
            expected_default: 10000,
            env_key: String::from("SAFE_INFO_REQUEST_TIMEOUT"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.safe_info_request_timeout()),
        },
        U64EnvValue {
            expected_default: 15000,
            env_key: String::from("TOKEN_INFO_REQUEST_TIMEOUT"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.token_info_request_timeout()),
        },
        U64EnvValue {
            expected_default: 3000,
            env_key: String::from("CONTRACT_INFO_REQUEST_TIMEOUT"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.contract_info_request_timeout()),
        },
        U64EnvValue {
            expected_default: 20000,
            env_key: String::from("BALANCES_REQUEST_TIMEOUT"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.balances_request_timeout()),
        },
        U64EnvValue {
            expected_default: 20000,
            env_key: String::from("COLLECTIBLES_REQUEST_TIMEOUT"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.collectibles_request_timeout()),
        },
        U64EnvValue {
            expected_default: 10000,
            env_key: String::from("DEFAULT_REQUEST_TIMEOUT"),
            generator: Box::new(|| DEFAULT_CONFIGURATION.default_request_timeout()),
        },
    ]
}

trait TestCase {
    fn assert_default(&self);
    fn assert_env_key(&self);
}

struct USizeEnvValue {
    expected_default: usize,
    env_key: String,
    generator: Box<dyn Fn() -> usize>,
}

impl TestCase for USizeEnvValue {
    fn assert_default(&self) {
        std::env::remove_var(&self.env_key);
        let actual_default = (&self.generator)();
        assert_eq!(
            self.expected_default, actual_default,
            "Test default value for env key: {}",
            &self.env_key
        );
    }

    fn assert_env_key(&self) {
        let mock_env_var_value = 1;
        std::env::set_var(&self.env_key, &mock_env_var_value.to_string());
        let actual_env = (&self.generator)();
        std::env::remove_var(&self.env_key);
        assert_eq!(
            mock_env_var_value, actual_env,
            "Test env var for env key: {}",
            &self.env_key
        );
    }
}

struct U64EnvValue {
    expected_default: u64,
    env_key: String,
    generator: Box<dyn Fn() -> u64>,
}

impl TestCase for U64EnvValue {
    fn assert_default(&self) {
        std::env::remove_var(&self.env_key);
        let actual_default = (&self.generator)();
        assert_eq!(
            self.expected_default, actual_default,
            "Test default value for env key: {}",
            &self.env_key
        );
    }

    fn assert_env_key(&self) {
        let mock_env_var_value = 1;
        std::env::set_var(&self.env_key, &mock_env_var_value.to_string());
        let actual_env = (&self.generator)();
        std::env::remove_var(&self.env_key);
        assert_eq!(
            mock_env_var_value, actual_env,
            "Test env var for env key: {}",
            &self.env_key
        );
    }
}
