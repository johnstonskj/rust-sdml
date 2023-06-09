/*!
Contains only string constants for the tree-sitter node types and field names.
 */

pub const NAME_SDML: &str = "sdml";

// ------------------------------------------------------------------------------------------------
// Grammar Node kinds
// ------------------------------------------------------------------------------------------------

pub const NODE_KIND_ANNOTATION: &str = "annotation";
pub const NODE_KIND_ANNOTATION_PROPERTY: &str = "annotation_property";
pub const NODE_KIND_ANNOTATION_ONLY_BODY: &str = "annotation_only_body";

pub const NODE_KIND_BOOLEAN: &str = "boolean";
pub const NODE_KIND_BUILTIN_SIMPLE_TYPE: &str = "builtin_simple_type";

pub const NODE_KIND_CARDINALITY_EXPRESSION: &str = "cardinality_expression";
pub const NODE_KIND_CONSTRAINT: &str = "constraint";

pub const NODE_KIND_DATA_TYPE_DEF: &str = "data_type_def";
pub const NODE_KIND_DECIMAL: &str = "decimal";
pub const NODE_KIND_DEFINITION: &str = "definition";
pub const NODE_KIND_DOUBLE: &str = "double";

pub const NODE_KIND_ENTITY_BODY: &str = "entity_body";
pub const NODE_KIND_ENTITY_DEF: &str = "entity_def";
pub const NODE_KIND_ENTITY_GROUP: &str = "entity_group";
pub const NODE_KIND_ENUM_BODY: &str = "enum_body";
pub const NODE_KIND_ENUM_DEF: &str = "enum_def";
pub const NODE_KIND_EVENT_DEF: &str = "event_def";

pub const NODE_KIND_FORMAL_CONSTRAINT: &str = "formal_constraint";

pub const NODE_KIND_IDENTIFIER: &str = "identifier";
pub const NODE_KIND_IDENTIFIER_REFERENCE: &str = "identifier_reference";
pub const NODE_KIND_IDENTITY_MEMBER: &str = "identity_member";
pub const NODE_KIND_IMPORT: &str = "import";
pub const NODE_KIND_IMPORT_STATEMENT: &str = "import_statement";
pub const NODE_KIND_INFORMAL_CONSTRAINT: &str = "informal_constraint";
pub const NODE_KIND_INTEGER: &str = "integer";
pub const NODE_KIND_IRI_REFERENCE: &str = "iri_reference";

pub const NODE_KIND_LANGUAGE_TAG: &str = "language_tag";
pub const NODE_KIND_LINE_COMMENT: &str = "line_comment";
pub const NODE_KIND_LIST_OF_VALUES: &str = "list_of_values";

pub const NODE_KIND_MEMBER_BY_VALUE: &str = "member_by_value";
pub const NODE_KIND_MEMBER_BY_REFERENCE: &str = "member_by_reference";
pub const NODE_KIND_MEMBER_IMPORT: &str = "member_import";
pub const NODE_KIND_MODULE: &str = "module";
pub const NODE_KIND_MODULE_BODY: &str = "module_body";
pub const NODE_KIND_MODULE_IMPORT: &str = "module_import";

pub const NODE_KIND_PROPERTY_BODY: &str = "property_body";
pub const NODE_KIND_PROPERTY_DEF: &str = "property_def";
pub const NODE_KIND_PROPERTY_MEMBER: &str = "property_member";
pub const NODE_KIND_PROPERTY_ROLE: &str = "property_role";

pub const NODE_KIND_QUALIFIED_IDENTIFIER: &str = "qualified_identifier";
pub const NODE_KIND_QUOTED_STRING: &str = "quoted_string";

pub const NODE_KIND_SIMPLE_VALUE: &str = "simple_value";
pub const NODE_KIND_STRING: &str = "string";
pub const NODE_KIND_STRUCTURE_BODY: &str = "structure_body";
pub const NODE_KIND_STRUCTURE_DEF: &str = "structure_def";
pub const NODE_KIND_STRUCTURE_GROUP: &str = "structure_group";
pub const NODE_KIND_STRUCTURE_MEMBER: &str = "structure_member";

pub const NODE_KIND_TYPE_VARIANT: &str = "type_variant";

pub const NODE_KIND_UNION_BODY: &str = "union_body";
pub const NODE_KIND_UNION_DEF: &str = "union_def";
pub const NODE_KIND_UNKNOWN_TYPE: &str = "unknown_type";
pub const NODE_KIND_UNSIGNED: &str = "unsigned";

pub const NODE_KIND_VALUE_CONSTRUCTOR: &str = "value_constructor";
pub const NODE_KIND_VALUE_VARIANT: &str = "value_variant";

// ------------------------------------------------------------------------------------------------
// Grammar Node field names
// ------------------------------------------------------------------------------------------------

pub const FIELD_NAME_BASE: &str = "base";
pub const FIELD_NAME_BODY: &str = "body";

pub const FIELD_NAME_IDENTITY: &str = "identity";

pub const FIELD_NAME_LANGUAGE: &str = "language";

pub const FIELD_NAME_MEMBER: &str = "member";
pub const FIELD_NAME_MAX: &str = "max";
pub const FIELD_NAME_MIN: &str = "min";
pub const FIELD_NAME_MODULE: &str = NODE_KIND_MODULE;

pub const FIELD_NAME_NAME: &str = "name";

pub const FIELD_NAME_RENAME: &str = "rename";
pub const FIELD_NAME_ROLE: &str = "role";

pub const FIELD_NAME_SOURCE: &str = "source";
pub const FIELD_NAME_SOURCE_CARDINALITY: &str = "source_cardinality";

pub const FIELD_NAME_TARGET: &str = "target";
pub const FIELD_NAME_TARGET_CARDINALITY: &str = "target_cardinality";

pub const FIELD_NAME_VALUE: &str = "value";

// ------------------------------------------------------------------------------------------------
// Simple Type Keywords
// ------------------------------------------------------------------------------------------------

// pub const KW_TYPE_STRING: &str = NODE_KIND_STRING;
// pub const KW_TYPE_DOUBLE: &str = NODE_KIND_DOUBLE;
// pub const KW_TYPE_DECIMAL: &str = NODE_KIND_DECIMAL;
// pub const KW_TYPE_INTEGER: &str = NODE_KIND_INTEGER;
// pub const KW_TYPE_BOOLEAN: &str = NODE_KIND_BOOLEAN;
// pub const KW_TYPE_IRI: &str = "iri";
