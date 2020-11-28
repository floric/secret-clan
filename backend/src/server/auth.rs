use crate::model::player::Player;
use hmac::{Hmac, NewMac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

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

pub fn verify_jwt_token(
    token_str: &str,
    secret: &str,
) -> Token<Header, BTreeMap<String, String>, jwt::token::Verified> {
    let key = init_key(secret);
    let token: Token<Header, BTreeMap<String, String>, jwt::token::Verified> =
        VerifyWithKey::verify_with_key(token_str, &key).unwrap();

    token
}

fn init_key(secret: &str) -> Hmac<Sha256> {
    Hmac::new_varkey(secret.as_bytes()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::model::player::Player;
    use crate::server::app_context::AppContext;

    use super::{generate_jwt_token, verify_jwt_token};

    const SECRET: &str = "super-secret";

    fn init_ctx() -> AppContext {
        AppContext::init()
    }

    #[test]
    fn should_create_token() {
        init_ctx();

        let player = Player::new("Random Dude", "game");
        let token = generate_jwt_token(&player, SECRET);

        // payload contains dynamic ID of user, so we can't check the payload for equalness
        assert!(token.starts_with("eyJhbGciOiJIUzI1NiJ9"));
    }
    #[test]
    fn should_verify_token() {
        init_ctx();

        let token = verify_jwt_token("eyJhbGciOiJIUzI1NiJ9.eyJnYW1lIjoiZ2FtZSIsIm5hbWUiOiJSYW5kb20gRHVkZSIsInN1YiI6Ik1sbEo3b2VDWTRSR3cyTTZ0SUhCWiJ9.LD1Z9u9G9LFUtqweQCikRi0NQs8SVF6Ri9f3weCKCX4", "super-secret");

        assert_eq!(token.claims().get("name").unwrap(), "Random Dude");
    }
    #[test]
    fn should_create_and_verify_token() {
        init_ctx();

        let player = Player::new("Random Dude", "game");
        let token_str = generate_jwt_token(&player, SECRET);
        let token = verify_jwt_token(&token_str, SECRET);
        assert_eq!(token.claims().get("name").unwrap(), "Random Dude");
    }
}