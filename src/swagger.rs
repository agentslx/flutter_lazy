// swagger.rs
// Module for parsing Swagger/OpenAPI specs and generating features

use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use std::fs;
use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use serde::Deserialize;
use reqwest;
use console::style;

use crate::features::{FeatureParams, create_feature};

/// Source of Swagger/OpenAPI specification
#[derive(Clone)]
pub enum SwaggerSource {
    Url(String),
    File(PathBuf),
}

/// Simple representation of a Swagger/OpenAPI spec
#[derive(Debug, Deserialize)]
struct SwaggerSpec {
    #[serde(default)]
    info: SwaggerInfo,
    #[serde(default)]
    tags: Vec<SwaggerTag>,
    #[serde(default)]
    paths: HashMap<String, HashMap<String, SwaggerOperation>>,
    #[serde(default, alias = "definitions")]
    schemas: HashMap<String, SwaggerSchema>,
    #[serde(default)]
    components: Option<SwaggerComponents>,
}

#[derive(Debug, Default, Deserialize)]
struct SwaggerComponents {
    #[serde(default)]
    schemas: HashMap<String, SwaggerSchema>,
}

#[derive(Debug, Default, Deserialize)]
struct SwaggerInfo {
    #[serde(default)]
    title: String,
    #[serde(default)]
    version: String,
}

#[derive(Debug, Deserialize)]
struct SwaggerTag {
    name: String,
    #[serde(default)]
    description: String,
}

#[derive(Debug, Deserialize)]
struct SwaggerOperation {
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    summary: String,
    #[serde(default)]
    description: String,
    #[serde(default, rename = "operationId")]
    operation_id: String,
    #[serde(default)]
    parameters: Vec<SwaggerParameter>,
    #[serde(default, rename = "responses")]
    responses: HashMap<String, SwaggerResponse>,
}

#[derive(Debug, Deserialize, Clone)]
struct SwaggerParameter {
    #[serde(default)]
    name: String,
    #[serde(default)]
    r#in: String,  // "path", "query", "header", "body"
    #[serde(default)]
    required: bool,
    #[serde(default)]
    schema: Option<SwaggerSchema>,
}

#[derive(Debug, Deserialize)]
struct SwaggerResponse {
    #[serde(default)]
    description: String,
    #[serde(default)]
    schema: Option<SwaggerSchema>,
}

#[derive(Debug, Deserialize, Clone)]
struct SwaggerSchema {
    #[serde(default)]
    r#type: String,
    #[serde(default, rename = "ref")]
    #[serde(alias = "$ref")]
    reference: Option<String>,
    #[serde(default)]
    properties: HashMap<String, SwaggerProperty>,
    #[serde(default)]
    required: Vec<String>,
    #[serde(default)]
    description: String,
}

#[derive(Debug, Deserialize, Clone)]
struct SwaggerProperty {
    #[serde(default)]
    r#type: String,
    #[serde(default)]
    format: Option<String>,
    #[serde(default, rename = "ref")]
    #[serde(alias = "$ref")]
    reference: Option<String>,
    #[serde(default)]
    items: Option<Box<SwaggerSchema>>,
    #[serde(default)]
    description: String,
}

/// Generate features based on Swagger/OpenAPI specification
pub fn generate_api_features(
    project_dir: &Path,
    source: SwaggerSource,
    domain_filter: Option<Vec<String>>,
    data_only: bool,
) -> Result<()> {
    // Load the Swagger spec
    let spec = load_swagger_spec(source)?;
    
    println!("Loaded API: {} (v{})", style(&spec.info.title).bold(), spec.info.version);
    
    // Extract all tags/domains
    let mut domains = extract_domains(&spec)?;
    
    // Apply filter if provided
    if let Some(filter) = domain_filter {
        domains.retain(|domain| filter.contains(&domain.name));
    }
    
    println!("Found {} domains/tags in the API", style(domains.len()).bold());
    
    // Extract schemas for model generation
    let schemas = extract_schemas(&spec)?;
    println!("Found {} data models in API schemas", style(schemas.len()).bold());
    
    // Generate features for each domain
    for domain in domains {
        println!("\n🔹 Generating feature for domain: {}", style(&domain.name).bold());
        
        // Create a feature name from the domain
        let feature_name = domain.name.to_case(Case::Snake);
        
        // Set up feature parameters
        let mut params = FeatureParams::new(&feature_name);
        
        // If data_only is true, disable state management (cubit)
        if data_only {
            params.has_state_management = false;
        }
        
        // Create the base feature structure
        create_feature(project_dir, params)?;
        
        // Generate model classes based on response schemas used in this domain
        generate_domain_models(project_dir, &feature_name, &domain, &schemas)?;
        
        // Generate the datasource and repository implementations
        generate_domain_datasources(project_dir, &feature_name, &domain, &spec)?;
    }
    
    Ok(())
}

