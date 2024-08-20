use anyhow::{Result, Context};
use std::{collections::HashMap, env, fs, path::Path};

fn evaluate_expression(expression: &str) -> String {
    // Check if the expression matches the pattern `${VAR:-default}`
    if let Some((var_name, default_value)) = parse_expression(expression) {
        env::var(var_name).unwrap_or_else(|_| default_value.to_string())
    } else {
        expression.to_string()
    }
}

fn parse_expression(expression: &str) -> Option<(&str, &str)> {
    let mut expression = expression;
    if expression.contains("#") {
        expression = expression.split("#").next().unwrap_or("");
        expression = &expression[0..expression.len() - 2];
        println!("expression: {}", expression);
    }
    if expression.starts_with("${") && expression.ends_with('}') {
        let trimmed = &expression[2..expression.len() - 1];
        if let Some((var_name, default_value)) = trimmed.split_once(":-") {
            return Some((var_name, default_value));
        } else {
            return Some((trimmed, ""));
        }
    } else if expression.starts_with("$") {
        let trimmed = &expression[1..expression.len()];
        return Some((trimmed, ""));
    }
    None
}

fn parse_context(content: String) -> Result<HashMap<String, String>> {
    let mut context = HashMap::new();

    for line in content.lines() {
        if line.starts_with("#") || line.trim().is_empty() {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().trim_start_matches("export ").to_string();
            let value = value.trim().trim_matches('"').to_string();
            let parsed_value = evaluate_expression(value.as_str());
            context.insert(key, parsed_value);
        }
    }

    Ok(context)
}

pub fn read_defaults<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>> {
    let content = fs::read_to_string(path).context("Cannot read file")?;
    let context = parse_context(content)?;
    Ok(context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_empty_file() {
        let content = "";

        let context = parse_context(content.to_string()).unwrap();
        assert_eq!(context, HashMap::new());
    }

    #[test]
    #[serial]
    fn test_one_line() {
        let content = "export PRESET_BASE=\"${PRESET_BASE:-mainnet}\"";

        let context = parse_context(content.to_string()).unwrap();

        let mut expected_context = HashMap::new();
        expected_context.insert("PRESET_BASE".to_string(), "mainnet".to_string());

        assert_eq!(context, expected_context);
    }

    #[test]
    #[serial]
    fn test_ignores_comments() {
        let content = "export PRESET_BASE=\"${PRESET_BASE:-mainnet}\" # Some comment";

        let context = parse_context(content.to_string()).unwrap();

        let mut expected_context = HashMap::new();
        expected_context.insert("PRESET_BASE".to_string(), "mainnet".to_string());

        assert_eq!(context, expected_context);
    }

    #[test]
    #[serial]
    fn test_env_overwrites_current_value() {
        env::set_var("PRESET_BASE", "testnet");
        let content = "export PRESET_BASE=\"${PRESET_BASE:-mainnet}\"";

        let context = parse_context(content.to_string()).unwrap();

        let mut expected_context = HashMap::new();
        expected_context.insert("PRESET_BASE".to_string(), "testnet".to_string());
        env::remove_var("PRESET_BASE");

        assert_eq!(context, expected_context);
    }

    #[test]
    #[serial]
    fn test_env_ignores_commented_variables() {
        let content = "#export EOF_ACTIVATION_EPOCH=\"${EOF_ACTIVATION_EPOCH:-99999}\"";

        let context = parse_context(content.to_string()).unwrap();

        let expected_context = HashMap::new();

        assert_eq!(context, expected_context);
    }

    #[test]
    #[serial]
    fn test_reads_curly_syntax_without_default_value() {
        let content = "export CHAIN_ID=\"${CHAIN_ID}\"";

        let context = parse_context(content.to_string()).unwrap();

        let mut expected_context = HashMap::new();
        expected_context.insert("CHAIN_ID".to_string(), "".to_string());

        assert_eq!(context, expected_context);
    }

    #[test]
    #[serial]
    fn test_reads_curly_syntax_without_default_value_and_evaluates() {
        env::set_var("CHAIN_ID", "1337");

        let content = "export CHAIN_ID=\"${CHAIN_ID}\"";

        let context = parse_context(content.to_string()).unwrap();

        let mut expected_context = HashMap::new();
        expected_context.insert("CHAIN_ID".to_string(), "1337".to_string());

        env::remove_var("CHAIN_ID");

        assert_eq!(context, expected_context);
    }

    #[test]
    #[serial]
    fn test_reads_dollar_syntax_without_default_value() {
        let content = "export CHAIN_ID=\"$CHAIN_ID\"";

        let context = parse_context(content.to_string()).unwrap();

        let mut expected_context = HashMap::new();
        expected_context.insert("CHAIN_ID".to_string(), "".to_string());

        assert_eq!(context, expected_context);
    }

    #[test]
    #[serial]
    fn test_reads_dollar_syntax_without_default_value_and_evaluates() {
        env::set_var("CHAIN_ID", "1337");

        let content = "export CHAIN_ID=\"$CHAIN_ID\"";

        let context = parse_context(content.to_string()).unwrap();

        let mut expected_context = HashMap::new();
        expected_context.insert("CHAIN_ID".to_string(), "1337".to_string());

        env::remove_var("CHAIN_ID");

        assert_eq!(context, expected_context);
    }
}
