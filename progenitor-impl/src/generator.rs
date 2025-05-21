// Update the generate_tokens method to generate all response types
impl Generator {
    pub fn generate_tokens(&mut self, spec: &OpenAPI) -> Result<TokenStream> {
        validate_openapi(spec)?;

        // Convert our components dictionary to schemars
        let schemas = spec.components.iter().flat_map(|components| {
            components
                .schemas
                .iter()
                .map(|(name, ref_or_schema)| {
                    let schema = ref_or_schema.resolve(spec)?;
                    let name = format!("_{}", name);
                    Ok((name, schema))
                })
                .collect::<Result<Vec<_>>>()?
        }).collect::<Vec<_>>();

        // Generate all response types
        let mut response_types = Vec::new();
        for method in &self.operations {
            // Generate success response types
            /*let (_, success_response_kind) = 
                self.extract_responses(method, OperationResponseStatus::is_success_or_default);
            
            if let Some(type_def) = self.generate_response_enum(method, &success_response_kind)? {
                response_types.push(type_def);
            }*/
            
            // Generate error response types
            let (_, error_response_kind) = 
                self.extract_responses(method, OperationResponseStatus::is_error);
            
            if let Some(type_def) = self.generate_response_enum(method, &error_response_kind)? {
                response_types.push(type_def);
            }
        }
        
        // Add the response types to the types module
        let types_module = quote! {
            /// Types used in the API.
            pub mod types {
                use serde::{Deserialize, Serialize};
                
                #(#type_definitions)*
                
                #(#response_types)*
            }
        };
        
        // ... rest of the method ...
    }
}