/// Load Swagger spec from URL or file
fn load_swagger_spec(source: SwaggerSource) -> Result<SwaggerSpec> {
    match source {
        SwaggerSource::Url(url) => {
            // Make HTTP request to fetch the spec
            println!("Fetching Swagger specification from URL...");
            
            let response = reqwest::blocking::get(&url)
                .context(format!("Failed to fetch Swagger spec from {}", &url))?;
            
            if !response.status().is_success() {
                return Err(anyhow::anyhow!(
                    "Failed to fetch Swagger spec, status code: {}", 
                    response.status()
                ));
            }
            
            let spec: SwaggerSpec = response.json()
                .context("Failed to parse Swagger JSON")?;
            
            Ok(spec)
        },
        SwaggerSource::File(path) => {
            // Read the file content
            println!("Reading Swagger specification from file...");
            
            let file_content = fs::read_to_string(&path)
                .context(format!("Failed to read Swagger file: {:?}", path))?;
            
            let spec: SwaggerSpec = serde_json::from_str(&file_content)
                .context("Failed to parse Swagger JSON")?;
            
            Ok(spec)
        }
    }
}

/// Extract all domains/tags from the Swagger spec
/// A domain is represented by a tag in the Swagger spec
struct Domain {
    name: String,
    description: String,
    endpoints: Vec<Endpoint>,
}

#[derive(Clone)]
struct Endpoint {
    path: String,
    method: String,
    operation_id: String,
    summary: String,
    parameters: Vec<SwaggerParameter>,
    response_type: Option<String>,
}

fn extract_domains(spec: &SwaggerSpec) -> Result<Vec<Domain>> {
    // First, collect all tags as domains
    let mut domains: HashMap<String, Domain> = spec.tags.iter()
        .map(|tag| {
            (tag.name.clone(), Domain {
                name: tag.name.clone(),
                description: tag.description.clone(),
                endpoints: Vec::new(),
            })
        })
        .collect();
    
    // Then associate each path/operation with its tag/domain
    for (path, methods) in &spec.paths {
        for (method, operation) in methods {
            // Skip if no tags
            if operation.tags.is_empty() {
                continue;
            }
            
            // Create endpoint
            let endpoint = Endpoint {
                path: path.clone(),
                method: method.to_uppercase(),
                operation_id: operation.operation_id.clone(),
                summary: operation.summary.clone(),
                parameters: operation.parameters.clone(),
                response_type: extract_response_type(&operation.responses),
            };
            
            // Add to each associated domain
            for tag in &operation.tags {
                if let Some(domain) = domains.get_mut(tag) {
                    domain.endpoints.push(endpoint.clone());
                } else {
                    // Create a new domain if the tag wasn't defined in the tags section
                    domains.insert(tag.clone(), Domain {
                        name: tag.clone(),
                        description: String::new(),
                        endpoints: vec![endpoint.clone()],
                    });
                }
            }
        }
    }
    
    Ok(domains.into_values().collect())
}

/// Extract response type from operation responses
fn extract_response_type(responses: &HashMap<String, SwaggerResponse>) -> Option<String> {
    // Look for 200/201 responses first
    for status in ["200", "201"] {
        if let Some(response) = responses.get(status) {
            if let Some(schema) = &response.schema {
                if let Some(ref_str) = &schema.reference {
                    // Extract the model name from the reference
                    // Typical format: "#/definitions/User" or "#/components/schemas/User"
                    if let Some(model_name) = ref_str.split('/').last() {
                        return Some(model_name.to_string());
                    }
                }
            }
        }
    }
    None
}

