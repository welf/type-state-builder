// Test the improved type names with PascalCase for snake_case fields
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
pub struct LanguageConfig {
    #[builder(required)]
    pub language_id: String,

    #[builder(required)]
    pub fqn_separator: String,

    pub enabled: Option<bool>,
}

// This should now generate type names like:
// - LanguageConfigBuilder_MissingLanguageId_MissingFqnSeparator
// - LanguageConfigBuilder_HasLanguageId_MissingFqnSeparator
// - LanguageConfigBuilder_MissingLanguageId_HasFqnSeparator
// - LanguageConfigBuilder_HasLanguageId_HasFqnSeparator

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_improved_type_names_with_snake_case_fields() {
        // This should work without any "no method named `build`" errors
        let config = LanguageConfig::builder()
            .language_id("en".to_string())
            .fqn_separator(".".to_string())
            .enabled(Some(true))
            .build();

        assert_eq!(config.language_id, "en");
        assert_eq!(config.fqn_separator, ".");
        assert_eq!(config.enabled, Some(true));
    }

    #[test]
    fn test_multiple_snake_case_fields() {
        #[derive(TypeStateBuilder)]
        pub struct DatabaseConfig {
            #[builder(required)]
            pub connection_string: String,

            #[builder(required)]
            pub max_pool_size: i32,

            #[builder(required)]
            pub retry_timeout_ms: u64,
        }

        // This should generate readable type names like:
        // DatabaseConfigBuilder_HasConnectionString_HasMaxPoolSize_HasRetryTimeoutMs
        let config = DatabaseConfig::builder()
            .connection_string("postgresql://localhost".to_string())
            .max_pool_size(10)
            .retry_timeout_ms(5000)
            .build();

        assert_eq!(config.connection_string, "postgresql://localhost");
        assert_eq!(config.max_pool_size, 10);
        assert_eq!(config.retry_timeout_ms, 5000);
    }
}
