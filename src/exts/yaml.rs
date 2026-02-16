use rustc_hash::FxHashMap;
use serde::{Serialize, Serializer};

/// Frontmatter 值类型
#[derive(Debug, Clone, PartialEq)]
pub enum YamlValue {
    String(String),
    Bool(bool),
    Integer(i64),
    Float(f64),
    List(Vec<YamlValue>),
    Null,
}

// Custom serialization to flatten the enum variants
impl Serialize for YamlValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            YamlValue::String(s) => serializer.serialize_str(s),
            YamlValue::Bool(b) => serializer.serialize_bool(*b),
            YamlValue::Integer(i) => serializer.serialize_i64(*i),
            YamlValue::Float(f) => serializer.serialize_f64(*f),
            YamlValue::List(list) => list.serialize(serializer),
            YamlValue::Null => serializer.serialize_none(),
        }
    }
}

/// Frontmatter 键值对映射
pub type YamlMap = FxHashMap<String, YamlValue>;

/// 解析 frontmatter YAML 文本为键值对映射
pub fn parse_yaml(input: &str) -> Option<YamlMap> {
    let mut map = FxHashMap::default();
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        // Parse key-value pair
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value_part = line[colon_pos + 1..].trim();

            let value = if value_part.is_empty() {
                // Check if next lines are indented list items
                if i + 1 < lines.len() && lines[i + 1].trim_start().starts_with('-') {
                    let (list_value, consumed) = parse_indented_list(&lines[i + 1..]);
                    i += consumed;
                    list_value
                } else {
                    YamlValue::Null
                }
            } else {
                parse_value(value_part)
            };

            map.insert(key, value);
        }

        i += 1;
    }

    if map.is_empty() { None } else { Some(map) }
}

/// 解析缩进列表
fn parse_indented_list(lines: &[&str]) -> (YamlValue, usize) {
    let mut items = Vec::new();
    let mut consumed = 0;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with('-') {
            let item_text = trimmed[1..].trim();
            items.push(parse_value(item_text));
            consumed += 1;
        } else if trimmed.is_empty() {
            consumed += 1;
            continue;
        } else {
            break;
        }
    }

    (YamlValue::List(items), consumed)
}

/// 解析单个值
fn parse_value(s: &str) -> YamlValue {
    let s = s.trim();

    // Null
    if s.is_empty() || s == "null" || s == "~" {
        return YamlValue::Null;
    }

    // Boolean
    if s == "true" || s == "True" || s == "TRUE" {
        return YamlValue::Bool(true);
    }
    if s == "false" || s == "False" || s == "FALSE" {
        return YamlValue::Bool(false);
    }

    // Quoted string
    if (s.starts_with('"') && s.ends_with('"') && s.len() >= 2)
        || (s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2)
    {
        return YamlValue::String(s[1..s.len() - 1].to_string());
    }

    // Inline list [item1, item2]
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len() - 1].trim();
        if inner.is_empty() {
            return YamlValue::List(Vec::new());
        }
        let items: Vec<YamlValue> = inner
            .split(',')
            .map(|item| parse_value(item.trim()))
            .collect();
        return YamlValue::List(items);
    }

    // Number (integer or float)
    if let Ok(i) = s.parse::<i64>() {
        return YamlValue::Integer(i);
    }
    if let Ok(f) = s.parse::<f64>() {
        return YamlValue::Float(f);
    }

    // Default to string
    YamlValue::String(s.to_string())
}

/// 将 YamlValue 序列化为 YAML 文本（用于往返测试）
#[cfg(test)]
pub fn serialize_yaml(map: &YamlMap) -> String {
    let mut result = String::new();

    // 对键进行排序以保证输出的确定性（用于测试）
    let mut keys: Vec<_> = map.keys().collect();
    keys.sort();

    for key in keys {
        let value = &map[key];
        result.push_str(key);
        result.push_str(": ");
        serialize_value(value, &mut result, 0);
        result.push('\n');
    }

    result
}

