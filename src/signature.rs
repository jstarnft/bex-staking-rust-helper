use ethers::{
    abi::{encode_packed, Token},
    signers::Signer,
    types::Signature,
};
use hex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SigningUtils {}

impl SigningUtils {
    const SELECTOR_REGISTER: &'static str = "8580974c";
    const SELECTOR_BUY: &'static str = "bad9a87d";
    const SELECTOR_SELL: &'static str = "68670601";
    const SELECTOR_RENEW: &'static str = "cb62320d";

    /// Returns the current timestamp in seconds since the Unix epoch.
    pub fn _now_timestamp() -> u64 {
        let current_time = SystemTime::now();
        let since_epoch = current_time.duration_since(UNIX_EPOCH).unwrap();
        let seconds = since_epoch.as_secs();
        seconds
    }

    /// Signs a base request with specified parameters and a signer.
    ///
    /// This function prepares a message for signing by encoding various parameters into a hex string.
    /// It then uses the provided signer to generate a signature for the message.
    ///
    /// # Arguments
    /// * `selector` - A hexadecimal string representing the function selector.
    /// * `name` - A string parameter, typically representing a name.
    /// * `amount` - A u128 integer representing an amount, encoded into a 64-character zero-padded hex string.
    /// * `user` - A string representing the user's address.
    /// * `time` - A u64 timestamp, encoded into a 64-character zero-padded hex string.
    /// * `signer` - An implementation of the `Signer` trait, used to sign the message.
    ///
    /// # Returns
    /// A tuple containing:
    /// * The original timestamp (`u64`).
    /// * A string representation of the message in hex format.
    /// * The generated `Signature`.
    ///
    /// # Examples
    /// ```
    /// // Example usage
    /// let (timestamp, message, signature) = sign_base_request(
    ///     "cb62320d",
    ///     "Alice",
    ///     1000000,
    ///     "b54e978a34af50228a3564662db6005e9fb04f5a",
    ///     1651015203,
    ///     signer_instance
    /// );
    /// ```
    ///
    /// # Errors
    /// This function will return an error if the hex decoding fails or if the signer fails to sign the message.
    pub async fn _sign_base_requset(
        selector: &str,
        name: &str,
        amount: u128,
        user: &str,
        time: u64,
        signer: impl Signer,
    ) -> (u64, String, Signature) {
        let amount_padded_hex = format!("{:0>64}", format!("{:016X}", amount));
        let timestamp_padded_hex = format!("{:0>64}", format!("{:016X}", time));

        let message_hexstring = encode_packed(&[
            Token::Bytes(hex::decode(selector).unwrap().into()),
            Token::String(name.into()),
            Token::Bytes(hex::decode(amount_padded_hex).unwrap().into()),
            Token::Address(user.parse().unwrap()),
            Token::Bytes(hex::decode(timestamp_padded_hex).unwrap().into()),
        ])
        .unwrap();
        let message_bytes = hex::encode(&message_hexstring);

        let signature = Signer::sign_message(&signer, &message_hexstring)
            .await
            .unwrap();
        (time, message_bytes, signature)
    }

    pub async fn sign_register(
        name: &str,
        user: &str,
        signer: impl Signer,
    ) -> (u64, String, Signature) {
        Self::_sign_base_requset(
            Self::SELECTOR_REGISTER,
            name,
            0,
            user,
            Self::_now_timestamp(),
            signer,
        )
        .await
    }

    pub async fn sign_buy_share(
        name: &str,
        share_num: u128,
        user: &str,
        signer: impl Signer,
    ) -> (u64, String, Signature) {
        Self::_sign_base_requset(
            Self::SELECTOR_BUY,
            name,
            share_num,
            user,
            Self::_now_timestamp(),
            signer,
        )
        .await
    }

    pub async fn sign_sell_share(
        name: &str,
        share_num: u128,
        user: &str,
        signer: impl Signer,
    ) -> (u64, String, Signature) {
        Self::_sign_base_requset(
            Self::SELECTOR_SELL,
            name,
            share_num,
            user,
            Self::_now_timestamp(),
            signer,
        )
        .await
    }

    pub async fn sign_renew_ownership(
        name: &str,
        token_amount: u128,
        user: &str,
        signer: impl Signer,
    ) -> (u64, String, Signature) {
        Self::_sign_base_requset(
            Self::SELECTOR_RENEW,
            name,
            token_amount,
            user,
            Self::_now_timestamp(),
            signer,
        )
        .await
    }
}
