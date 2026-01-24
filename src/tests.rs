//! Testes Unitários para Clios Shell
//! 
//! Execute com: cargo test

#[cfg(test)]
mod tests {
    // =========================================================================
    // TESTES DE EXPANSÃO
    // =========================================================================

    #[test]
    fn test_expand_variables_simple() {
        use std::env;
        unsafe {
            env::set_var("TEST_VAR", "hello");
        }
        
        let tokens = vec!["$TEST_VAR".to_string()];
        let result = crate::expansion::expand_variables(tokens);
        
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_expand_variables_with_braces() {
        use std::env;
        unsafe {
            env::set_var("TEST_VAR", "hello");
        }
        
        let tokens = vec!["${TEST_VAR}".to_string()];
        let result = crate::expansion::expand_variables(tokens);
        
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_expand_variables_in_string() {
        use std::env;
        unsafe {
            env::set_var("TESTUSER", "myname");
        }
        
        let tokens = vec!["prefix-$TESTUSER-suffix".to_string()];
        let result = crate::expansion::expand_variables(tokens);
        
        assert!(result[0].contains("myname"), "Expected result to contain 'myname', got: {}", result[0]);
    }

    #[test]
    fn test_expand_tilde() {
        use std::env;
        let home = env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
        
        let tokens = vec!["~".to_string()];
        let result = crate::expansion::expand_tilde(tokens);
        
        assert_eq!(result[0], home);
    }

    #[test]
    fn test_expand_tilde_with_path() {
        use std::env;
        let home = env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
        
        let tokens = vec!["~/Documents".to_string()];
        let result = crate::expansion::expand_tilde(tokens);
        
        assert_eq!(result[0], format!("{}/Documents", home));
    }

    #[test]
    fn test_split_logical_and_simple() {
        let input = "echo hello && echo world";
        let result = crate::expansion::split_logical_and(input);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].trim(), "echo hello");
        assert_eq!(result[1].trim(), "echo world");
    }

    #[test]
    fn test_split_logical_and_with_quotes() {
        let input = r#"echo "a && b" && echo test"#;
        let result = crate::expansion::split_logical_and(input);
        
        assert_eq!(result.len(), 2);
        assert!(result[0].contains("a && b"));
    }

    #[test]
    fn test_split_logical_and_no_split() {
        let input = r#"echo "test && test2""#;
        let result = crate::expansion::split_logical_and(input);
        
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_expand_alias_simple() {
        use std::collections::HashMap;
        let mut aliases = HashMap::new();
        aliases.insert("ll".to_string(), "ls -la".to_string());
        
        let input = "ll /tmp";
        let result = crate::expansion::expand_alias_string(input, &aliases);
        
        assert_eq!(result, "ls -la /tmp");
    }

    #[test]
    fn test_expand_alias_no_match() {
        use std::collections::HashMap;
        let aliases = HashMap::new();
        
        let input = "ls -la";
        let result = crate::expansion::expand_alias_string(input, &aliases);
        
        assert_eq!(result, "ls -la");
    }

    // =========================================================================
    // TESTES DE PIPELINE
    // =========================================================================

    #[test]
    fn test_parse_redirection_stdout() {
        let tokens = vec![
            "echo".to_string(),
            "test".to_string(),
            ">".to_string(),
            "/tmp/test_output.txt".to_string()
        ];
        
        let (clean, stdout_file, stderr_file) = crate::pipeline::parse_redirection(&tokens);
        
        assert_eq!(clean, vec!["echo", "test"]);
        assert!(stdout_file.is_some());
        assert!(stderr_file.is_none());
    }

    #[test]
    fn test_parse_redirection_stderr() {
        let tokens = vec![
            "ls".to_string(),
            "/nonexistent".to_string(),
            "2>".to_string(),
            "/tmp/test_error.txt".to_string()
        ];
        
        let (clean, stdout_file, stderr_file) = crate::pipeline::parse_redirection(&tokens);
        
        assert_eq!(clean, vec!["ls", "/nonexistent"]);
        assert!(stdout_file.is_none());
        assert!(stderr_file.is_some());
    }

    #[test]
    fn test_parse_redirection_both() {
        let tokens = vec![
            "ls".to_string(),
            "/tmp".to_string(),
            ">".to_string(),
            "/tmp/out.txt".to_string(),
            "2>".to_string(),
            "/tmp/err.txt".to_string()
        ];
        
        let (clean, stdout_file, stderr_file) = crate::pipeline::parse_redirection(&tokens);
        
        assert_eq!(clean, vec!["ls", "/tmp"]);
        assert!(stdout_file.is_some());
        assert!(stderr_file.is_some());
    }

    // =========================================================================
    // TESTES DE SHELL
    // =========================================================================

    #[test]
    fn test_shell_creation() {
        use crate::config::CliosConfig;
        let config = CliosConfig::default();
        let shell = crate::shell::CliosShell::new(config);
        
        assert_eq!(shell.last_exit_code, 0);
        assert!(shell.aliases.is_empty());
        assert!(shell.previous_dir.is_none());
    }

    // =========================================================================
    // TESTES DE SUBSHELLS
    // =========================================================================

    #[test]
    fn test_expand_subshells_simple() {
        let input = "echo $(echo test)";
        let result = crate::expansion::expand_subshells(input);
        
        // O resultado deve conter "test" expandido
        assert!(result.contains("test"));
    }

    #[test]
    fn test_expand_subshells_empty() {
        let input = "echo $()";
        let result = crate::expansion::expand_subshells(input);
        
        // Deve processar sem travar
        assert!(result.contains("echo"));
    }

    #[test]
    fn test_expand_subshells_unclosed() {
        let input = "echo $(echo test";
        let result = crate::expansion::expand_subshells(input);
        
        // Deve retornar algo sem travar
        assert!(result.contains("echo"));
    }

    // =========================================================================
    // TESTES DE PROTEÇÃO CONTRA RECURSÃO
    // =========================================================================

    #[test]
    fn test_alias_recursive_protection() {
        use std::collections::HashMap;
        let mut aliases = HashMap::new();
        // Alias que se refere a si mesmo
        aliases.insert("ls".to_string(), "ls -la".to_string());
        
        let input = "ls";
        let result = crate::expansion::expand_alias_string(input, &aliases);
        
        // Deve detectar recursão e retornar original
        assert_eq!(result, "ls");
    }

    #[test]
    fn test_alias_deep_recursion() {
        use std::collections::HashMap;
        let mut aliases = HashMap::new();
        aliases.insert("a".to_string(), "b".to_string());
        aliases.insert("b".to_string(), "c".to_string());
        aliases.insert("c".to_string(), "d".to_string());
        // ... muitos níveis
        
        let input = "a";
        let result = crate::expansion::expand_alias_string(input, &aliases);
        
        // Deve parar antes de overflow
        assert!(!result.is_empty());
    }
}