/// Extract schemas for model generation
struct SchemaInfo {
    name: String,
    properties: HashMap<String, PropertyInfo>,
    required: Vec<String>,
    description: String,
}

impl SchemaInfo {
    fn has_date_time_field(&self) -> bool {
        self.properties.values().any(|prop| prop.type_name == "dateTime")
    }

    fn has_list_field(&self) -> bool {
        self.properties.values().any(|prop| prop.is_list)
    }
}

struct PropertyInfo {
    name: String,
    type_name: String,
    is_list: bool,
    is_nullable: bool,
    description: String,
    reference: Option<String>,
}

fn extract_schemas(spec: &SwaggerSpec) -> Result<HashMap<String, SchemaInfo>> {
    let mut result = HashMap::new();
    
    // In OpenAPI 3.0, schemas are under components/schemas
    if let Some(components) = &spec.components {
        for (name, schema) in &components.schemas {
            let schema_info = extract_schema_info(name, schema);
            result.insert(name.clone(), schema_info);
        }
    }
    
    // In Swagger 2.0, schemas are under definitions
    for (name, schema) in &spec.schemas {
        let schema_info = extract_schema_info(name, schema);
        result.insert(name.clone(), schema_info);
    }
    
    Ok(result)
}

fn extract_schema_info(name: &str, schema: &SwaggerSchema) -> SchemaInfo {
    let mut properties = HashMap::new();
    
    for (prop_name, prop) in &schema.properties {
        let type_name = determine_property_type(prop);
        let is_list = prop.r#type == "array";
        let is_nullable = !schema.required.contains(prop_name);
        
        let reference = if let Some(ref_path) = &prop.reference {
            // Extract model name from reference (e.g., "#/definitions/User" -> "User")
            ref_path.split('/').last().map(|s| s.to_string())
        } else if let Some(items) = &prop.items {
            // For array items with a reference
            items.reference.as_ref().and_then(|ref_path| {
                ref_path.split('/').last().map(|s| s.to_string())
            })
        } else {
            None
        };
        
        properties.insert(prop_name.clone(), PropertyInfo {
            name: prop_name.clone(),
            type_name,
            is_list,
            is_nullable,
            description: prop.description.clone(),
            reference,
        });
    }
    
    SchemaInfo {
        name: name.to_string(),
        properties,
        required: schema.required.clone(),
        description: schema.description.clone(),
    }
}

