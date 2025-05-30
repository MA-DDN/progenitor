// Copyright 2025 Oxide Computer Company

//! Generation of comprehensive unit tests using httpmock

use openapiv3::OpenAPI;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    method::{
        BodyContentType, OperationMethod, OperationParameter, OperationParameterKind,
        OperationParameterType, OperationResponse, OperationResponseKind, OperationResponseStatus,
    },
    to_schema::ToSchema,
    validate_openapi, Generator, Result,
};

/// Configuration for test generation
#[derive(Debug, Clone)]
pub struct TestGenerationConfig {
    /// Whether to generate success case tests
    pub generate_success_tests: bool,
    /// Whether to generate error case tests
    pub generate_error_tests: bool,
    /// Whether to generate parameter validation tests
    pub generate_parameter_tests: bool,
    /// Whether to generate integration-style tests that test multiple operations
    pub generate_integration_tests: bool,
    /// Custom test module name (defaults to "tests")
    pub test_module_name: String,
    /// Whether to include doc comments in generated tests
    pub include_documentation: bool,
}

impl Default for TestGenerationConfig {
    fn default() -> Self {
        Self {
            generate_success_tests: true,
            generate_error_tests: true,
            generate_parameter_tests: true,
            generate_integration_tests: false,
            test_module_name: "tests".to_string(),
            include_documentation: true,
        }
    }
}

impl Generator {
    /// Generate comprehensive unit tests using httpmock
    ///
    /// The `crate_path` parameter should be a valid Rust path corresponding to
    /// the SDK. This can include `::` and instances of `-` in the crate name
    /// should be converted to `_`.
    pub fn generate_tests_impl(
        &mut self,
        spec: &OpenAPI,
        crate_path: &str,
        config: &TestGenerationConfig,
    ) -> Result<TokenStream> {
        validate_openapi(spec)?;

        // Convert our components dictionary to schemars
        let schemas = spec.components.iter().flat_map(|components| {
            components
                .schemas
                .iter()
                .map(|(name, ref_or_schema)| (name.clone(), ref_or_schema.to_schema()))
        });

        self.type_space.add_ref_types(schemas)?;

        let raw_methods = spec
            .paths
            .iter()
            .flat_map(|(path, ref_or_item)| {
                // Exclude externally defined path items.
                let item = ref_or_item.as_item().unwrap();
                item.iter().map(move |(method, operation)| {
                    (path.as_str(), method, operation, &item.parameters)
                })
            })
            .map(|(path, method, operation, path_parameters)| {
                self.process_operation(operation, &spec.components, path, method, path_parameters)
            })
            .collect::<Result<Vec<_>>>()?;

        let mut test_functions = Vec::new();

        // Generate tests for each operation
        for method in &raw_methods {
            if config.generate_success_tests {
                test_functions.extend(self.generate_success_tests_internal(method, crate_path)?);
            }

            if config.generate_error_tests {
                test_functions.extend(self.generate_error_tests_internal(method, crate_path)?);
            }

            if config.generate_parameter_tests {
                test_functions.extend(self.generate_parameter_tests_internal(method, crate_path)?);
            }
        }

        // Generate integration tests if requested
        if config.generate_integration_tests {
            test_functions
                .extend(self.generate_integration_tests_internal(&raw_methods, crate_path)?);
        }

        let test_module_name = format_ident!("{}", config.test_module_name);
        let crate_path = syn::TypePath {
            qself: None,
            path: syn::parse_str(crate_path)
                .unwrap_or_else(|_| panic!("{} is not a valid identifier", crate_path)),
        };

        let code = quote! {
            #[cfg(test)]
            pub mod #test_module_name {
                use super::*;
                use #crate_path::httpmock::{MockServerExt, operations};
                use ::httpmock::prelude::*;
                use tokio;

                #(#test_functions)*
            }
        };

