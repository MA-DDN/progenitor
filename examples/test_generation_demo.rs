// Example demonstrating automatic unit test generation with progenitor
// This shows how to generate comprehensive test suites for OpenAPI clients

use progenitor_impl::{Generator, TestGenerationConfig};
use openapiv3::OpenAPI;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Progenitor Test Generation Demo");
    println!("==================================\n");

    // Create a sample OpenAPI specification
    let spec_yaml = create_sample_openapi_spec();
    
    // Parse the OpenAPI spec
    let spec: OpenAPI = serde_yaml::from_str(&spec_yaml)?;
    println!("✓ Parsed OpenAPI specification");

    // Create generator
    let mut generator = Generator::default();

    // Generate the main client code
    println!("\n📦 Generating main client code...");
    let tokens = generator.generate_tokens(&spec)?;
    let ast = syn::parse2(tokens)?;
    let content = prettyplease::unparse(&ast);
    fs::write("demo_client.rs", content)?;
    println!("✓ Generated client code -> demo_client.rs");

    // Generate httpmock support
    println!("\n🔧 Generating httpmock support...");
    let httpmock_tokens = generator.httpmock(&spec, "crate")?;
    let httpmock_ast = syn::parse2(httpmock_tokens)?;
    let httpmock_content = prettyplease::unparse(&httpmock_ast);
    fs::write("demo_httpmock.rs", httpmock_content)?;
    println!("✓ Generated httpmock support -> demo_httpmock.rs");

    // Generate comprehensive unit tests
    println!("\n🧪 Generating comprehensive unit tests...");
    
    // Configuration for comprehensive test generation
    let test_config = TestGenerationConfig {
        generate_success_tests: true,
        generate_error_tests: true,
        generate_parameter_tests: true,
        generate_integration_tests: false,
        test_module_name: "generated_tests".to_string(),
        include_documentation: true,
    };

    let test_tokens = generator.generate_tests(&spec, "crate", &test_config)?;
    let test_ast = syn::parse2(test_tokens)?;
    let test_content = prettyplease::unparse(&test_ast);
    fs::write("demo_tests.rs", test_content)?;
    println!("✓ Generated comprehensive tests -> demo_tests.rs");

    // Generate different types of tests separately
    println!("\n🎯 Generating specialized test suites...");

    // Success tests only
    let success_config = TestGenerationConfig {
        generate_success_tests: true,
        generate_error_tests: false,
        generate_parameter_tests: false,
        generate_integration_tests: false,
        test_module_name: "success_tests".to_string(),
        include_documentation: true,
    };

    let success_tokens = generator.generate_tests(&spec, "crate", &success_config)?;
    let success_ast = syn::parse2(success_tokens)?;
    let success_content = prettyplease::unparse(&success_ast);
    fs::write("demo_success_tests.rs", success_content)?;
    println!("✓ Generated success tests -> demo_success_tests.rs");

    // Error tests only
    let error_config = TestGenerationConfig {
        generate_success_tests: false,
        generate_error_tests: true,
        generate_parameter_tests: false,
        generate_integration_tests: false,
        test_module_name: "error_tests".to_string(),
        include_documentation: true,
    };

    let error_tokens = generator.generate_tests(&spec, "crate", &error_config)?;
    let error_ast = syn::parse2(error_tokens)?;
    let error_content = prettyplease::unparse(&error_ast);
    fs::write("demo_error_tests.rs", error_content)?;
    println!("✓ Generated error tests -> demo_error_tests.rs");

    // Parameter tests only
    let param_config = TestGenerationConfig {
        generate_success_tests: false,
        generate_error_tests: false,
        generate_parameter_tests: true,
        generate_integration_tests: false,
        test_module_name: "parameter_tests".to_string(),
        include_documentation: true,
    };

    let param_tokens = generator.generate_tests(&spec, "crate", &param_config)?;
    let param_ast = syn::parse2(param_tokens)?;
    let param_content = prettyplease::unparse(&param_ast);
    fs::write("demo_parameter_tests.rs", param_content)?;
    println!("✓ Generated parameter tests -> demo_parameter_tests.rs");

    // Summary
    println!("\n🎉 Test Generation Complete!");
    println!("============================");
    println!("Generated files:");
    println!("  📄 demo_client.rs           - Main API client");
    println!("  🔧 demo_httpmock.rs         - httpmock support");
    println!("  🧪 demo_tests.rs            - Comprehensive test suite");
    println!("  ✅ demo_success_tests.rs    - Success case tests");
    println!("  ❌ demo_error_tests.rs      - Error case tests");
    println!("  🔍 demo_parameter_tests.rs  - Parameter validation tests");

    println!("\n📊 Test Coverage Includes:");
    println!("  • Type-safe httpmock integration");
    println!("  • Success tests for all operations");
    println!("  • Error tests for 4xx/5xx responses");
    println!("  • Parameter validation tests");
    println!("  • Comprehensive documentation");
    println!("  • Async/await support with tokio");

    println!("\n💡 Next Steps:");
    println!("  1. Review the generated test files");
    println!("  2. Customize sample data generation if needed");
    println!("  3. Add the tests to your project");
    println!("  4. Run: cargo test");

    Ok(())
}

