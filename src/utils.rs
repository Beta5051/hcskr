use std::collections::HashMap;
use std::error::Error;
use std::fmt::Formatter;
use rand::rngs::OsRng;
use rsa::{RsaPublicKey, pkcs8::FromPublicKey, PublicKey, PaddingScheme};
use serde::de::{EnumAccess, MapAccess, SeqAccess, Unexpected, Visitor};
use serde::Deserializer;

const PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA81dCnCKt0NVH7j5Oh2+S
GgEU0aqi5u6sYXemouJWXOlZO3jqDsHYM1qfEjVvCOmeoMNFXYSXdNhflU7mjWP8
jWUmkYIQ8o3FGqMzsMTNxr+bAp0cULWu9eYmycjJwWIxxB7vUwvpEUNicgW7v5nC
wmF5HS33Hmn7yDzcfjfBs99K5xJEppHG0qc+q3YXxxPpwZNIRFn0Wtxt0Muh1U8a
vvWyw03uQ/wMBnzhwUC8T4G5NclLEWzOQExbQ4oDlZBv8BM/WxxuOyu0I8bDUDdu
tJOfREYRZBlazFHvRKNNQQD2qDfjRz484uFs7b5nykjaMB9k/EJAuHjJzGs9MMMW
tQIDAQAB
-----END PUBLIC KEY-----";

pub fn encrypt_plain_text(plain_text: &str) -> Result<String, Box<dyn Error>> {
    let mut rng = OsRng;
    let public_key = RsaPublicKey::from_public_key_pem(PUBLIC_KEY)?;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypt_data = public_key.encrypt(&mut rng, padding, plain_text.as_bytes())?;
    Ok(base64::encode(encrypt_data))
}

pub fn get_endpoint(url_code: Option<&str>, path: &str) -> String {
    format!("https://{}hcs.eduro.go.kr/v2/{}", url_code.unwrap_or(""), path)
}

struct DeserializeStringToBool;

impl<'de> Visitor<'de> for DeserializeStringToBool {
    type Value = bool;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("string")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }
}

pub fn deserialize_string_to_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    deserializer.deserialize_any(DeserializeStringToBool)
}