        Ok(code)
    }

    /// Generate success case tests for an operation
    fn generate_success_tests_internal(
        &self,
        method: &OperationMethod,
        _crate_path: &str,
    ) -> Result<Vec<TokenStream>> {
        let mut tests = Vec::new();

        // Get success responses
        let success_responses: Vec<_> = method
            .responses
            .iter()
            .filter(|r| r.status_code.is_success_or_default())
            .collect();

        for response in success_responses {
            let test_name = format_ident!(
                "test_{}_success_{}",
                method.operation_id,
                self.response_status_suffix_internal(&response.status_code)
            );

            let operation_call = self.generate_operation_call_internal(method, false)?;
            let mock_setup = self.generate_mock_setup_internal(method, response, false)?;
            let assertions = self.generate_success_assertions_internal(method, response)?;

            let summary = method.summary.as_deref().unwrap_or("operation");
            let doc_comment = format!(
                "Test successful {} with {} response",
                summary, response.status_code
            );

            tests.push(quote! {
                #[tokio::test]
                #[doc = #doc_comment]
                async fn #test_name() {
                    let server = MockServer::start();
                    let client = Client::new(&server.base_url());

                    #mock_setup

                    #operation_call

                    #assertions
                }
            });
        }

        Ok(tests)
    }

    /// Generate error case tests for an operation
    fn generate_error_tests_internal(
        &self,
        method: &OperationMethod,
        _crate_path: &str,
    ) -> Result<Vec<TokenStream>> {
        let mut tests = Vec::new();

        // Get error responses
        let error_responses: Vec<_> = method
            .responses
            .iter()
            .filter(|r| {
                r.status_code.is_error_or_default() && !r.status_code.is_success_or_default()
            })
            .collect();

        for response in error_responses {
            let test_name = format_ident!(
                "test_{}_error_{}",
                method.operation_id,
                self.response_status_suffix_internal(&response.status_code)
            );

            let operation_call = self.generate_operation_call_internal(method, true)?;
            let mock_setup = self.generate_mock_setup_internal(method, response, true)?;
            let assertions = self.generate_error_assertions_internal(method, response)?;

            let summary = method.summary.as_deref().unwrap_or("operation");
            let doc_comment = format!(
                "Test error case for {} with {} response",
                summary, response.status_code
            );

            tests.push(quote! {
                #[tokio::test]
                #[doc = #doc_comment]
                async fn #test_name() {
                    let server = MockServer::start();
                    let client = Client::new(&server.base_url());

                    #mock_setup

                    #operation_call

                    #assertions
                }
            });
        }

        Ok(tests)
    }

    /// Generate parameter validation tests
    fn generate_parameter_tests_internal(
        &self,
        method: &OperationMethod,
        _crate_path: &str,
    ) -> Result<Vec<TokenStream>> {
        let mut tests = Vec::new();

        // Test each parameter individually (simplified for now)
        for param in &method.params {
            let test_name = format_ident!("test_{}_param_{}", method.operation_id, param.name);

            let doc_comment = format!("Test {} with parameter {}", method.operation_id, param.name);

            // Generate test for parameter validation
            tests.push(quote! {
                #[tokio::test]
                #[doc = #doc_comment]
                async fn #test_name() {
                    let server = MockServer::start();
                    let client = Client::new(&server.base_url());

                    // Test with parameter validation
                    // Implementation would verify parameter handling
                    // TODO: Implement parameter validation test
                }
            });
        }

        Ok(tests)
    }

    /// Generate integration tests that test multiple operations together
    fn generate_integration_tests_internal(
        &self,
        _methods: &[OperationMethod],
        _crate_path: &str,
    ) -> Result<Vec<TokenStream>> {
        // This would generate tests that combine multiple operations
        // For example: create -> read -> update -> delete workflows
        Ok(vec![])
    }

    /// Generate the operation call code for tests
    fn generate_operation_call_internal(
        &self,
        method: &OperationMethod,
        _expect_error: bool,
    ) -> Result<TokenStream> {
        let operation_id = format_ident!("{}", method.operation_id);

        // Generate the operation call based on the method signature
        let result_handling = if method.params.iter().any(|p| matches!(p.kind, OperationParameterKind::Body(_))) {
            // Method has a body parameter
            let body_param = method.params.iter().find(|p| matches!(p.kind, OperationParameterKind::Body(_))).unwrap();
            let sample_value = self.generate_sample_value_internal(body_param)?;
            quote! {
                let result = client.#operation_id(&#sample_value).await;
            }
        } else {
            // Method has no body parameter
            quote! {
                let result = client.#operation_id().await;
            }
        };

        Ok(result_handling)
    }

    /// Generate mock setup code for httpmock
    fn generate_mock_setup_internal(
        &self,
        method: &OperationMethod,
        response: &OperationResponse,
        is_error: bool,
    ) -> Result<TokenStream> {
        let operation_id = format_ident!("{}", method.operation_id);

        // Generate when conditions for body parameters
        let when_conditions = if let Some(body_param) = method.params.iter().find(|p| matches!(p.kind, OperationParameterKind::Body(_))) {
            let sample_value = self.generate_sample_value_internal(body_param)?;
            quote! { when.body(&#sample_value); }
        } else {
            quote! { when; }
        };

        // Generate then response
        let then_response = self.generate_mock_response_internal(response, is_error)?;

        Ok(quote! {
            let _mock = server.#operation_id(|when, then| {
                #when_conditions
                #then_response
            });
        })
    }

    /// Generate mock response based on response type
    fn generate_mock_response_internal(
        &self,
        response: &OperationResponse,
        _is_error: bool,
    ) -> Result<TokenStream> {
        match &response.status_code {
            OperationResponseStatus::Code(200) => {
                let sample_response =
                    self.generate_sample_response_value_internal(&response.typ)?;
                Ok(quote! { then.ok(&#sample_response); })
            }
            OperationResponseStatus::Code(201) => {
                let sample_response =
                    self.generate_sample_response_value_internal(&response.typ)?;
                Ok(quote! { then.created(&#sample_response); })
            }
            OperationResponseStatus::Code(400) => {
                let sample_response =
                    self.generate_sample_response_value_internal(&response.typ)?;
                Ok(quote! { then.bad_request(&#sample_response); })
            }
            OperationResponseStatus::Code(500) => {
                let sample_response =
                    self.generate_sample_response_value_internal(&response.typ)?;
                Ok(quote! { then.internal_server_error(&#sample_response); })
            }
            OperationResponseStatus::Code(code) if *code >= 400 => {
                let sample_response =
                    self.generate_sample_response_value_internal(&response.typ)?;
                // Use a generic error method for other status codes
                Ok(quote! { then.status(#code).json_body_obj(&#sample_response); })
            }
            OperationResponseStatus::Range(4) => {
                let sample_response =
                    self.generate_sample_response_value_internal(&response.typ)?;
                Ok(quote! { then.bad_request(&#sample_response); })
            }
            OperationResponseStatus::Range(5) => {
                let sample_response =
                    self.generate_sample_response_value_internal(&response.typ)?;
                Ok(quote! { then.internal_server_error(&#sample_response); })
            }
            _ => {
                let sample_response =
                    self.generate_sample_response_value_internal(&response.typ)?;
                Ok(quote! { then.ok(&#sample_response); })
            }
        }
    }

    /// Generate sample parameter values for testing
    fn generate_sample_value_internal(&self, param: &OperationParameter) -> Result<TokenStream> {
        match &param.typ {
            OperationParameterType::Type(type_id) => {
                let type_entry = self.type_space.get_type(type_id).unwrap();
                let type_ident = type_entry.ident();
                Ok(quote! { #type_ident::default() })
            }
            OperationParameterType::RawBody => match &param.kind {
                OperationParameterKind::Body(BodyContentType::Json) => {
                    Ok(quote! { serde_json::json!({"test": "data"}) })
                }
                OperationParameterKind::Body(BodyContentType::Text(_)) => {
                    Ok(quote! { "test body content" })
                }
                _ => Ok(quote! { "test data" }),
            },
        }
    }

    /// Generate sample response values for mocking
    fn generate_sample_response_value_internal(
        &self,
        response_kind: &OperationResponseKind,
    ) -> Result<TokenStream> {
        match response_kind {
            OperationResponseKind::Type(type_id) => {
                let type_entry = self.type_space.get_type(type_id).unwrap();

                // Use a more careful approach to generate the type reference
                // to avoid issues with generic parameters and spacing
                let details = type_entry.details();
                match details {
                    typify::TypeDetails::Vec(inner_type_id) => {
                        let inner_type = self.type_space.get_type(&inner_type_id).unwrap();
                        let inner_ident = inner_type.ident();
                        Ok(quote! { Vec::<#inner_ident>::default() })
                    }
                    _ => {
                        let type_ident = type_entry.ident();
                        Ok(quote! { #type_ident::default() })
                    }
                }
            }
            OperationResponseKind::None => Ok(quote! { () }),
            OperationResponseKind::Raw => Ok(quote! { serde_json::json!({"message": "success"}) }),
            OperationResponseKind::Upgrade => Ok(quote! { () }),
            OperationResponseKind::Multiple { enum_name, .. } => {
                let enum_ident = format_ident!("{}", enum_name);
                Ok(quote! { #enum_ident::default() })
            }
        }
    }

    /// Generate success test assertions
    fn generate_success_assertions_internal(
        &self,
        _method: &OperationMethod,
        _response: &OperationResponse,
    ) -> Result<TokenStream> {
        Ok(quote! {
            assert!(result.is_ok());
            let response = result.unwrap();
            // Add specific assertions based on response type
        })
    }

    /// Generate error test assertions
    fn generate_error_assertions_internal(
        &self,
        _method: &OperationMethod,
        response: &OperationResponse,
    ) -> Result<TokenStream> {
        match &response.status_code {
            OperationResponseStatus::Code(code) if *code >= 400 => {
                Ok(quote! {
                    assert!(result.is_err());
                    // Verify specific error type and status code
                })
            }
            _ => Ok(quote! {
                assert!(result.is_err());
            }),
        }
    }

    /// Generate a suffix for response status in test names
    fn response_status_suffix_internal(&self, status: &OperationResponseStatus) -> String {
        match status {
            OperationResponseStatus::Code(code) => code.to_string(),
            OperationResponseStatus::Range(range) => format!("{}xx", range),
            OperationResponseStatus::Default => "default".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        method::{
            BodyContentType, OperationParameter, OperationParameterKind, OperationParameterType,
        },
        Generator, TestGenerationConfig,
    };
    use openapiv3::OpenAPI;

    #[test]
    fn test_generate_tests_basic() {
        // Simple OpenAPI spec for testing
        let spec_yaml = r#"
openapi: 3.0.0
info:
  title: Test API
  version: 1.0.0
paths:
  /users:
    get:
      operationId: list_users
      summary: List all users
      responses:
        '200':
          description: List of users
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/User'
    post:
      operationId: create_user
      summary: Create a new user
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/CreateUserRequest'
      responses:
        '201':
          description: User created
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
        email:
          type: string
      required:
        - id
        - name
        - email
    CreateUserRequest:
      type: object
      properties:
        name:
          type: string
        email:
          type: string
      required:
        - name
        - email
    Error:
      type: object
      properties:
        message:
          type: string
        code:
          type: integer
      required:
        - message
"#;

        // Parse the OpenAPI spec
        let spec: OpenAPI = serde_yaml::from_str(spec_yaml).expect("Failed to parse OpenAPI spec");

        // Create generator
        let mut generator = Generator::default();

        // Test configuration
        let test_config = TestGenerationConfig {
            generate_success_tests: true,
            generate_error_tests: true,
            generate_parameter_tests: true,
            generate_integration_tests: false,
            test_module_name: "generated_tests".to_string(),
            include_documentation: true,
        };

        // Generate tests
        let result = generator.generate_tests(&spec, "crate", &test_config);

        // Verify the generation succeeded
        assert!(result.is_ok(), "Test generation should succeed");

        let test_tokens = result.unwrap();
        let test_code = test_tokens.to_string();

        // Verify the generated code contains expected elements
        assert!(
            test_code.contains("mod generated_tests"),
            "Should contain test module"
        );
        assert!(
            test_code.contains("test_list_users_success"),
            "Should contain success test for list_users"
        );
        assert!(
            test_code.contains("test_create_user_success"),
            "Should contain success test for create_user"
        );
        assert!(
            test_code.contains("test_create_user_error"),
            "Should contain error test for create_user"
        );
        assert!(
            test_code.contains("MockServer :: start ()"),
            "Should use MockServer"
        );
        assert!(
            test_code.contains("async fn"),
            "Should generate async test functions"
        );
        assert!(
            test_code.contains("# [tokio :: test]"),
            "Should use tokio::test attribute"
        );

        println!("Generated test code preview:");
        println!("{}", &test_code[..std::cmp::min(1000, test_code.len())]);
    }

    #[test]
    fn test_generate_tests_config_variations() {
        let spec_yaml = r#"
openapi: 3.0.0
info:
  title: Test API
  version: 1.0.0
paths:
  /test:
    get:
      operationId: test_operation
      responses:
        '200':
          description: Success
          content:
            application/json:
              schema:
                type: string
        '400':
          description: Error
          content:
            application/json:
              schema:
                type: string
"#;

        let spec: OpenAPI = serde_yaml::from_str(spec_yaml).expect("Failed to parse OpenAPI spec");
        let mut generator = Generator::default();

        // Test with only success tests
        let success_only_config = TestGenerationConfig {
            generate_success_tests: true,
            generate_error_tests: false,
            generate_parameter_tests: false,
            generate_integration_tests: false,
            test_module_name: "success_tests".to_string(),
            include_documentation: true,
        };

        let result = generator.generate_tests(&spec, "crate", &success_only_config);
        assert!(
            result.is_ok(),
            "Success-only test generation should succeed"
        );

        let test_code = result.unwrap().to_string();
        assert!(
            test_code.contains("test_test_operation_success"),
            "Should contain success test"
        );
        assert!(
            !test_code.contains("test_test_operation_error"),
            "Should not contain error test"
        );

        // Test with only error tests
        let error_only_config = TestGenerationConfig {
            generate_success_tests: false,
            generate_error_tests: true,
            generate_parameter_tests: false,
            generate_integration_tests: false,
            test_module_name: "error_tests".to_string(),
            include_documentation: true,
        };

        let result = generator.generate_tests(&spec, "crate", &error_only_config);
        assert!(result.is_ok(), "Error-only test generation should succeed");

        let test_code = result.unwrap().to_string();
        assert!(
            test_code.contains("test_test_operation_error"),
            "Should contain error test"
        );
        assert!(
            !test_code.contains("test_test_operation_success"),
            "Should not contain success test"
        );
    }

    #[test]
    fn test_sample_value_generation() {
        let generator = Generator::default();

        // Test string parameter
        let string_param = OperationParameter {
            api_name: "test_string".to_string(),
            name: "test_string".to_string(),
            typ: OperationParameterType::RawBody,
            kind: OperationParameterKind::Body(BodyContentType::Json),
            description: None,
        };

        let result = generator.generate_sample_value_internal(&string_param);
        assert!(result.is_ok(), "String sample generation should succeed");

        let sample_code = result.unwrap().to_string();
        assert!(sample_code.contains("test"), "Should generate test data");
    }
}
