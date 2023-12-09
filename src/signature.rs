use hex;
use std::time::{SystemTime, UNIX_EPOCH};
use ethers::abi::{encode_packed, Token};

pub fn now_timestamp() -> u64 {
    let current_time = SystemTime::now();
    let since_epoch = current_time.duration_since(UNIX_EPOCH).unwrap();
    let seconds = since_epoch.as_secs();
    seconds
}

pub fn try_abi_encode() {
    /*
     * 0x
     * cb62320d
     * 6869
     * 00000000000000000000000000000000000000000000000000f8b0a10e470000
     * b54e978a34af50228a3564662db6005e9fb04f5a
     * 0000000000000000000000000000000000000000000000000000000065705d53
     */

    let selector = "cb62320d";
    let binder_name = "hi";
    let amount = 70000_000000_000000_u128;
    let amount_padded_hex = format!("{:0>64}", format!("{:016X}", amount));
    let timestamp_padded_hex = format!("{:0>64}", format!("{:016X}", now_timestamp()));

    let packed_data: Vec<u8> = encode_packed(&[
        Token::Bytes(hex::decode(selector).unwrap().into()), 
        Token::String(binder_name.into()),
        Token::Bytes(hex::decode(amount_padded_hex).unwrap().into()),
        Token::Address("b54e978a34af50228a3564662db6005e9fb04f5a".parse().unwrap()),
        Token::Bytes(hex::decode(timestamp_padded_hex).unwrap().into())
    ]).unwrap();
    println!("{:?}", hex::encode(&packed_data));
}