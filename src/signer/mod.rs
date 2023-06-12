use async_trait::async_trait;
use coinstr_core::{nostr_sdk, nostr_sdk::{secp256k1, secp256k1::XOnlyPublicKey,Event, UnsignedEvent}};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use secp256k1::schnorr::Signature;

use wasm_bindgen::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("get public key error: {0}")]
    GetPublicKey(String),
    #[error("encryption error: {0}")]
    Encrypt(String),
    #[error("decryption error: {0}")]
    Decrypt(String),
    #[error("sign event error: {0}")]
    SignEvent(String),
    #[error(transparent)]
    Secp256k1(#[from] secp256k1::Error),
    #[error(transparent)]
    Unsigned(#[from] nostr_sdk::prelude::unsigned::Error),
    #[error(transparent)]
    Serde(#[from] serde_wasm_bindgen::Error),
    
}

#[async_trait(?Send)]
pub trait Signer {
    async fn public_key(&self) -> Result<XOnlyPublicKey, Error>;
    async fn encrypt(&self, public_key: &XOnlyPublicKey, plaintext: &str) -> Result<String, Error>;
    async fn decrypt(&self, public_key: &XOnlyPublicKey, ciphertext: &str) -> Result<String, Error>;
    async fn sign_event(&self, event: UnsignedEvent) -> Result<Event, Error>;
}

#[wasm_bindgen(module = "/wasm-js-api.js")]
extern "C" {

    #[wasm_bindgen(catch)]
    async fn get_public_key() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn encrypt(pubkey: &str, plaintext: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn decrypt(pubkey: &str, plaintext: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn sign_event(event: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
pub struct JavascriptError {
    pub message: String,
}

impl From<JsValue> for JavascriptError {
    fn from(val: JsValue) -> Self {
        if let Ok(err) = serde_wasm_bindgen::from_value(val) {
            err
        } else {
            JavascriptError {
                message: "failed converting error to JavascriptError".to_string(),
            }
        }
    }
}

impl fmt::Display for JavascriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct WebExtensionSigner;

#[async_trait(?Send)]
impl Signer for WebExtensionSigner {
    async fn public_key(&self) -> Result<XOnlyPublicKey, Error> {
        match get_public_key().await {
            Ok(key) => Ok(XOnlyPublicKey::from_str(&key.as_string().unwrap())?),
            Err(err) => {
                let err: JavascriptError = err.into();
                Err(Error::GetPublicKey(err.to_string()))
            }
        }
    }

    async fn encrypt(&self, public_key: &XOnlyPublicKey, plaintext: &str) -> Result<String, Error> {
        match encrypt(&public_key.to_string(), plaintext).await {
            Ok(ciphertext) => Ok(ciphertext.as_string().unwrap()),
            Err(err) => {
                let err: JavascriptError = err.into();
                Err(Error::Encrypt(err.to_string()))
            }
        }
    }

    async fn decrypt(&self, public_key: &XOnlyPublicKey, ciphertext: &str) -> Result<String, Error> {
        match decrypt(&public_key.to_string(), ciphertext).await {
            Ok(plaintext) => Ok(plaintext.as_string().unwrap()),
            Err(err) => {
                let err: JavascriptError = err.into();
                Err(Error::Decrypt(err.to_string()))
            }
        }
    }

    async fn sign_event(&self, event: UnsignedEvent) -> Result<Event, Error> {
        match sign_event(serde_wasm_bindgen::to_value(&event)?).await {
            Ok(sig) => {
              let event = event.add_signature(Signature::from_str(&sig.as_string().unwrap())?)?;
              Ok(event)
            },
            Err(err) => {
                let err: JavascriptError = err.into();
                Err(Error::SignEvent(err.to_string()))
            }
        }
    }
}
