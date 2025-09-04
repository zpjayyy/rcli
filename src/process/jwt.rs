use crate::cli::jwt::{SignOpts, VerifyOpts};
use anyhow::{Ok, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

pub fn sign(opts: SignOpts) -> Result<String> {
    let now = Utc::now();
    let exp_time = now + Duration::seconds(opts.exp as i64);

    let claims = Claims {
        aud: opts.aud.clone().unwrap_or_else(|| "default".to_string()),
        exp: exp_time.timestamp() as usize,
        iat: now.timestamp() as usize,
        iss: opts.iss.clone().unwrap_or_else(|| "rcli".to_string()),
        nbf: now.timestamp() as usize,
        sub: opts.sub,
    };

    let header = Header::new(Algorithm::HS256);
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(opts.secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify(opts: VerifyOpts) -> Result<String> {
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(
        &opts.token,
        &DecodingKey::from_secret(opts.secret.as_ref()),
        &validation,
    )?;

    let claims = token_data.claims;
    let now = Utc::now().timestamp() as usize;

    // 检查是否过期
    if claims.exp < now {
        return Err(anyhow::anyhow!("Token has expired"));
    }

    // 检查是否过早（未来时间）
    if claims.iat > now {
        return Err(anyhow::anyhow!("Token issued in the future"));
    }

    // 返回验证成功的消息和claims信息
    let result = serde_json::json!({
        "valid": true,
        "claims": {
            "sub": claims.sub,
            "aud": claims.aud,
            "exp": claims.exp,
            "iat": claims.iat,
            "iss": claims.iss,
            "nbf": claims.nbf
        }
    });

    Ok(result.to_string())
}