#[cfg(test)]
fn serialize_value(value: &YamlValue, output: &mut String, indent: usize) {
    match value {
        YamlValue::String(s) => {
            // Quote strings that contain special characters or look like other types
            let needs_quotes = s.is_empty()
                || s.trim() != s  // Has leading/trailing whitespace
                || s.trim().is_empty()  // Only whitespace
                || s == "true"
                || s == "false"
                || s == "null"
                || s.parse::<i64>().is_ok()
                || s.parse::<f64>().is_ok()
                || s.contains(':')
                || s.contains('#');

            if needs_quotes {
                output.push('"');
                output.push_str(s);
                output.push('"');
            } else {
                output.push_str(s);
            }
        }
        YamlValue::Bool(b) => output.push_str(if *b { "true" } else { "false" }),
        YamlValue::Integer(i) => output.push_str(&i.to_string()),
        YamlValue::Float(f) => output.push_str(&f.to_string()),
        YamlValue::Null => output.push_str("null"),
        YamlValue::List(items) => {
            if items.is_empty() {
                output.push_str("[]");
            } else {
                // Use inline format for simple lists
                let all_simple = items.iter().all(|item| {
                    matches!(
                        item,
                        YamlValue::String(_)
                            | YamlValue::Bool(_)
                            | YamlValue::Integer(_)
                            | YamlValue::Float(_)
                            | YamlValue::Null
                    )
                });

                if all_simple {
                    output.push('[');
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 {
                            output.push_str(", ");
                        }
                        serialize_value(item, output, indent);
                    }
                    output.push(']');
                } else {
                    // Use indented format for complex lists
                    for item in items {
                        output.push('\n');
                        output.push_str(&"  ".repeat(indent + 1));
                        output.push_str("- ");
                        serialize_value(item, output, indent + 1);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Property-based tests
    use proptest::prelude::*;

    // Feature: performance-optimization, Property 5: YAML 解析往返一致性
    // For any合法的 YamlMap（由随机生成的键值对组成），parse_yaml(serialize_yaml(map)) SHALL 产生与原始 map 等价的结果。
    // Validates: Requirements 4.1, 4.2, 4.3, 4.6, 4.7, 4.9
    proptest! {
        #[test]
        fn prop_yaml_roundtrip_consistency(
            keys in prop::collection::vec("[a-z]{1,10}", 1..5),
            values in prop::collection::vec(yaml_value_strategy(), 1..5)
        ) {
            // Create a YamlMap from generated keys and values
            let mut map = FxHashMap::default();
            for (key, value) in keys.into_iter().zip(values.into_iter()) {
                map.insert(key, value);
            }

            // Serialize and parse back
            let yaml_text = serialize_yaml(&map);
            let parsed = parse_yaml(&yaml_text);

            // Should successfully parse
            prop_assert!(parsed.is_some(), "Failed to parse serialized YAML");
            let parsed_map = parsed.unwrap();

            // Should have same keys
            prop_assert_eq!(map.len(), parsed_map.len(), "Different number of keys");

            // Check each key-value pair
            for (key, original_value) in &map {
                let parsed_value = parsed_map.get(key);
                prop_assert!(parsed_value.is_some(), "Key '{}' missing in parsed map", key);

                // Compare values (with special handling for floats)
                match (original_value, parsed_value.unwrap()) {
                    (YamlValue::Float(f1), YamlValue::Float(f2)) => {
                        // Handle NaN and infinity specially
                        if f1.is_nan() && f2.is_nan() {
                            // Both NaN - OK
                        } else {
                            prop_assert!((f1 - f2).abs() < 1e-10, "Float values differ: {} vs {}", f1, f2);
                        }
                    }
                    (v1, v2) => {
                        prop_assert_eq!(v1, v2, "Values differ for key '{}'", key);
                    }
                }
            }
        }
    }

    // Strategy for generating YamlValue
    fn yaml_value_strategy() -> impl Strategy<Value = YamlValue> {
        prop_oneof![
            // String values
            "[a-zA-Z0-9 ]{0,20}".prop_map(YamlValue::String),
            // Boolean values
            prop::bool::ANY.prop_map(YamlValue::Bool),
            // Integer values
            (-1000i64..1000i64).prop_map(YamlValue::Integer),
            // Float values (avoid NaN and infinity for simplicity)
            (-1000.0f64..1000.0f64).prop_map(YamlValue::Float),
            // Null
            Just(YamlValue::Null),
            // Simple lists (non-recursive to avoid complexity)
            prop::collection::vec(
                prop_oneof![
                    "[a-zA-Z0-9 ]{0,10}".prop_map(YamlValue::String),
                    prop::bool::ANY.prop_map(YamlValue::Bool),
                    (-100i64..100i64).prop_map(YamlValue::Integer),
                ],
                0..5
            )
            .prop_map(YamlValue::List),
        ]
    }

    // Feature: performance-optimization, Property 6: YAML 非法输入安全性
    // For any 随机生成的字符串（包括非法 YAML 格式），parse_yaml(input) SHALL 返回 None 或 Some(valid_map)，不产生 panic。
    // Validates: Requirements 4.8
    proptest! {
        #[test]
        fn prop_yaml_invalid_input_safety(input in "\\PC{0,100}") {
            // Should not panic on any input
            let result = parse_yaml(&input);

            // If it returns Some, it should be a valid map
            if let Some(map) = result {
                // Just verify it's a valid BTreeMap - no panic
                prop_assert!(map.len() >= 0);
            }
            // If it returns None, that's also fine
        }
    }

    #[test]
    fn test_parse_simple_string() {
        let input = "title: Hello World";
        let result = parse_yaml(input).unwrap();
        assert_eq!(
            result.get("title"),
            Some(&YamlValue::String("Hello World".to_string()))
        );
    }

    #[test]
    fn test_parse_boolean() {
        let input = "draft: true\nexternal: false";
        let result = parse_yaml(input).unwrap();
        assert_eq!(result.get("draft"), Some(&YamlValue::Bool(true)));
        assert_eq!(result.get("external"), Some(&YamlValue::Bool(false)));
    }

    #[test]
    fn test_parse_integer() {
        let input = "count: 42";
        let result = parse_yaml(input).unwrap();
        assert_eq!(result.get("count"), Some(&YamlValue::Integer(42)));
    }

    #[test]
    fn test_parse_float() {
        let input = "price: 3.14";
        let result = parse_yaml(input).unwrap();
        assert_eq!(result.get("price"), Some(&YamlValue::Float(3.14)));
    }

    #[test]
    fn test_parse_quoted_string() {
        let input = r#"message: "Hello: World""#;
        let result = parse_yaml(input).unwrap();
        assert_eq!(
            result.get("message"),
            Some(&YamlValue::String("Hello: World".to_string()))
        );
    }

    #[test]
    fn test_parse_inline_list() {
        let input = "tags: [rust, yaml, parser]";
        let result = parse_yaml(input).unwrap();
        assert_eq!(
            result.get("tags"),
            Some(&YamlValue::List(vec![
                YamlValue::String("rust".to_string()),
                YamlValue::String("yaml".to_string()),
                YamlValue::String("parser".to_string()),
            ]))
        );
    }

    #[test]
    fn test_parse_indented_list() {
        let input = "items:\n  - first\n  - second\n  - third";
        let result = parse_yaml(input).unwrap();
        assert_eq!(
            result.get("items"),
            Some(&YamlValue::List(vec![
                YamlValue::String("first".to_string()),
                YamlValue::String("second".to_string()),
                YamlValue::String("third".to_string()),
            ]))
        );
    }

    #[test]
    fn test_parse_null() {
        let input = "value: null";
        let result = parse_yaml(input).unwrap();
        assert_eq!(result.get("value"), Some(&YamlValue::Null));
    }

    #[test]
    fn test_serialize_simple() {
        let mut map = FxHashMap::default();
        map.insert("title".to_string(), YamlValue::String("Test".to_string()));
        map.insert("count".to_string(), YamlValue::Integer(42));

        let yaml = serialize_yaml(&map);
        assert!(yaml.contains("title: Test"));
        assert!(yaml.contains("count: 42"));
    }

    #[test]
    fn test_roundtrip() {
        let mut map = FxHashMap::default();
        map.insert("title".to_string(), YamlValue::String("Hello".to_string()));
        map.insert("draft".to_string(), YamlValue::Bool(true));
        map.insert("count".to_string(), YamlValue::Integer(42));

        let yaml = serialize_yaml(&map);
        let parsed = parse_yaml(&yaml).unwrap();

        assert_eq!(
            parsed.get("title"),
            Some(&YamlValue::String("Hello".to_string()))
        );
        assert_eq!(parsed.get("draft"), Some(&YamlValue::Bool(true)));
        assert_eq!(parsed.get("count"), Some(&YamlValue::Integer(42)));
    }
}
