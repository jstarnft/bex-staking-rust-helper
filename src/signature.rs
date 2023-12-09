use ethers::abi::{encode_packed, Token};

pub fn try_abi_encode() {
    /*
     * 0x
     * cb62320d
     * 6869
     * 00000000000000000000000000000000000000000000000000f8b0a10e470000
     * b54e978a34af50228a3564662db6005e9fb04f5a
     * 0000000000000000000000000000000000000000000000000000000065705d53
     */
    let encoded = encode_packed(&[
        Token::Bytes("cb62320d".into()), 
        Token::String("hi".into()),
        Token::Uint("7000000000".parse().unwrap()),
        Token::Address("b54e978a34af50228a3564662db6005e9fb04f5a".parse().unwrap()),
        Token::Uint("1701862739".parse().unwrap()),
    ]).unwrap();
    println!("{:?}", encoded);
}