use openssl::rsa::{Rsa, Padding};
use openssl::pkey::Public;
use openssl::error::ErrorStack;

const RSA_ENCRYPT_PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA81dCnCKt0NVH7j5Oh2+S
GgEU0aqi5u6sYXemouJWXOlZO3jqDsHYM1qfEjVvCOmeoMNFXYSXdNhflU7mjWP8
jWUmkYIQ8o3FGqMzsMTNxr+bAp0cULWu9eYmycjJwWIxxB7vUwvpEUNicgW7v5nC
wmF5HS33Hmn7yDzcfjfBs99K5xJEppHG0qc+q3YXxxPpwZNIRFn0Wtxt0Muh1U8a
vvWyw03uQ/wMBnzhwUC8T4G5NclLEWzOQExbQ4oDlZBv8BM/WxxuOyu0I8bDUDdu
tJOfREYRZBlazFHvRKNNQQD2qDfjRz484uFs7b5nykjaMB9k/EJAuHjJzGs9MMMW
tQIDAQAB
-----END PUBLIC KEY-----";

fn get_encrypt_key() -> Result<Rsa<Public>, ErrorStack> { Rsa::public_key_from_pem(RSA_ENCRYPT_PUBLIC_KEY.as_bytes()) }

pub fn encrypt_text(text: &str) -> Result<String, ErrorStack> {
    let rsa = get_encrypt_key()?;
    let mut buf = vec![0; rsa.size() as usize];
    rsa.public_encrypt(text.as_bytes(), &mut buf, Padding::PKCS1)?;
    Ok(base64::encode(buf))
}