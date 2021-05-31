mod apis;
mod models;

pub const WS_URL: &'static str = "wss://www.deribit.com/ws/api/v2";
pub const WS_URL_TESTNET: &'static str = "wss://test.deribit.com/ws/api/v2";

pub struct Deribit {
    testnet : bool,
}

impl Deribit {

}