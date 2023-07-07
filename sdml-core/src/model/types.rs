use super::{
    Annotation, ByReferenceMember, ByValueMember, Cardinality, Comment, Identifier,
    IdentifierReference, IdentityMember, Span, TypeReference,
};
use std::{collections::HashSet, fmt::Debug};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum TypeDefinition {
    Datatype(DatatypeDef),
    Entity(EntityDef),
    Enum(EnumDef),
    Event(EventDef),
    Structure(StructureDef),
    Union(UnionDef),
    Property(PropertyDef),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Datatypes
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct DatatypeDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    base_type: IdentifierReference,
    body: Option<AnnotationOnlyBody>,
}

#[derive(Clone, Debug, Default)]
pub struct AnnotationOnlyBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>, // assert!(!annotations.is_empty());
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct EntityDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    body: Option<EntityBody>,
}

#[derive(Clone, Debug)]
pub struct EntityBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    identity: IdentityMember,
    annotations: Vec<Annotation>,
    members: Vec<EntityMember>,
    groups: Vec<EntityGroup>,
}

#[derive(Clone, Debug)]
pub enum EntityMember {
    ByValue(ByValueMember),
    ByReference(ByReferenceMember),
}

#[derive(Clone, Debug, Default)]
pub struct EntityGroup {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
    members: Vec<EntityMember>, // assert!(!members.is_empty());
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct EnumDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    body: Option<EnumBody>,
}

#[derive(Clone, Debug, Default)]
pub struct EnumBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
    variants: Vec<EnumVariant>, // assert!(!variants.is_empty());
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    value: u32,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Events
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct EventDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    event_source: IdentifierReference,
    body: Option<StructureBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct StructureDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    body: Option<StructureBody>,
}

#[derive(Clone, Debug, Default)]
pub struct StructureBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>,
    groups: Vec<StructureGroup>,
}

#[derive(Clone, Debug, Default)]
pub struct StructureGroup {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
    members: Vec<ByValueMember>, // assert!(!members.is_empty());
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Unions
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct UnionDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    body: Option<UnionBody>,
}

#[derive(Clone, Debug, Default)]
pub struct UnionBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
    variants: Vec<TypeVariant>, // assert!(!variants.is_empty());
}

#[derive(Clone, Debug)]
pub struct TypeVariant {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: IdentifierReference,
    rename: Option<Identifier>,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Properties
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct PropertyDef {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    body: Option<PropertyBody>,
}

#[derive(Clone, Debug, Default)]
pub struct PropertyBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    annotations: Vec<Annotation>,
    roles: Vec<PropertyRole>, // assert!(!roles.is_empty());
}

#[derive(Clone, Debug)]
pub struct PropertyRole {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    target_type: TypeReference,
    source_cardinality: Option<Option<Cardinality>>,
    target_cardinality: Option<Cardinality>,
    body: Option<AnnotationOnlyBody>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Type Definitions
// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(TypeDefinition, Datatype, DatatypeDef);

impl_from_for_variant!(TypeDefinition, Entity, EntityDef);

impl_from_for_variant!(TypeDefinition, Enum, EnumDef);

impl_from_for_variant!(TypeDefinition, Event, EventDef);

impl_from_for_variant!(TypeDefinition, Structure, StructureDef);

impl_from_for_variant!(TypeDefinition, Union, UnionDef);

impl_from_for_variant!(TypeDefinition, Property, PropertyDef);

impl TypeDefinition {
    pub fn name(&self) -> &Identifier {
        match self {
            Self::Datatype(v) => v.name(),
            Self::Entity(v) => v.name(),
            Self::Enum(v) => v.name(),
            Self::Event(v) => v.name(),
            Self::Structure(v) => v.name(),
            Self::Union(v) => v.name(),
            Self::Property(v) => v.name(),
        }
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        match self {
            Self::Datatype(v) => v.referenced_types(),
            Self::Entity(v) => v.referenced_types(),
            Self::Enum(v) => v.referenced_types(),
            Self::Event(v) => v.referenced_types(),
            Self::Structure(v) => v.referenced_types(),
            Self::Union(v) => v.referenced_types(),
            Self::Property(v) => v.referenced_types(),
        }
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        match self {
            Self::Datatype(v) => v.referenced_annotations(),
            Self::Entity(v) => v.referenced_annotations(),
            Self::Enum(v) => v.referenced_annotations(),
            Self::Event(v) => v.referenced_annotations(),
            Self::Structure(v) => v.referenced_annotations(),
            Self::Union(v) => v.referenced_annotations(),
            Self::Property(v) => v.referenced_annotations(),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            Self::Datatype(v) => v.is_complete(),
            Self::Entity(v) => v.is_complete(),
            Self::Enum(v) => v.is_complete(),
            Self::Event(v) => v.is_complete(),
            Self::Structure(v) => v.is_complete(),
            Self::Union(v) => v.is_complete(),
            Self::Property(v) => v.is_complete(),
        }
    }

