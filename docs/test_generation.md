# Automatic Unit Test Generation with httpmock

Progenitor can automatically generate comprehensive unit tests for your OpenAPI client using the httpmock library. This feature creates type-safe, well-structured tests that cover success cases, error scenarios, and parameter validation.

## Features

### Test Types Generated

1. **Success Tests**: Test each operation with valid parameters and expected success responses
2. **Error Tests**: Test error scenarios with various HTTP error codes (4xx, 5xx)
3. **Parameter Tests**: Test optional parameters, parameter validation, and edge cases
4. **Integration Tests**: Test workflows that combine multiple operations (optional)

### Key Benefits

- **Type Safety**: Generated tests use the same type-safe httpmock extensions as your manual tests
- **Comprehensive Coverage**: Automatically covers all operations, parameters, and response types
- **Maintainable**: Tests are regenerated when your OpenAPI spec changes
- **Customizable**: Configure which types of tests to generate
- **Documentation**: Generated tests include helpful documentation

## Usage

### Basic Setup

Add test generation to your `build.rs`:

```rust
use progenitor::{Generator, TestGenerationConfig};

fn main() {
    let spec = /* load your OpenAPI spec */;
    let mut generator = Generator::default();
    
    // Generate main client and httpmock support first
    // ... (standard generation code)
    
    // Generate comprehensive tests
    let test_config = TestGenerationConfig::default();
    let test_tokens = generator.generate_tests(&spec, "crate", &test_config)
        .expect("Failed to generate tests");
    
    let test_content = prettyplease::unparse(&syn::parse2(test_tokens).unwrap());
    std::fs::write("src/generated_tests.rs", test_content)
        .expect("Failed to write tests");
}
```

### Configuration Options

```rust
let test_config = TestGenerationConfig {
    generate_success_tests: true,      // Generate success case tests
    generate_error_tests: true,        // Generate error case tests  
    generate_parameter_tests: true,    // Generate parameter validation tests
    generate_integration_tests: false, // Generate multi-operation workflows
    test_module_name: "tests".to_string(), // Name of the test module
    include_documentation: true,       // Include doc comments in tests
};
```

### Generated Test Structure

For an operation `create_user`, the generator creates tests like:

```rust
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::httpmock::{MockServerExt, operations};
    use httpmock::prelude::*;

    #[tokio::test]
    /// Test successful create_user with 201 response
    async fn test_create_user_success_201() {
        let server = MockServer::start();
        let client = Client::new(&server.base_url()).unwrap();
        
        let _mock = server.create_user(|when, then| {
            when.body(&types::CreateUserRequest {
                name: "test_value".to_string(),
                email: "test_value".to_string(),
            });
            then.created(&types::User::default());
        });
        
        let result = client
            .create_user()
            .body(&types::CreateUserRequest {
                name: "test_value".to_string(),
                email: "test_value".to_string(),
            })
            .send()
            .await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        // Add specific assertions based on response type
    }

    #[tokio::test]
    /// Test error case for create_user with 400 response
    async fn test_create_user_error_400() {
        let server = MockServer::start();
        let client = Client::new(&server.base_url()).unwrap();
        
        let _mock = server.create_user(|when, then| {
            when.body(&types::CreateUserRequest {
                name: "test_value".to_string(),
                email: "test_value".to_string(),
            });
            then.client_error(400, &types::ErrorResponse::default());
        });
        
        let result = client
            .create_user()
            .body(&types::CreateUserRequest {
                name: "test_value".to_string(),
                email: "test_value".to_string(),
            })
            .send()
            .await;
        
        assert!(result.is_err());
        // Verify specific error type and status code
    }
}
```

## Advanced Usage

### Separate Test Files

Generate different types of tests in separate files:

```rust
// Generate only success tests
let success_config = TestGenerationConfig {
    generate_success_tests: true,
    generate_error_tests: false,
    generate_parameter_tests: false,
    generate_integration_tests: false,
    test_module_name: "success_tests".to_string(),
    include_documentation: true,
};

// Generate only error tests  
let error_config = TestGenerationConfig {
    generate_success_tests: false,
    generate_error_tests: true,
    generate_parameter_tests: false,
    generate_integration_tests: false,
    test_module_name: "error_tests".to_string(),
    include_documentation: true,
};
```

### Integration with Existing Tests

Generated tests complement your manual tests:

```rust
// Your manual tests
#[cfg(test)]
mod manual_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_complex_workflow() {
        // Your custom test logic
    }
}

// Generated tests (included from separate file)
include!("generated_tests.rs");
```

## Dependencies

Add these to your `Cargo.toml`:

```toml
[dev-dependencies]
httpmock = "0.7"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

## Best Practices

1. **Regenerate on Spec Changes**: Use `cargo:rerun-if-changed` in build.rs
2. **Customize Sample Data**: Override the default sample value generation for your domain
3. **Combine with Manual Tests**: Use generated tests as a foundation, add custom tests for complex scenarios
4. **Review Generated Tests**: Periodically review generated tests to ensure they match your expectations
5. **CI Integration**: Run generated tests in your CI pipeline to catch API contract changes

## Limitations

- Sample data generation uses simple defaults (may need customization for complex types)
- Integration tests are basic (complex workflows need manual implementation)
- Error response validation is generic (specific error checking needs manual tests)

## Future Enhancements

- Custom sample data providers
- Property-based testing integration
- Contract testing support
- Performance test generation
- OpenAPI specification validation tests
