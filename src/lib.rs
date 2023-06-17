use wasm_bindgen::prelude::*;
use coinstr_core::bitcoin::Network;
use coinstr_core::nostr_sdk::prelude::{*};
use coinstr_core::{CoinstrClient, policy::Policy};
use coinstr_core::constants::{SHARED_KEY_KIND};
use std::str::FromStr;
use std::time::Duration;
use secp256k1::schnorr::Signature;

const BECH32_SK: &str = "nsec18lkp320pjm7n5eqhk3066uq9akermpffedqa3trn3n7a054h2ems37v3ar";
const DEFAULT_RELAY: &str = "wss://relay.rip";
const TIMEOUT: Option<Duration> = Some(Duration::from_secs(300));

mod signer;

use signer::{Signer, JavascriptError, WebSigner};


#[wasm_bindgen(module = "/wasm-js-api.js")]
extern "C" {

  fn loggear(s: &str);

  fn syncfn(s: &str) -> String;

  async fn asyncfn(s: &str) -> JsValue;

  #[wasm_bindgen(catch)]
  async fn asyncfne(s: &str) -> Result<JsValue, JsValue>;

//   #[wasm_bindgen(catch)]
//   async fn get_public_key() -> Result<JsValue, JsValue>;

//   #[wasm_bindgen(catch)]
//   async fn encrypt(pubkey: &str, plaintext: &str) -> Result<JsValue, JsValue>;

//   #[wasm_bindgen(catch)]
//   async fn decrypt(pubkey: &str, plaintext: &str) -> Result<JsValue, JsValue>;

//   #[wasm_bindgen(catch)]
//   async fn sign_event(event: JsValue) -> Result<JsValue, JsValue>;
}



#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
#[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}


#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn concatenate(left: &str, right: &str) -> String {
    let mut s =  left.to_string();
    s.push_str(right);
    log("This console.log is from wasm!");
    log("This console.log is from wasm! 2");
    let m = syncfn("hola");
    log(m.as_str());
    s
}

#[wasm_bindgen]
pub async fn promise(v: &str) -> Result<String, String> {
    let mut s =  v.to_string();
    s.push_str("promise");
    let m = asyncfn("adios").await;
    log(m.as_string().unwrap().as_str());
    let r = asyncfne("adios").await;
    match r {
        Ok(m) => log(format!("Ok: {}", m.as_string().unwrap().as_str()).as_str()),
        Err(m) => log(format!("Error: {}", m.as_string().unwrap().as_str()).as_str()),
    }   
    Ok(s)
}

#[wasm_bindgen]
pub async fn save_policy(name: &str, description: &str, descriptor: &str ) -> Result<JsValue, JsValue> {
    let secret_key = SecretKey::from_bech32(BECH32_SK).unwrap();
    let keys = Keys::new(secret_key);
    let client =
        CoinstrClient::new(keys, vec![DEFAULT_RELAY.to_string()], Network::Testnet)
            .await
            .unwrap();
    
    let event_id = client.save_policy(name, description, descriptor).await.unwrap();
    Ok(serde_wasm_bindgen::to_value(&event_id)?)
}

#[wasm_bindgen]
pub async fn policies() -> Result<JsValue, JsValue> {
    let secret_key = SecretKey::from_bech32(BECH32_SK).unwrap();
    let keys = Keys::new(secret_key);
    let client =
        CoinstrClient::new(keys, vec![DEFAULT_RELAY.to_string()], Network::Testnet)
            .await
            .unwrap();
    let policies = client.get_policies(TIMEOUT).await.unwrap();
    Ok(serde_wasm_bindgen::to_value(&policies)?)
}

#[wasm_bindgen]
pub async fn contacts() -> Result<JsValue, JsValue> {
    let secret_key = SecretKey::from_bech32(BECH32_SK).unwrap();
    let keys = Keys::new(secret_key);
    let client =
        CoinstrClient::new(keys, vec![DEFAULT_RELAY.to_string()], Network::Testnet)
            .await
            .unwrap();
    let contacts = client.get_contacts(TIMEOUT).await.unwrap();
    Ok(serde_wasm_bindgen::to_value(&contacts)?)
}

#[wasm_bindgen]
pub async fn encrypt_decrypt_test(v: &str) {
    let signer = WebSigner;
    let shared_key = Keys::generate();
    let pubkey = shared_key.public_key();
    log(format!("Public key to encrypt to: {}", pubkey).as_str());
    let r = signer.encrypt(&pubkey, v).await;
    match r {
        Ok(ciphertext) => {
            log(format!("Ok ciphertext: {}", ciphertext).as_str());
            let r = signer.decrypt(&pubkey, &ciphertext).await;
            match r {
                Ok(plaintext) => log(format!("Ok plaintext: {}", plaintext).as_str()),
                Err(err) => log(format!("Error plaintext: {}", err).as_str()),
            }
        },
        Err(err) => log(format!("Error plaintext: {}", err).as_str()),
    }
}

#[wasm_bindgen]
pub async fn sign_event_test() -> Result<(), JsValue> {
    let shared_key = Keys::generate();
    let pubkeyto = shared_key.public_key();
    loggear(format!("pubkeyto: {}", pubkeyto.to_string()).as_str());
    let signer = WebSigner;
    let r = signer.public_key().await;
    if let Ok(pubkey) =  r {
        let tags = vec![Tag::PubKey(pubkeyto, None)];
        let content = "Contenido del evento";
        let uevent  = EventBuilder::new(Kind::TextNote, content, &tags).to_unsigned_event(pubkey);
        match signer.sign_event(uevent).await {
            Ok(event) => {
                loggear(format!("Ok event: {:?}", event).as_str());
                let keys =Keys::from_public_key(pubkey);
                let client = Client::new(&keys);
                client.add_relays(vec![DEFAULT_RELAY.to_string()]).await.unwrap();
                client.connect().await;
                let event_id = client.send_event(event).await.unwrap();
                loggear(format!("Send event {}", event_id).as_str());
            },
            Err(err) => loggear(format!("Error event: {}", err).as_str()),
        }

        Ok(())
    } else  {
        Err(r.err().unwrap().to_string().into())
    }
}



// #[wasm_bindgen]
// pub async fn policies(v: &str) -> Result<Vec<(EventId, Policy)>, String> {
//     let secret_key = SecretKey::from_bech32(BECH32_SK).unwrap();
//     let keys = Keys::new(secret_key);
//     let client =
//         CoinstrClient::new(keys, vec![DEFAULT_RELAY.to_string()], Network::Testnet)
//             .await
//             .unwrap();
//     let policies = client.get_policies(TIMEOUT).await.unwrap();
//     Ok(policies)
// }

