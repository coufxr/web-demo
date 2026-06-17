use crate::project::error::AppError;

/// 密码哈希
pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| {
        tracing::error!("密码哈希失败: {}", e);
        AppError::internal("服务器内部错误")
    })
}

/// 密码验证
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash).map_err(|e| {
        tracing::error!("密码验证失败: {}", e);
        AppError::internal("服务器内部错误")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_ok() {
        let hash = hash_password("my_password").unwrap();
        assert!(verify_password("my_password", &hash).unwrap());
    }

    #[test]
    fn wrong_password_fails() {
        let hash = hash_password("correct_password").unwrap();
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn different_hashes_for_same_password() {
        let a = hash_password("same").unwrap();
        let b = hash_password("same").unwrap();
        assert_ne!(a, b, "bcrypt should produce different salts each time");
    }

    #[test]
    fn verify_invalid_hash_returns_error() {
        let result = verify_password("pwd", "not_a_valid_hash");
        assert!(result.is_err());
    }
}
