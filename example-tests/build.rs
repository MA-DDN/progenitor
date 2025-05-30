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

            // Create generator
            let mut generator = Generator::default();

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
            
            let test_ast = syn::parse2(test_tokens)
                .expect("Failed to parse test tokens");
            
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

            // Verify the generated content
            let mut checks_passed = 0;
            let mut total_checks = 0;

            macro_rules! check {
                ($condition:expr, $description:expr) => {
                    total_checks += 1;
                    if $condition {
                        println!("✓ {}", $description);
                        checks_passed += 1;
                    } else {
                        println!("✗ {}", $description);
                    }
                };
            }

            println!("\n🔍 Verification Checks:");
            check!(
                Path::new("src/lib.rs").exists(),
                "lib.rs file was created"
            );
            check!(
                combined_content.contains("pub struct Client"),
                "Contains Client struct"
            );
            check!(
                combined_content.contains("pub mod httpmock"),
                "Contains httpmock module"
            );
            check!(
                combined_content.contains("mod generated_tests"),
                "Contains test module"
            );
            check!(
                combined_content.contains("test_list_users_success"),
                "Contains list_users success test"
            );
            check!(
                combined_content.contains("test_create_user_success"),
                "Contains create_user success test"
            );
            check!(
                combined_content.contains("test_list_users_error"),
                "Contains list_users error test"
            );
            check!(
                combined_content.contains("test_create_user_error"),
                "Contains create_user error test"
            );
            check!(
                combined_content.contains("MockServer::start()"), 
                "Uses MockServer"
            );
            check!(
                combined_content.contains("#[tokio::test]"),
                "Uses tokio::test attribute"
            );

            println!(
                "\n📊 Summary: {}/{} checks passed",
                checks_passed, total_checks
            );

            if checks_passed == total_checks {
                println!("🎉 All verification checks passed!");
                println!("\n💡 The client and test generation is working correctly!");
                println!("   Run 'cargo test' to execute the generated tests.");
            } else {
                println!("⚠️  Some checks failed. The implementation may need refinement.");
            }

            // Also write a preview file for inspection
            fs::write("generated_preview.rs", combined_content.clone())
                .expect("Failed to write preview file");
            println!("\n📄 Full generated code also written to: generated_preview.rs");
        }
        Err(e) => {
            println!("❌ Failed to parse OpenAPI spec: {}", e);
        }
    }

    println!("\n🏁 Demo complete!");
    
    // Tell Cargo to rerun this build script if the build.rs file changes
    println!("cargo:rerun-if-changed=build.rs");
}