fn determine_property_type(prop: &SwaggerProperty) -> String {
    if let Some(ref_path) = &prop.reference {
        // It's a reference to another schema
        if let Some(model_name) = ref_path.split('/').last() {
            return model_name.to_string();
        }
    }
    
    if prop.r#type == "array" {
        if let Some(items) = &prop.items {
            if let Some(ref_path) = &items.reference {
                // Array of references
                if let Some(model_name) = ref_path.split('/').last() {
                    return format!("List<{}>", model_name);
                }
            }
            // Array of primitive type
            return format!("List<{}>", map_type(&items.r#type));
        }
        return "List<dynamic>".to_string();
    }
    
    // Map Swagger/OpenAPI types to Dart types
    map_type(&prop.r#type)
}

fn map_type(swagger_type: &str) -> String {
    match swagger_type {
        "integer" => "int".to_string(),
        "number" => "double".to_string(),
        "boolean" => "bool".to_string(),
        "string" => "String".to_string(),
        "object" => "Map<String, dynamic>".to_string(),
        "" => "dynamic".to_string(),
        _ => "dynamic".to_string(),
    }
}

/// Generate model classes for a domain
fn generate_domain_models(
    project_dir: &Path,
    feature_name: &str,
    domain: &Domain,
    schemas: &HashMap<String, SchemaInfo>,
) -> Result<()> {
    // Create models directory
    let models_dir = project_dir.join("lib/features").join(feature_name).join("data/models");
    fs::create_dir_all(&models_dir).context("Failed to create models directory")?;
    
    // Create entities directory structure under core/entities/{feature_name}
    let core_entities_dir = project_dir.join("lib/core/entities").join(feature_name);
    fs::create_dir_all(&core_entities_dir).context("Failed to create core entities directory")?;
    
    // Find all response types used in the domain
    let mut domain_models = HashSet::new();
    
    for endpoint in &domain.endpoints {
        if let Some(response_type) = &endpoint.response_type {
            domain_models.insert(response_type.clone());
        }
    }
    
    // Generate models and entities for the domain
    for model_name in domain_models {
        if let Some(schema) = schemas.get(&model_name) {
            // Generate model (data layer)
            let model_file_path = models_dir.join(format!("{}_model.dart", model_name.to_case(Case::Snake)));
            generate_model_class(&model_file_path, schema)?;
            println!("  ✓ Generated model: {}", style(&model_name).bold());
            
            // Generate entity (domain layer)
            let entity_name = if model_name.ends_with("Model") {
                model_name.replace("Model", "")
            } else {
                model_name.clone()
            };
            
            let entity_file_path = core_entities_dir.join(format!("{}.dart", entity_name.to_case(Case::Snake)));
            generate_entity_class(&entity_file_path, schema, &entity_name)?;
            println!("  ✓ Generated entity: {}", style(&entity_name).bold());
        }
    }
    
    Ok(())
}

/// Generate a Dart model class from a schema
fn generate_model_class(
    file_path: &Path,
    schema: &SchemaInfo,
) -> Result<()> {
    let class_name = format!("{}Model", schema.name);
    let entity_name = schema.name.clone();
    let feature_name = get_feature_name_from_file_path(file_path)?;
    
    let mut content = String::new();
    
    // Add imports
    content.push_str("import 'package:json_annotation/json_annotation.dart';\n");
    
    // Add documentation if available
    if !schema.description.is_empty() {
        content.push_str("/// ");
        content.push_str(&schema.description);
        content.push_str("\n");
    }
    
    // Add class annotation
    content.push_str("@JsonSerializable()\n");
    content.push_str(&format!("class {} {{\n", class_name));
    
    // Add properties
    for (name, prop) in &schema.properties {
        // Add documentation if available
        if !prop.description.is_empty() {
            content.push_str("  /// ");
            content.push_str(&prop.description);
            content.push_str("\n");
        }
        
        // Add JsonKey annotation if the property name is not a valid Dart identifier
        if name.contains("-") || name.contains(".") || name.contains(" ") {
            content.push_str(&format!("  @JsonKey(name: '{}')\n", name));
        }
        
        // Add property declaration
        let dart_type = if prop.is_nullable {
            format!("{}?", prop.type_name)
        } else {
            prop.type_name.clone()
        };
        
        let dart_name = name.to_case(Case::Camel);
        content.push_str(&format!("  final {} {};\n\n", dart_type, dart_name));
    }
    
    // Add constructor
    content.push_str(&format!("  {}({{\n", class_name));
    for (name, _prop) in &schema.properties {
        let dart_name = name.to_case(Case::Camel);
        if schema.required.contains(name) {
            content.push_str(&format!("    required this.{},\n", dart_name));
        } else {
            content.push_str(&format!("    this.{},\n", dart_name));
        }
    }
    content.push_str("  });\n\n");
    
    // Add fromJson and toJson methods
    content.push_str(&format!("  factory {}.fromJson(Map<String, dynamic> json) => _${}FromJson(json);\n\n", class_name, class_name));
    content.push_str(&format!("  Map<String, dynamic> toJson() => _${}ToJson(this);\n", class_name));
    
    // Add entity conversion methods
    update_model_class_with_entity_conversion(&mut content, schema, &class_name, &entity_name, &feature_name);
    
    // Close class definition
    content.push_str("}\n");
    
    // Write to file
    fs::write(file_path, content).context("Failed to write model file")?;
    
    Ok(())
}

/// Generate a Dart entity class from a schema
fn generate_entity_class(
    file_path: &Path,
    schema: &SchemaInfo,
    entity_name: &str,
) -> Result<()> {
    let mut content = String::new();
    
    // Add imports if needed
    if schema.has_date_time_field() || schema.has_list_field() {
        content.push_str("import 'package:intl/intl.dart';\n");
    }
    content.push_str("\n");
    
    // Add documentation if available
    if !schema.description.is_empty() {
        content.push_str("/// ");
        content.push_str(&schema.description);
        content.push_str("\n");
    }
    
    // Add entity class
    content.push_str(&format!("class {} {{\n", entity_name));
    
    // Add properties
    for (name, prop) in &schema.properties {
        // Add documentation if available
        if !prop.description.is_empty() {
            content.push_str("  /// ");
            content.push_str(&prop.description);
            content.push_str("\n");
        }
        
        // Add property declaration
        let dart_type = if prop.is_nullable {
            format!("{}?", prop.type_name)
        } else {
            prop.type_name.clone()
        };
        
        let dart_name = name.to_case(Case::Camel);
        content.push_str(&format!("  final {} {};\n\n", dart_type, dart_name));
    }
    
    // Add constructor
    content.push_str(&format!("  const {}({{\n", entity_name));
    for (name, _prop) in &schema.properties {
        let dart_name = name.to_case(Case::Camel);
        if schema.required.contains(name) {
            content.push_str(&format!("    required this.{},\n", dart_name));
        } else {
            content.push_str(&format!("    this.{},\n", dart_name));
        }
    }
    content.push_str("  });\n\n");
    
    // Add copyWith method for immutability
    content.push_str(&format!("  {} copyWith({{\n", entity_name));
    for (name, prop) in &schema.properties {
        let dart_name = name.to_case(Case::Camel);
        let dart_type = if prop.is_nullable {
            format!("{}?", prop.type_name)
        } else {
            prop.type_name.clone()
        };
        content.push_str(&format!("    {}? {},\n", dart_type, dart_name));
    }
    content.push_str("  }) {\n");
    content.push_str(&format!("    return {}(\n", entity_name));
    for (name, _) in &schema.properties {
        let dart_name = name.to_case(Case::Camel);
        content.push_str(&format!("      {}: {} ?? this.{},\n", dart_name, dart_name, dart_name));
    }
    content.push_str("    );\n");
    content.push_str("  }\n");

    // Close class
    content.push_str("}\n");
    
    // Write the content to the file
    fs::write(file_path, content).context("Failed to write entity file")?;
    
    Ok(())
}

/// Generate datasource and repository implementations for a domain
fn generate_domain_datasources(
    project_dir: &Path,
    feature_name: &str,
    domain: &Domain,
    _spec: &SwaggerSpec,
) -> Result<()> {
    // Paths for the files we'll generate
    let repo_dir = project_dir.join("lib/features").join(feature_name).join("data/repository");
    let datasource_dir = project_dir.join("lib/features").join(feature_name).join("data/datasources");
    
    fs::create_dir_all(&repo_dir).context("Failed to create repository directory")?;
    fs::create_dir_all(&datasource_dir).context("Failed to create datasources directory")?;
    
    // Generate remote datasource
    let remote_ds_path = datasource_dir.join(format!("{}_remote_datasource.dart", feature_name));
    generate_remote_datasource(&remote_ds_path, feature_name, domain)?;
    
    // Generate local datasource
    let local_ds_path = datasource_dir.join(format!("{}_local_datasource.dart", feature_name));
    generate_local_datasource(&local_ds_path, feature_name, domain)?;
    
    // Generate repository
    let repo_path = repo_dir.join(format!("{}_repository.dart", feature_name));
    generate_repository(&repo_path, feature_name, domain)?;
    
    println!("✅ Generated data layer for domain: {}", style(&domain.name).bold());
    
    Ok(())
}

/// Generate remote datasource implementation
fn generate_remote_datasource(
    file_path: &Path,
    feature_name: &str,
    domain: &Domain,
) -> Result<()> {
    let pascal_name = feature_name.to_case(Case::Pascal);
    
    let mut content = format!(
        "import 'package:dio/dio.dart';
import 'package:injectable/injectable.dart';

import '../../../../core/api/api_client.dart';
import '../models/{}_model.dart';

abstract class {}_RemoteDatasource {{
", feature_name, pascal_name);

    // Add method signatures for each endpoint
    for endpoint in &domain.endpoints {
        let method_name = endpoint.operation_id.to_case(Case::Camel);
        let return_type = endpoint.response_type
            .as_ref()
            .map(|t| format!("Future<{}>", t))
            .unwrap_or_else(|| "Future<void>".to_string());
        
        // Add method signature
        content.push_str(&format!("  /// {}\n", endpoint.summary));
        content.push_str(&format!("  {} {}(", return_type, method_name));
        
        // Add parameters
        let params: Vec<String> = endpoint.parameters
            .iter()
            .filter(|p| p.r#in == "path" || p.r#in == "query" || p.r#in == "body")
            .map(|p| {
                let required = if p.required { "required " } else { "" };
                format!("{}dynamic {}", required, p.name)
            })
            .collect();
        
        content.push_str(&params.join(", "));
        content.push_str(");\n");
    }
    
    // Add implementation class
    content.push_str(&format!(
        "}}

@Injectable(as: {}_RemoteDatasource)
class {}_RemoteDatasourceImpl implements {}_RemoteDatasource {{
  final ApiClient _apiClient;

  {}_RemoteDatasourceImpl(this._apiClient);

", pascal_name, pascal_name, pascal_name, pascal_name));

    // Add method implementations
    for endpoint in &domain.endpoints {
        let method_name = endpoint.operation_id.to_case(Case::Camel);
        let return_type = endpoint.response_type
            .as_ref()
            .map(|t| format!("Future<{}>", t))
            .unwrap_or_else(|| "Future<void>".to_string());
        
        // Add method implementation
        content.push_str(&format!("  @override\n"));
        content.push_str(&format!("  {} {}(", return_type, method_name));
        
        // Add parameters
        let params: Vec<String> = endpoint.parameters
            .iter()
            .filter(|p| p.r#in == "path" || p.r#in == "query" || p.r#in == "body")
            .map(|p| {
                let required = if p.required { "required " } else { "" };
                format!("{}dynamic {}", required, p.name)
            })
            .collect();
        
        content.push_str(&params.join(", "));
        content.push_str(") async {\n");
        
        // Implementation
        let method = endpoint.method.to_lowercase();
        let path = endpoint.path.clone();
        
        content.push_str(&format!(
            "    final response = await _apiClient.{}('{}', ",
            method, path
        ));
        
        // Add query params if any
        let query_params: Vec<String> = endpoint.parameters
            .iter()
            .filter(|p| p.r#in == "query")
            .map(|p| format!("'{}': {}", p.name, p.name))
            .collect();
        
        if !query_params.is_empty() {
            content.push_str("queryParameters: {");
            content.push_str(&query_params.join(", "));
            content.push_str("}, ");
        }
        
        // Add body if any
        let body_param = endpoint.parameters
            .iter()
            .find(|p| p.r#in == "body");
        
        if let Some(body) = body_param {
            content.push_str(&format!("data: {}", body.name));
        }
        
        content.push_str(");\n");
        
        // Return statement based on return type
        if let Some(response_type) = &endpoint.response_type {
            content.push_str(&format!(
                "    return {}Model.fromJson(response.data);\n",
                response_type
            ));
        } else {
            content.push_str("    return;\n");
        }
        
        content.push_str("  }\n\n");
    }
    
    // Close the class
    content.push_str("}");
    
    // Write to file
    fs::write(file_path, content).context("Failed to write remote datasource file")?;
    
    Ok(())
}

/// Generate local datasource implementation
fn generate_local_datasource(
    file_path: &Path,
    feature_name: &str,
    _domain: &Domain,
) -> Result<()> {
    let pascal_name = feature_name.to_case(Case::Pascal);
    
    let content = format!(
        "import 'package:injectable/injectable.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../models/{}_model.dart';

abstract class {}_LocalDatasource {{
  Future<void> cache{}Data(dynamic data);
  Future<dynamic> get{}Data();
}}

@Injectable(as: {}_LocalDatasource)
class {}_LocalDatasourceImpl implements {}_LocalDatasource {{
  final SharedPreferences _preferences;
  static const String {}DataKey = '{}_data';

  {}_LocalDatasourceImpl(this._preferences);

  @override
  Future<void> cache{}Data(dynamic data) async {{
    // Implementation depends on the data type - this is a basic example
    if (data is String) {{
      await _preferences.setString({}DataKey, data);
    }}
    // Add more type handling as needed
  }}

  @override
  Future<dynamic> get{}Data() async {{
    // Basic implementation - enhance as needed
    final data = _preferences.getString({}DataKey);
    if (data != null) {{
      // Parse the data based on your needs
      return data;
    }}
    return [];
  }}
}}
",
        feature_name, // Import path
        pascal_name, // Class name abstract
        pascal_name, // Method cache
        pascal_name, // Method get
        pascal_name, // Injectable
        pascal_name, // Impl class
        pascal_name, // Implements
        feature_name, // Key const
        feature_name, // Key value
        pascal_name, // Constructor
        pascal_name, // Override cache
        feature_name, // Key reference
        pascal_name, // Override get
        feature_name, // Key reference
    );
    
    // Write to file
    fs::write(file_path, content).context("Failed to write local datasource file")?;
    
    Ok(())
}

/// Generate repository implementation
fn generate_repository(
    file_path: &Path,
    feature_name: &str,
    domain: &Domain,
) -> Result<()> {
    let pascal_name = feature_name.to_case(Case::Pascal);
    
    let mut content = generate_repository_formatted_content(feature_name, &pascal_name);
    
    // Add method signatures for each endpoint
    for endpoint in &domain.endpoints {
        let method_name = endpoint.operation_id.to_case(Case::Camel);
        let entity_type = endpoint.response_type
            .as_ref()
            .map(|t| if t.ends_with("Model") { t[0..t.len()-5].to_string() } else { t.clone() });
            
        let return_type = entity_type
            .as_ref()
            .map(|t| format!("Future<Either<Failure, {}>>", t))
            .unwrap_or_else(|| "Future<Either<Failure, void>>".to_string());
        
        // Add method signature
        content.push_str(&format!("  /// {}\n", endpoint.summary));
        content.push_str(&format!("  {} {}(", return_type, method_name));
        
        // Add parameters
        let params: Vec<String> = endpoint.parameters
            .iter()
            .filter(|p| p.r#in == "path" || p.r#in == "query" || p.r#in == "body")
            .map(|p| {
                let required = if p.required { "required " } else { "" };
                format!("{}dynamic {}", required, p.name)
            })
            .collect();
        
        content.push_str(&params.join(", "));
        content.push_str(");\n");
    }
    
    // Add implementation class
    content.push_str(&format!(
        "}}

@Injectable(as: {}_Repository)
class {}_RepositoryImpl implements {}_Repository {{
  final {}_RemoteDatasource _remoteDatasource;
  final {}_LocalDatasource _localDatasource;

  {}_RepositoryImpl(this._remoteDatasource, this._localDatasource);

",
        pascal_name, // Injectable
        pascal_name, // Impl class
        pascal_name, // Implements
        pascal_name, // Remote DS
        pascal_name, // Local DS
        pascal_name, // Constructor
    ));
    
    // Add method implementations
    for endpoint in &domain.endpoints {
        let method_name = endpoint.operation_id.to_case(Case::Camel);
        let response_type = endpoint.response_type.as_deref().unwrap_or("void");
        let return_type = format!("Future<Either<Failure, {}>>", response_type);
        
        // Add method implementation
        content.push_str(&format!("  @override\n"));
        content.push_str(&format!("  {} {}(", return_type, method_name));
        
        // Add parameters
        let params: Vec<String> = endpoint.parameters
            .iter()
            .filter(|p| p.r#in == "path" || p.r#in == "query" || p.r#in == "body")
            .map(|p| {
                let required = if p.required { "required " } else { "" };
                format!("{}dynamic {}", required, p.name)
            })
            .collect();
        
        content.push_str(&params.join(", "));
        content.push_str(") async {\n");
        
        // Error handling implementation
        content.push_str("    try {\n");
        
        // Call to remote datasource
        content.push_str(&format!("      final modelResult = await _remoteDatasource.{}(", method_name));
        
        // Add parameter names
        let param_names: Vec<String> = endpoint.parameters
            .iter()
            .filter(|p| p.r#in == "path" || p.r#in == "query" || p.r#in == "body")
            .map(|p| p.name.clone())
            .collect();
        
        content.push_str(&param_names.join(", "));
        content.push_str(");\n");
        
        // Convert model to entity if there is a response type
        if let Some(_model_type) = &endpoint.response_type {
            // Convert model to entity
            content.push_str(&format!("      final entity = modelResult.toEntity();\n"));
            
            // Cache result if needed
            if endpoint.method == "GET" {
                content.push_str("      await _localDatasource.cacheData(modelResult);\n");
            }
            
            // Return successful entity
            content.push_str(&format!("      return Right(entity);\n"));
        } else {
            // Return successful void result
            content.push_str(&format!("      return const Right(null);\n"));
        }
        
        // Error handling
        content.push_str("    } on DioException catch (e) {\n");
        content.push_str("      return Left(NetworkFailure(message: e.message ?? 'Network error'));\n");
        content.push_str("    } catch (e) {\n");
        content.push_str("      return Left(UnexpectedFailure(message: e.toString()));\n");
        content.push_str("    }\n");
        
        content.push_str("  }\n\n");
    }
    
    // Close the class
    content.push_str("}");
    
    // Write to file
    fs::write(file_path, content).context("Failed to write repository file")?;
    
    Ok(())
}

/// Helper function to extract feature name from file path
fn get_feature_name_from_file_path(file_path: &Path) -> Result<String> {
    // Extract feature name from path like "lib/features/{feature_name}/data/models/..."
    let path_str = file_path.to_string_lossy();
    let features_pattern = "lib/features/";
    let data_pattern = "/data/";
    
    if let Some(start_idx) = path_str.find(features_pattern) {
        let feature_start = start_idx + features_pattern.len();
        if let Some(end_idx) = path_str[feature_start..].find(data_pattern) {
            let feature_name = &path_str[feature_start..feature_start + end_idx];
            return Ok(feature_name.to_string());
        }
    }
    
    // Default fallback
    Ok("unknown".to_string())
}

/// Update the model class generation to include to/from entity methods
fn update_model_class_with_entity_conversion(
    content: &mut String,
    schema: &SchemaInfo,
    model_name: &str,
    entity_name: &str,
    feature_name: &str
) {
    // Add entity import
    content.push_str(&format!("import 'package:{{package_name}}/core/entities/{}/{}.dart';\n\n", 
        feature_name,
        entity_name.to_case(Case::Snake)
    ));
    
    // After the toJson method, add toEntity method
    content.push_str(&format!("\n  // Convert to Entity\n  {} toEntity() {{\n", entity_name));
    content.push_str(&format!("    return {}(\n", entity_name));
    
    // Add properties
    for (name, _) in &schema.properties {
        let dart_name = name.to_case(Case::Camel);
        content.push_str(&format!("      {0}: {0},\n", dart_name));
    }
    
    content.push_str("    );\n");
    content.push_str("  }\n\n");
    
    // Add fromEntity factory method
    content.push_str(&format!("  // Create from Entity\n  factory {}.fromEntity({} entity) {{\n", model_name, entity_name));
    content.push_str(&format!("    return {}(\n", model_name));
    
    // Add properties
    for (name, _) in &schema.properties {
        let dart_name = name.to_case(Case::Camel);
        content.push_str(&format!("      {0}: entity.{0},\n", dart_name));
    }
    
    content.push_str("    );\n");
    content.push_str("  }\n");
}

/// Fix repository import format
fn generate_repository_formatted_content(
    feature_name: &str, 
    pascal_name: &str
) -> String {
    format!(
        "import 'package:dartz/dartz.dart';
import 'package:dio/dio.dart';
import 'package:injectable/injectable.dart';

import '../../../../core/failures/failure.dart';
import '../../../../core/entities/{0}/{0}.dart';
import '../datasources/{0}_remote_datasource.dart';
import '../datasources/{0}_local_datasource.dart';
import '../models/{0}_model.dart';

abstract class {1}_Repository {{
",
        feature_name,
        pascal_name
    )
}
