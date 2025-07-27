// Test visibility inheritance for generated builders
use type_state_builder::TypeStateBuilder;

// Test 1: Public struct should generate public builders
#[derive(TypeStateBuilder)]
pub struct PublicConfig {
    #[builder(required)]
    pub name: String,
    pub value: Option<i32>,
}

// Test 2: Private struct should generate private builders
#[derive(TypeStateBuilder)]
struct PrivateConfig {
    #[builder(required)]
    name: String,
    value: Option<i32>,
}

// Test 3: pub(crate) struct should generate pub(crate) builders
#[derive(TypeStateBuilder)]
pub(crate) struct CrateConfig {
    #[builder(required)]
    name: String,
    value: Option<i32>,
}

// Test 4: pub(super) struct should generate pub(super) builders
mod test_module {
    use type_state_builder::TypeStateBuilder;

    #[derive(TypeStateBuilder)]
    pub(super) struct SuperConfig {
        #[builder(required)]
        pub(super) name: String,
        pub(super) value: Option<i32>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_struct_builder_works_across_modules() {
        // This should work - public struct with public builders
        let config = PublicConfig::builder()
            .name("test".to_string())
            .value(Some(42))
            .build();

        assert_eq!(config.name, "test");
        assert_eq!(config.value, Some(42));
    }

    #[test]
    fn test_private_struct_builder_works_within_module() {
        // This should work - private struct with private builders in same module
        let config = PrivateConfig::builder()
            .name("test".to_string())
            .value(Some(42))
            .build();

        assert_eq!(config.name, "test");
        assert_eq!(config.value, Some(42));
    }

    #[test]
    fn test_crate_struct_builder_works_within_crate() {
        // This should work - pub(crate) struct with pub(crate) builders in same crate
        let config = CrateConfig::builder()
            .name("test".to_string())
            .value(Some(42))
            .build();

        assert_eq!(config.name, "test");
        assert_eq!(config.value, Some(42));
    }

    #[test]
    fn test_super_struct_builder_works_within_super() {
        // This should work - pub(super) struct with pub(super) builders
        let config = test_module::SuperConfig::builder()
            .name("test".to_string())
            .value(Some(42))
            .build();

        assert_eq!(config.name, "test");
        assert_eq!(config.value, Some(42));
    }

    #[test]
    fn test_type_state_transitions_respect_visibility() {
        // Test that intermediate builder states also respect visibility
        let builder = PublicConfig::builder();
        let builder_with_name = builder.name("test".to_string());
        let config = builder_with_name.value(Some(42)).build();

        assert_eq!(config.name, "test");
        assert_eq!(config.value, Some(42));
    }

    #[test]
    fn test_regular_builder_visibility_inheritance() {
        // Test that regular builders (optional-only) also inherit visibility
        #[derive(TypeStateBuilder)]
        pub struct PublicOptionalOnly {
            value: Option<i32>,
            name: Option<String>,
        }

        let config = PublicOptionalOnly::builder()
            .value(Some(42))
            .name(Some("test".to_string()))
            .build();

        assert_eq!(config.value, Some(42));
        assert_eq!(config.name, Some("test".to_string()));
    }
}