    pub fn has_ts_span(&self) -> bool {
        match self {
            Self::Datatype(v) => v.has_ts_span(),
            Self::Entity(v) => v.has_ts_span(),
            Self::Enum(v) => v.has_ts_span(),
            Self::Event(v) => v.has_ts_span(),
            Self::Structure(v) => v.has_ts_span(),
            Self::Union(v) => v.has_ts_span(),
            Self::Property(v) => v.has_ts_span(),
        }
    }

    pub fn ts_span(&self) -> Option<&Span> {
        match self {
            Self::Datatype(v) => v.ts_span(),
            Self::Entity(v) => v.ts_span(),
            Self::Enum(v) => v.ts_span(),
            Self::Event(v) => v.ts_span(),
            Self::Structure(v) => v.ts_span(),
            Self::Union(v) => v.ts_span(),
            Self::Property(v) => v.ts_span(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Datatypes
// ------------------------------------------------------------------------------------------------

impl DatatypeDef {
    type_definition_impl!(AnnotationOnlyBody, base_type, IdentifierReference);

    referenced_optional_body_annotations!();

    is_complete_fn!(true);

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        [self.base_type()].into_iter().collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl AnnotationOnlyBody {
    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    has_owned_annotations!();

    referenced_own_annotations!();
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Entities
// ------------------------------------------------------------------------------------------------

impl EntityDef {
    type_definition_impl!(EntityBody);

    referenced_optional_body_annotations!();

    is_body_complete_fn!();

    referenced_optional_body_types!();
}

// ------------------------------------------------------------------------------------------------

impl EntityBody {
    pub fn new(identity: IdentityMember) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            identity,
            annotations: Default::default(),
            members: Default::default(),
            groups: Default::default(),
        }
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    has_owned_annotations!();

    get_and_mutate_collection_of!(pub members => Vec, EntityMember);

    get_and_mutate_collection_of!(pub groups => Vec, EntityGroup);

    get_and_mutate!(pub identity => IdentityMember);

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete()) && self.groups().all(|m| m.is_complete())
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------

impl_from_for_variant!(EntityMember, ByValue, ByValueMember);

impl_from_for_variant!(EntityMember, ByReference, ByReferenceMember);

impl EntityMember {
    is_as_variant!(pub by_value => ByValue, ByValueMember);
    is_as_variant!(pub by_reference => ByReference, ByReferenceMember);

    pub fn name(&self) -> &Identifier {
        match self {
            Self::ByValue(v) => v.name(),
            Self::ByReference(v) => v.name(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        match self {
            Self::ByValue(v) => v.is_complete(),
            Self::ByReference(v) => v.is_complete(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl EntityGroup {
    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    has_owned_annotations!();

    referenced_own_annotations!();

    get_and_mutate_collection_of!(pub members => Vec, EntityMember);

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete())
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Enumerations
// ------------------------------------------------------------------------------------------------

impl EnumDef {
    type_definition_impl!(EnumBody);

    referenced_optional_body_annotations!();

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.body.is_some()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        Default::default()
    }
}

// ------------------------------------------------------------------------------------------------

impl EnumBody {
    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    has_owned_annotations!();

    // --------------------------------------------------------------------------------------------

    get_and_mutate_collection_of!(pub variants => Vec, EnumVariant);

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        let mut body: HashSet<&IdentifierReference> = self
            .annotations()
            .filter_map(|a| {
                if let Annotation::Property(aprop) = a {
                    Some(aprop.name())
                } else {
                    None
                }
            })
            .collect();
        let variants: HashSet<&IdentifierReference> = self
            .variants()
            .flat_map(|v| v.referenced_annotations())
            .collect();
        body.extend(variants);
        body
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        Default::default()
    }
}

// ------------------------------------------------------------------------------------------------

impl EnumVariant {
    pub fn new(name: Identifier, value: u32) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            value,
            body: None,
        }
    }

    pub fn new_with(name: Identifier, value: u32, body: AnnotationOnlyBody) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            value,
            body: Some(body),
        }
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    get_and_mutate!(pub body => option AnnotationOnlyBody);

    referenced_optional_body_annotations!();

    get_and_mutate!(pub name => Identifier);

    get_and_mutate!(pub value => copy u32);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Events
// ------------------------------------------------------------------------------------------------

impl EventDef {
    type_definition_impl!(StructureBody, event_source, IdentifierReference);

    referenced_optional_body_annotations!();

    is_body_complete_fn!();

    referenced_optional_body_types!();
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

impl StructureDef {
    type_definition_impl!(StructureBody);

    referenced_optional_body_annotations!();

    is_body_complete_fn!();

    referenced_optional_body_types!();
}

// ------------------------------------------------------------------------------------------------

impl StructureBody {
    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    has_owned_annotations!();

    get_and_mutate_collection_of!(pub members => Vec, ByValueMember);

    get_and_mutate_collection_of!(pub groups => Vec, StructureGroup);

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete()) && self.groups().all(|m| m.is_complete())
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------

impl StructureGroup {
    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    has_owned_annotations!();

    get_and_mutate_collection_of!(pub members => Vec, ByValueMember);

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.members().all(|m| m.is_complete())
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.annotation_properties().map(|a| a.name()).collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Unions
// ------------------------------------------------------------------------------------------------

impl UnionDef {
    type_definition_impl!(UnionBody);

    referenced_optional_body_annotations!();

    referenced_optional_body_types!();

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.body.is_some()
    }
}

// ------------------------------------------------------------------------------------------------

impl UnionBody {
    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    has_owned_annotations!();

    get_and_mutate_collection_of!(pub variants => Vec, TypeVariant);

    // --------------------------------------------------------------------------------------------

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.variants().map(|v| v.name()).collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl TypeVariant {
    pub fn new(name: IdentifierReference) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            rename: None,
            body: None,
        }
    }

    pub fn new_with(name: IdentifierReference, body: AnnotationOnlyBody) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            rename: None,
            body: Some(body),
        }
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    get_and_mutate!(pub body => option AnnotationOnlyBody);

    referenced_optional_body_annotations!();

    // --------------------------------------------------------------------------------------------

    get_and_mutate!(pub name => IdentifierReference);

    with!(pub rename => option Identifier);
    get_and_mutate!(pub rename => option Identifier);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Properties
// ------------------------------------------------------------------------------------------------

impl PropertyDef {
    type_definition_impl!(PropertyBody);

    // --------------------------------------------------------------------------------------------

    fn is_complete(&self) -> bool {
        self.body.is_some()
    }

    referenced_optional_body_annotations!();

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.body()
            .map(|b| b.referenced_types())
            .unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------

impl PropertyBody {
    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    has_owned_annotations!();

    get_and_mutate_collection_of!(pub roles => Vec, PropertyRole);

    // --------------------------------------------------------------------------------------------

    referenced_own_annotations!();

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.roles()
            .flat_map(|role| role.referenced_types())
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl PropertyRole {
    pub fn new(name: Identifier, target_type: TypeReference) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            target_type,
            source_cardinality: Default::default(),
            target_cardinality: Default::default(),
            body: None,
        }
    }

    pub fn new_unknown(name: Identifier) -> Self {
        Self::new(name, TypeReference::Unknown)
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    get_and_mutate!(pub body => option AnnotationOnlyBody);

    get_and_mutate!(pub name => Identifier);

    get_and_mutate!(pub target_type => TypeReference);

    get_and_mutate!(pub source_cardinality => option Option<Cardinality>);

    get_and_mutate!(pub target_cardinality(target_cardinality) => option Cardinality);

    // --------------------------------------------------------------------------------------------

    referenced_optional_body_annotations!();

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
