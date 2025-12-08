use ::serde::{Deserialize, Serialize};
use ::std::collections::BTreeMap;

/// Represents the persistent configuration of the application.
///
/// This struct maps directly to the JSON configuration file.
/// It uses `BTreeMap` for `addresses` and `providers` to ensure that keys are
/// sorted alphabetically when the file is saved. This prevents arbitrary reordering
/// of lines in the config file, making it friendlier for version control and human reading.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    /// A collection of location aliases.
    ///
    /// Maps a short alias (e.g., "home") to a specific location query (e.g., "London, UK").
    #[serde(default)]
    pub addresses: BTreeMap<String, String>,

    /// The alias to use when no specific location is provided in the arguments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_alias: Option<String>,

    /// Configuration for specific weather providers.
    ///
    /// Maps a provider ID (e.g., "ow") to its specific settings (API keys, etc.).
    #[serde(default)]
    pub providers: BTreeMap<String, ProviderConfig>,

    /// The ID of the provider to use by default if none is specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_provider: Option<String>,
}

impl Default for Settings {
    /// Creates a default configuration with a mock provider pre-configured.
    fn default() -> Self {
        let mut providers = BTreeMap::new();
        providers.insert(
            "mock".to_string(),
            ProviderConfig {
                key: Some("mock-key".to_string()),
            },
        );
        providers.insert(
            "grpc".to_string(),
            ProviderConfig {
                key: Some("grpc-mock-key".to_string()),
            },
        );

        Self {
            addresses: BTreeMap::new(),
            default_alias: None,
            providers,
            default_provider: None,
        }
    }
}

/// Configuration options for a specific weather provider.
#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
pub struct ProviderConfig {
    /// The API key required to authenticate with the provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();

        assert!(settings.addresses.is_empty());
        assert_eq!(settings.default_alias, None);
        assert_eq!(settings.default_provider, None);

        // Mock provider should be present by default
        assert!(settings.providers.contains_key("mock"));
        assert_eq!(
            settings.providers["mock"].key,
            Some("mock-key".to_string())
        );
    }

    #[test]
    fn test_serialization_skip_none() {
        let settings = Settings {
            addresses: BTreeMap::new(),
            default_alias: None,
            providers: BTreeMap::new(),
            default_provider: None,
        };

        let json_output = serde_json::to_string(&settings).unwrap();

        // Should produce empty objects because BTreeMaps are empty and Options are None (skipped)
        assert_eq!(json_output, r#"{"addresses":{},"providers":{}}"#);
    }

    #[test]
    fn test_serialization_full() {
        let mut addresses = BTreeMap::new();
        addresses.insert("home".to_string(), "London".to_string());

        let mut providers = BTreeMap::new();
        providers.insert(
            "ow".to_string(),
            ProviderConfig {
                key: Some("12345".to_string()),
            },
        );

        let settings = Settings {
            addresses,
            default_alias: Some("home".to_string()),
            providers,
            default_provider: Some("ow".to_string()),
        };

        let json_value: serde_json::Value = serde_json::to_value(&settings).unwrap();

        assert_eq!(json_value["default_alias"], "home");
        assert_eq!(json_value["default_provider"], "ow");
        assert_eq!(json_value["addresses"]["home"], "London");
        assert_eq!(json_value["providers"]["ow"]["key"], "12345");
    }

    #[test]
    fn test_deserialization_partial() {
        // Simulating a config file that might be missing some fields (they should use defaults)
        let json_input = json!({
            "addresses": {
                "work": "Berlin"
            }
        });

        let settings: Settings = serde_json::from_value(json_input).unwrap();

        assert_eq!(settings.addresses.get("work"), Some(&"Berlin".to_string()));
        assert_eq!(settings.default_alias, None);
        // Providers should be empty map (default for BTreeMap) because we didn't use Settings::default() as base here,
        // but serde's Default trait behavior for the field itself.
        assert!(settings.providers.is_empty());
    }

    #[test]
    fn test_btreemap_ordering() {
        let mut settings = Settings::default();
        settings.addresses.insert("z".to_string(), "Last".to_string());
        settings.addresses.insert("a".to_string(), "First".to_string());

        let json_output = serde_json::to_string(&settings).unwrap();

        // In JSON string, "a" should appear before "z" due to BTreeMap
        let a_pos = json_output.find("a").unwrap();
        let z_pos = json_output.find("z").unwrap();

        assert!(a_pos < z_pos, "Keys should be sorted alphabetically");
    }
}