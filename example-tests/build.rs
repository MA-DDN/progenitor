// Comprehensive demonstration of client and test generation functionality

use progenitor_impl::{Generator, TestGenerationConfig};
use std::fs;
use std::path::Path;

fn main() {
    println!("🚀 Progenitor Client and Test Generation Demo");
    println!("============================================\n");

    // Simple OpenAPI spec
    let spec_yaml = r#"
openapi: 3.0.0
info:
  title: Simple API
  version: 1.0.0
paths:
  /users:
    get:
      operationId: list_users
      summary: List users
      responses:
        '200':
          description: Success
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/User'
        '500':
          description: Server error
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
    post:
      operationId: create_user
      summary: Create user
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/CreateUserRequest'
      responses:
        '201':
          description: Created
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
        '400':
          description: Bad request
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
components:
  schemas:
    User:
      type: object
      properties:
        id:
          type: string
        name:
          type: string
      required:
        - id
        - name
    CreateUserRequest:
      type: object
      properties:
        name:
          type: string
      required:
        - name
    Error:
      type: object
      properties:
        message:
          type: string
      required:
        - message
"#;

    // Parse the spec
    match serde_yaml::from_str::<openapiv3::OpenAPI>(spec_yaml) {
        Ok(spec) => {
            println!("✓ Successfully parsed OpenAPI specification");

            // Create generator with proper settings
            let mut settings = progenitor_impl::GenerationSettings::default();
            settings.with_derive("PartialEq").with_derive("Default");
            let mut generator = Generator::new(&settings);

            // 1. Generate the main client code
            println!("\n📦 Generating main client code...");
            let client_tokens = generator.generate_tokens(&spec)
                .expect("Failed to generate client tokens");

            let client_ast = syn::parse2(client_tokens)
                .expect("Failed to parse client tokens");

            let client_content = prettyplease::unparse(&client_ast);
            println!("✓ Successfully generated client code");

            // 2. Generate httpmock support
            println!("\n🔧 Generating httpmock support...");
            let httpmock_tokens = generator.httpmock(&spec, "crate")
                .expect("Failed to generate httpmock tokens");

            let httpmock_ast = syn::parse2(httpmock_tokens)
                .expect("Failed to parse httpmock tokens");

            let httpmock_content = prettyplease::unparse(&httpmock_ast);
            println!("✓ Successfully generated httpmock support");

            // 3. Generate unit tests
            println!("\n🧪 Generating unit tests...");
            let test_config = TestGenerationConfig {
                generate_success_tests: true,
                generate_error_tests: true,
                generate_parameter_tests: true,
                generate_integration_tests: false,
                test_module_name: "generated_tests".to_string(),
                include_documentation: true,
            };

            let test_tokens = generator.generate_tests(&spec, "crate", &test_config)
                .expect("Failed to generate test tokens");

            // Try to parse the tokens directly without string conversion
            let test_ast = match syn::parse2::<syn::File>(test_tokens.clone()) {
                Ok(ast) => {
                    println!("✓ Successfully parsed test tokens");
                    ast
                }
                Err(e) => {
                    println!("❌ Failed to parse test tokens: {}", e);

                    // Try to find the problematic part by parsing smaller chunks
                    let tokens_str = test_tokens.to_string();
                    println!("Raw tokens (first 500 chars): {}", &tokens_str[..tokens_str.len().min(500)]);
                    panic!("Failed to parse test tokens: {}", e);
                }
            };

            let test_content = prettyplease::unparse(&test_ast);
            println!("✓ Successfully generated unit tests");

            // 4. Combine client and tests into a single lib.rs file
            println!("\n📄 Combining client and tests into lib.rs...");

            // Create src directory if it doesn't exist
            fs::create_dir_all("src").expect("Failed to create src directory");

            // Combine the content
            let combined_content = format!(
                "//! Auto-generated client and tests for the Simple API\n\n\
                 // Client code\n\
                 {}\n\n\
                 // HTTPMock support\n\
                 pub mod httpmock {{\n{}\n}}\n\n\
                 // Generated tests\n\
                 #[cfg(test)]\n\
                 {}\n",
                client_content, httpmock_content, test_content
            );

            // Write the combined file
            fs::write("src/lib.rs", &combined_content)
                .expect("Failed to write lib.rs file");

            println!("✓ Successfully created src/lib.rs with client and tests");
        }
        Err(e) => {
            println!("❌ Failed to parse OpenAPI spec: {}", e);
        }
    }

    println!("\n🏁 Demo complete!");

    // Tell Cargo to rerun this build script if the build.rs file changes
    println!("cargo:rerun-if-changed=build.rs");
}
