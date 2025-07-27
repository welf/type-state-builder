// Integration test for visibility cross-module usage
// This simulates the user's real-world scenario

mod config_module {
    use type_state_builder::TypeStateBuilder;

    #[derive(TypeStateBuilder, Debug, PartialEq)]
    pub struct LanguageConfig {
        #[builder(required)]
        pub language_id: String,

        #[builder(required)]
        pub fqn_separator: String,

        pub enabled: Option<bool>,
    }
}

mod validator_module {
    use super::config_module::LanguageConfig;

    pub fn create_language_config() -> LanguageConfig {
        // This should now work without visibility errors
        // The intermediate builder types should be public because LanguageConfig is public
        LanguageConfig::builder()
            .language_id("en".to_string())
            .fqn_separator(".".to_string())
            .enabled(Some(true))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_module_builder_usage() {
        let config = validator_module::create_language_config();

        assert_eq!(config.language_id, "en");
        assert_eq!(config.fqn_separator, ".");
        assert_eq!(config.enabled, Some(true));
    }
}
