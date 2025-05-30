// Example build.rs showing how to generate unit tests automatically
// This would be used in a client crate's build.rs file

use std::path::Path;

fn main() {
    // Read the OpenAPI specification
    let spec_path = "openapi.yaml";
    let spec_content = std::fs::read_to_string(spec_path)
        .expect("Failed to read OpenAPI specification");
    
    let spec: openapiv3::OpenAPI = serde_yaml::from_str(&spec_content)
        .expect("Failed to parse OpenAPI specification");

    let mut generator = progenitor::Generator::default();

    // Generate the main client code
    let tokens = generator.generate_tokens(&spec)
        .expect("Failed to generate client tokens");
    
    let ast = syn::parse2(tokens)
        .expect("Failed to parse generated tokens");
    
    let content = prettyplease::unparse(&ast);
    
    // Write the main client
    std::fs::write("src/lib.rs", content)
        .expect("Failed to write client code");

    // Generate httpmock support
    let httpmock_tokens = generator.httpmock(&spec, "crate")
        .expect("Failed to generate httpmock tokens");
    
    let httpmock_ast = syn::parse2(httpmock_tokens)
        .expect("Failed to parse httpmock tokens");
    
    let httpmock_content = prettyplease::unparse(&httpmock_ast);
    
    // Write httpmock support
    std::fs::write("src/httpmock.rs", httpmock_content)
        .expect("Failed to write httpmock code");

    // Generate comprehensive unit tests
    let test_config = progenitor::TestGenerationConfig {
        generate_success_tests: true,
        generate_error_tests: true,
        generate_parameter_tests: true,
        generate_integration_tests: false,
        test_module_name: "generated_tests".to_string(),
        include_documentation: true,
    };

    let test_tokens = generator.generate_tests(&spec, "crate", &test_config)
        .expect("Failed to generate test tokens");
    
    let test_ast = syn::parse2(test_tokens)
        .expect("Failed to parse test tokens");
    
    let test_content = prettyplease::unparse(&test_ast);
    
    // Write the generated tests
    std::fs::write("src/generated_tests.rs", test_content)
        .expect("Failed to write test code");

    println!("cargo:rerun-if-changed={}", spec_path);
    println!("Generated client, httpmock support, and comprehensive unit tests");
}

// Alternative: Generate tests as a separate file that can be included
fn generate_tests_as_include_file() {
    let spec_path = "openapi.yaml";
    let spec_content = std::fs::read_to_string(spec_path)
        .expect("Failed to read OpenAPI specification");
    
    let spec: openapiv3::OpenAPI = serde_yaml::from_str(&spec_content)
        .expect("Failed to parse OpenAPI specification");

    let mut generator = progenitor::Generator::default();

    // Configure test generation
    let test_config = progenitor::TestGenerationConfig {
        generate_success_tests: true,
        generate_error_tests: true,
        generate_parameter_tests: true,
        generate_integration_tests: true, // Enable integration tests
        test_module_name: "api_tests".to_string(),
        include_documentation: true,
    };

    // Generate tests
    let test_tokens = generator.generate_tests(&spec, "crate", &test_config)
        .expect("Failed to generate test tokens");
    
    let test_ast = syn::parse2(test_tokens)
        .expect("Failed to parse test tokens");
    
    let test_content = prettyplease::unparse(&test_ast);
    
    // Write to tests directory
    std::fs::create_dir_all("tests").expect("Failed to create tests directory");
    std::fs::write("tests/generated_api_tests.rs", test_content)
        .expect("Failed to write test file");
}

// Example of customizing test generation for specific needs
fn generate_custom_tests() {
    let spec_path = "openapi.yaml";
    let spec_content = std::fs::read_to_string(spec_path)
        .expect("Failed to read OpenAPI specification");
    
    let spec: openapiv3::OpenAPI = serde_yaml::from_str(&spec_content)
        .expect("Failed to parse OpenAPI specification");

    let mut generator = progenitor::Generator::default();

    // Custom configuration for different test types
    let configs = vec![
        // Basic success tests only
        progenitor::TestGenerationConfig {
            generate_success_tests: true,
            generate_error_tests: false,
            generate_parameter_tests: false,
            generate_integration_tests: false,
            test_module_name: "success_tests".to_string(),
            include_documentation: true,
        },
        // Error handling tests only
        progenitor::TestGenerationConfig {
            generate_success_tests: false,
            generate_error_tests: true,
            generate_parameter_tests: false,
            generate_integration_tests: false,
            test_module_name: "error_tests".to_string(),
            include_documentation: true,
        },
        // Parameter validation tests only
        progenitor::TestGenerationConfig {
            generate_success_tests: false,
            generate_error_tests: false,
            generate_parameter_tests: true,
            generate_integration_tests: false,
            test_module_name: "parameter_tests".to_string(),
            include_documentation: true,
        },
    ];

    for (i, config) in configs.iter().enumerate() {
        let test_tokens = generator.generate_tests(&spec, "crate", config)
            .expect("Failed to generate test tokens");
        
        let test_ast = syn::parse2(test_tokens)
            .expect("Failed to parse test tokens");
        
        let test_content = prettyplease::unparse(&test_ast);
        
        let filename = format!("tests/generated_tests_{}.rs", i);
        std::fs::write(&filename, test_content)
            .expect("Failed to write test file");
    }
}
