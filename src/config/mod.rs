use std::env;
use std::str::FromStr;

use lazy_static::lazy_static;
use mockall::automock;

#[cfg(test)]
mod tests;

#[automock]
pub trait Configuration {
    fn redis_uri(self: &Self) -> String;
    fn base_config_service_uri(self: &Self) -> String;
    fn base_exchange_api_uri(self: &Self) -> String;
    fn webhook_token(self: &Self) -> String;
    fn transaction_service_auth_token(self: &Self) -> String;
    fn scheme(self: &Self) -> String;
    fn indefinite_timeout(self: &Self) -> usize;
    fn short_error_duration(self: &Self) -> usize;
    fn long_error_duration(self: &Self) -> usize;
    fn safe_info_cache_duration(self: &Self) -> usize;
    fn address_info_cache_duration(self: &Self) -> usize;
    fn token_info_cache_duration(self: &Self) -> usize;
    fn chain_info_cache_duration(self: &Self) -> usize;
    fn chain_info_response_cache_duration(self: &Self) -> usize;
    fn exchange_api_cache_duration(self: &Self) -> usize;
    fn request_cache_duration(self: &Self) -> usize;
    fn about_cache_duration(self: &Self) -> usize;
    fn balances_cache_duration(self: &Self) -> usize;
    fn balances_core_request_cache_duration(self: &Self) -> usize;
    fn safe_app_manifest_cache_duration(self: &Self) -> usize;
    fn owners_for_safes_cache_duration(self: &Self) -> usize;
    fn safe_apps_cache_duration(self: &Self) -> usize;
    fn token_price_cache_duration(self: &Self) -> usize;
    fn tx_queued_cache_duration(self: &Self) -> usize;
    fn internal_client_connect_timeout(self: &Self) -> u64;
    fn safe_app_info_request_timeout(self: &Self) -> u64;
    fn transaction_request_timeout(self: &Self) -> u64;
    fn safe_info_request_timeout(self: &Self) -> u64;
    fn token_info_request_timeout(self: &Self) -> u64;
    fn chain_info_request_timeout(self: &Self) -> u64;
    fn contract_info_request_timeout(self: &Self) -> u64;
    fn balances_request_timeout(self: &Self) -> u64;
    fn collectibles_request_timeout(self: &Self) -> u64;
    fn default_request_timeout(self: &Self) -> u64;
    fn request_error_cache_duration(self: &Self) -> usize;
    fn log_all_error_responses(self: &Self) -> bool;
    fn redis_scan_count(self: &Self) -> usize;
    fn feature_flag_nested_decoding(self: &Self) -> bool;
    fn feature_flag_balances_rate_implementation(self: &Self) -> bool;
    fn vpc_transaction_service_uri(self: &Self) -> bool;
    fn concurrent_balance_token_requests(self: &Self) -> usize;
    fn log_threshold(self: &Self) -> f32;
    fn build_number(self: &Self) -> Option<String>;
    fn version(self: &Self) -> String;
}

struct DefaultConfiguration {}

lazy_static! {
    pub static ref DEFAULT_CONFIGURATION: Box<dyn Configuration + Sync> =
        Box::new(DefaultConfiguration {});
}

impl Configuration for DefaultConfiguration {
    fn redis_uri(self: &Self) -> String {
        env::var("REDIS_URI").expect("REDIS_URI missing in env")
    }

    fn base_config_service_uri(self: &Self) -> String {
        format!(
            "{}{}",
            env::var("CONFIG_SERVICE_URI").expect("CONFIG_SERVICE_URI missing in env"),
            "/api"
        )
    }

    fn base_exchange_api_uri(self: &Self) -> String {
        format!(
            "{}?access_key={}",
            env::var("EXCHANGE_API_BASE_URI").unwrap(),
            env::var("EXCHANGE_API_KEY").unwrap()
        )
    }

    fn webhook_token(self: &Self) -> String {
        env::var("WEBHOOK_TOKEN").expect("WEBHOOK_TOKEN missing in env")
    }

    fn transaction_service_auth_token(self: &Self) -> String {
        let token = env::var("TRANSACTION_SERVICE_AUTH_TOKEN").unwrap_or_else(|_| {
            log::warn!("TRANSACTION_SERVICE_AUTH_TOKEN missing in env");
            String::new()
        });
        format!("Token {}", token)
    }

    fn scheme(self: &Self) -> String {
        env_with_default("SCHEME", "https".into())
    }

    // TIME DURATION VALUES
    fn indefinite_timeout(self: &Self) -> usize {
        env_with_default("INDEFINITE_TIMEOUT", 60 * 60 * 1000)
    }

    fn short_error_duration(self: &Self) -> usize {
        env_with_default("SHORT_ERROR_DURATION", 60 * 1000)
    }

    fn long_error_duration(self: &Self) -> usize {
        env_with_default("LONG_ERROR_DURATION", 60 * 15 * 1000)
    }

    // FUNCTIONAL TIMEOUTS
    fn safe_info_cache_duration(self: &Self) -> usize {
        env_with_default("SAFE_INFO_CACHE_DURATION", self.indefinite_timeout())
    }

    fn address_info_cache_duration(self: &Self) -> usize {
        env_with_default("ADDRESS_INFO_CACHE_DURATION", self.indefinite_timeout())
    }

    fn token_info_cache_duration(self: &Self) -> usize {
        env_with_default("TOKEN_INFO_CACHE_DURATION", 60 * 60 * 24 * 1000)
    }

