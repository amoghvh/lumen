use std::sync::Arc;
use tokio::sync::RwLock;

pub struct SecurityEngine {
    rulebook: Arc<RwLock<Vec<String>>>,
}

impl SecurityEngine {
    pub fn new(rules: Vec<String>) -> Self {
        Self {
            rulebook: Arc::new(RwLock::new(rules)),
        }
    }
    
    pub async fn check_threat(&self, log: &str) -> bool {
        let rules = self.rulebook.read().await;
        rules.iter().any(|pattern| {
            log.to_lowercase().contains(&pattern.to_lowercase())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_sql_injection_detection() {
        let engine = SecurityEngine::new(vec!["SELECT".to_string()]);
        assert!(engine.check_threat("SELECT * FROM users").await);
        assert!(!engine.check_threat("normal login").await);
    }
    
    #[tokio::test]
    async fn test_xss_detection() {
        let engine = SecurityEngine::new(vec!["<script>".to_string()]);
        assert!(engine.check_threat("<script>alert(1)</script>").await);
        assert!(!engine.check_threat("plain text").await);
    }
    
    #[tokio::test]
    async fn test_multiple_rules() {
        let engine = SecurityEngine::new(vec![
            "SELECT".to_string(),
            "DROP".to_string(),
            "<script>".to_string(),
        ]);
        
        assert!(engine.check_threat("SELECT id FROM users").await);
        assert!(engine.check_threat("DROP TABLE users").await);
        assert!(engine.check_threat("<script>").await);
        assert!(!engine.check_threat("User login").await);
    }
    
    #[tokio::test]
    async fn test_case_insensitive() {
        let engine = SecurityEngine::new(vec!["select".to_string()]);
        assert!(engine.check_threat("SELECT").await);
        assert!(engine.check_threat("Select").await);
        assert!(engine.check_threat("select").await);
    }
    
    #[tokio::test]
    async fn test_pii_detection() {
        let engine = SecurityEngine::new(vec!["cc_number".to_string(), "password".to_string()]);
        assert!(engine.check_threat("cc_number=4111111111111111").await);
        assert!(engine.check_threat("password=secret123").await);
    }
}
