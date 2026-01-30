use soroban_sdk::{contractevent, Address};

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializationEvent {
    pub usdc_token: Address,
    pub platform_wallet: Address,
    pub event_registry: Address,
}