    fn chain_info_cache_duration(self: &Self) -> usize {
        env_with_default("CHAIN_INFO_CACHE_DURATION", self.indefinite_timeout())
    }

    fn chain_info_response_cache_duration(self: &Self) -> usize {
        env_with_default("CHAIN_INFO_RESPONSE_CACHE_DURATION", 1) // set to negligible value
    }

    fn exchange_api_cache_duration(self: &Self) -> usize {
        env_with_default("EXCHANGE_API_CACHE_DURATION", 60 * 60 * 12 * 1000)
    }

    fn request_cache_duration(self: &Self) -> usize {
        env_with_default("REQUEST_CACHE_DURATION", self.indefinite_timeout())
    }

    fn about_cache_duration(self: &Self) -> usize {
        env_with_default("ABOUT_CACHE_DURATION", 60 * 15 * 1000)
    }

    fn balances_cache_duration(self: &Self) -> usize {
        env_with_default("BALANCES_REQUEST_CACHE_DURATION", 60 * 1000)
    }

    fn balances_core_request_cache_duration(self: &Self) -> usize {
        env_with_default(
            "BALANCES_CORE_REQUEST_CACHE_DURATION",
            self.indefinite_timeout(),
        )
    }

    fn safe_app_manifest_cache_duration(self: &Self) -> usize {
        env_with_default(
            "SAFE_APP_MANIFEST_CACHE_DURATION",
            self.indefinite_timeout(),
        )
    }

    fn owners_for_safes_cache_duration(self: &Self) -> usize {
        env_with_default("OWNERS_FOR_SAFES_CACHE_DURATION", 60 * 1000)
    }

    fn safe_apps_cache_duration(self: &Self) -> usize {
        env_with_default("SAFE_APPS_CACHE_DURATION", self.indefinite_timeout())
    }

    fn token_price_cache_duration(self: &Self) -> usize {
        env_with_default("TOKEN_PRICE_CACHE_DURATION", 10 * 1000)
    }

    fn tx_queued_cache_duration(self: &Self) -> usize {
        env_with_default("TX_QUEUED_CACHE_DURATION", self.request_cache_duration())
    }

    // REQUEST TIMEOUTS
    fn internal_client_connect_timeout(self: &Self) -> u64 {
        env_with_default("INTERNAL_CLIENT_CONNECT_TIMEOUT", 1000)
    }

    fn safe_app_info_request_timeout(self: &Self) -> u64 {
        env_with_default("SAFE_APP_INFO_REQUEST_TIMEOUT", 3000)
    }

    fn transaction_request_timeout(self: &Self) -> u64 {
        env_with_default("TRANSACTION_REQUEST_TIMEOUT", 30000)
    }

    fn safe_info_request_timeout(self: &Self) -> u64 {
        env_with_default("SAFE_INFO_REQUEST_TIMEOUT", 10000)
    }

    fn token_info_request_timeout(self: &Self) -> u64 {
        env_with_default("TOKEN_INFO_REQUEST_TIMEOUT", 15000)
    }

    fn chain_info_request_timeout(self: &Self) -> u64 {
        env_with_default("CHAIN_INFO_REQUEST_TIMEOUT", 15000)
    }

    fn contract_info_request_timeout(self: &Self) -> u64 {
        env_with_default("CONTRACT_INFO_REQUEST_TIMEOUT", 3000)
    }

    fn balances_request_timeout(self: &Self) -> u64 {
        env_with_default("BALANCES_REQUEST_TIMEOUT", 20000)
    }

    fn collectibles_request_timeout(self: &Self) -> u64 {
        env_with_default("COLLECTIBLES_REQUEST_TIMEOUT", 20000)
    }

    fn default_request_timeout(self: &Self) -> u64 {
        env_with_default("DEFAULT_REQUEST_TIMEOUT", 10000)
    }

    // ERRORS
    fn request_error_cache_duration(self: &Self) -> usize {
        env_with_default("REQS_ERROR_CACHE_DURATION", self.short_error_duration())
    }

    fn log_all_error_responses(self: &Self) -> bool {
        env_with_default("LOG_ALL_ERROR_RESPONSES", false)
    }

    // OTHERS
    fn redis_scan_count(self: &Self) -> usize {
        env_with_default("REDIS_SCAN_COUNT", 300)
    }

    fn feature_flag_nested_decoding(self: &Self) -> bool {
        env_with_default("FEATURE_FLAG_NESTED_DECODING", true)
    }

    fn feature_flag_balances_rate_implementation(self: &Self) -> bool {
        env_with_default("FEATURE_FLAG_BALANCES_RATE_IMPLEMENTATION", false)
    }

    fn vpc_transaction_service_uri(self: &Self) -> bool {
        env_with_default("VPC_TRANSACTION_SERVICE_URI", true)
    }

    fn concurrent_balance_token_requests(self: &Self) -> usize {
        env_with_default("CONCURRENT_BALANCE_TOKEN_REQUESTS", 5)
    }

    fn log_threshold(self: &Self) -> f32 {
        env_with_default("LOG_THRESHOLD", 1.0)
    }

    fn build_number(self: &Self) -> Option<String> {
        option_env!("BUILD_NUMBER").map(|it| it.to_string())
    }

    fn version(self: &Self) -> String {
        option_env!("VERSION")
            .unwrap_or(env!("CARGO_PKG_VERSION"))
            .to_string()
    }
}

fn env_with_default<T: FromStr>(key: &str, default: T) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    match env::var(key) {
        Ok(value) => value
            .parse()
            .expect(&format!("Parsing of {} env var key failed", &key)),
        Err(_) => default,
    }
}