fn create_sample_openapi_spec() -> String {
    r#"
openapi: 3.0.0
info:
  title: User Management API
  description: A comprehensive API for managing users and organizations
  version: 1.0.0
  contact:
    name: API Support
    email: support@example.com
servers:
  - url: https://api.example.com/v1
    description: Production server
paths:
  /users:
    get:
      operationId: list_users
      summary: List all users
      description: Retrieve a paginated list of all users
      parameters:
        - name: limit
          in: query
          description: Maximum number of users to return
          schema:
            type: integer
            minimum: 1
            maximum: 100
            default: 20
        - name: offset
          in: query
          description: Number of users to skip
          schema:
            type: integer
            minimum: 0
            default: 0
      responses:
        '200':
          description: List of users
          content:
            application/json:
              schema:
                type: object
                properties:
                  users:
                    type: array
                    items:
                      $ref: '#/components/schemas/User'
                  total:
                    type: integer
                  limit:
                    type: integer
                  offset:
                    type: integer
        '400':
          description: Bad request
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
        '500':
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
    post:
      operationId: create_user
      summary: Create a new user
      description: Create a new user account
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/CreateUserRequest'
      responses:
        '201':
          description: User created successfully
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
        '400':
          description: Invalid input
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
        '409':
          description: User already exists
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
  /users/{id}:
    get:
      operationId: get_user
      summary: Get user by ID
      description: Retrieve a specific user by their ID
      parameters:
        - name: id
          in: path
          required: true
          description: User ID
          schema:
            type: string
            format: uuid
      responses:
        '200':
          description: User details
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
        '404':
          description: User not found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
    put:
      operationId: update_user
      summary: Update user
      description: Update an existing user's information
      parameters:
        - name: id
          in: path
          required: true
          description: User ID
          schema:
            type: string
            format: uuid
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/UpdateUserRequest'
      responses:
        '200':
          description: User updated successfully
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
        '400':
          description: Invalid input
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
        '404':
          description: User not found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
    delete:
      operationId: delete_user
      summary: Delete user
      description: Delete a user account
      parameters:
        - name: id
          in: path
          required: true
          description: User ID
          schema:
            type: string
            format: uuid
      responses:
        '204':
          description: User deleted successfully
        '404':
          description: User not found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
components:
  schemas:
    User:
      type: object
      description: A user in the system
      properties:
        id:
          type: string
          format: uuid
          description: Unique user identifier
        name:
          type: string
          description: User's full name
          minLength: 1
          maxLength: 100
        email:
          type: string
          format: email
          description: User's email address
        created_at:
          type: string
          format: date-time
          description: When the user was created
        updated_at:
          type: string
          format: date-time
          description: When the user was last updated
        is_active:
          type: boolean
          description: Whether the user account is active
          default: true
      required:
        - id
        - name
        - email
        - created_at
        - updated_at
        - is_active
    CreateUserRequest:
      type: object
      description: Request to create a new user
      properties:
        name:
          type: string
          description: User's full name
          minLength: 1
          maxLength: 100
        email:
          type: string
          format: email
          description: User's email address
      required:
        - name
        - email
    UpdateUserRequest:
      type: object
      description: Request to update a user
      properties:
        name:
          type: string
          description: User's full name
          minLength: 1
          maxLength: 100
        email:
          type: string
          format: email
          description: User's email address
        is_active:
          type: boolean
          description: Whether the user account is active
    Error:
      type: object
      description: Error response
      properties:
        message:
          type: string
          description: Human-readable error message
        code:
          type: string
          description: Machine-readable error code
        details:
          type: object
          description: Additional error details
          additionalProperties: true
      required:
        - message
        - code
"#.to_string()
}
