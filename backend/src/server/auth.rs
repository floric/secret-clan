use super::app_context::AppContext;
use crate::model::Player;
use hmac::{Hmac, NewMac};
use jwt::{AlgorithmType, Error, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::result::Result;

pub fn generate_jwt_token(player: &Player, secret: &str) -> String {
    let key = init_key(secret);
    let header = Header {
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    };
    let mut claims = BTreeMap::new();
    claims.insert(String::from("sub"), String::from(player.id()));
    claims.insert(String::from("name"), String::from(player.name()));
    claims.insert(String::from("game"), String::from(player.game_token()));

    let jwt_token = Token::new(header, claims).sign_with_key(&key).unwrap();

    String::from(jwt_token.as_str())
}

pub fn extract_verified_token(
    token_str: &str,
    secret: &str,
) -> Result<Token<Header, BTreeMap<String, String>, jwt::token::Verified>, Error> {
    let key = init_key(secret);
    let raw_token: &str = &token_str.replace("Bearer ", "");
    let token: Result<Token<Header, BTreeMap<String, String>, jwt::token::Verified>, Error> =
        VerifyWithKey::verify_with_key(raw_token, &key);

    token
}

pub fn extract_verified_id(authorization: &str, ctx: &AppContext) -> Option<String> {
    extract_verified_token(&authorization, &ctx.config().auth_secret)
        .ok()
        .and_then(|token| token.claims().get("sub").map(String::from))
}

fn init_key(secret: &str) -> Hmac<Sha256> {
    Hmac::new_varkey(secret.as_bytes()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{extract_verified_id, extract_verified_token, generate_jwt_token};
    use crate::{model::Player, server::app_context::AppContext};

    const SECRET: &str = "super-secret";

    #[test]
    fn should_create_token() {
        let player = Player::new("game");
        let token = generate_jwt_token(&player, SECRET);

        // payload contains dynamic ID of user, so we can't check the payload for equalness
        assert!(token.starts_with("eyJhbGciOiJIUzI1NiJ9"));
    }

    #[test]
    fn should_verify_token() {
        let token = extract_verified_token("eyJhbGciOiJIUzI1NiJ9.eyJnYW1lIjoiZ2FtZSIsIm5hbWUiOiJSYW5kb20gRHVkZSIsInN1YiI6Ik1sbEo3b2VDWTRSR3cyTTZ0SUhCWiJ9.LD1Z9u9G9LFUtqweQCikRi0NQs8SVF6Ri9f3weCKCX4", "super-secret");

        assert_eq!(token.unwrap().claims().get("name").unwrap(), "Random Dude");
    }

    #[tokio::test]
    async fn should_extract_verified_id() {
        let ctx = AppContext::init();
        let id = extract_verified_id("eyJhbGciOiJIUzI1NiJ9.eyJnYW1lIjoiZ2FtZSIsIm5hbWUiOiJSYW5kb20gRHVkZSIsInN1YiI6Ik1sbEo3b2VDWTRSR3cyTTZ0SUhCWiJ9.LD1Z9u9G9LFUtqweQCikRi0NQs8SVF6Ri9f3weCKCX4", &ctx);

        assert_eq!(id.unwrap(), "MllJ7oeCY4RGw2M6tIHBZ");
    }

    #[tokio::test]
    async fn should_not_extract_unverified_id() {
        let ctx = AppContext::init();
        let id = extract_verified_id("eyJhbGciOiJIUzI1NiJ9.eyJnYW1lIjoiZ2FtZSIsIm5hbWUiOiJSYW5kb20gRHVkZSIsInN1YiI6Ik1sbEo3b2VDWTRSR3cyTTZ0SUhCWiJ9.LD1Z9u9G9LFUtqweQCikRi0NQs8SVF6Ri9f3weCKCX3", &ctx);

        assert!(id.is_none());
    }

    #[test]
    fn should_create_and_verify_token() {
        let player = Player::new("game");
        let token_str = generate_jwt_token(&player, SECRET);
        let token = extract_verified_token(&token_str, SECRET);
        assert!(!token.unwrap().claims().get("name").unwrap().is_empty());
    }

    #[test]
    fn should_fail_to_verify_token() {
        let token = extract_verified_token("eyJhbGciOiJIUzI1NiJ9.XYZ.ABC", "super-secret");

        assert!(token.is_err());
    }
}
