#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TsModule {
    declarations: Vec<TsDeclaration>,
}

impl TsModule {
    pub fn new(declarations: Vec<TsDeclaration>) -> Self {
        Self { declarations }
    }

    pub fn render(&self) -> String {
        self.declarations
            .iter()
            .map(TsDeclaration::render)
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TsDeclaration {
    TypeAlias(TsTypeAlias),
    Const(TsConst),
    ImportType(TsImportType),
}

impl TsDeclaration {
    fn render(&self) -> String {
        match self {
            Self::TypeAlias(alias) => alias.render(),
            Self::Const(constant) => constant.render(),
            Self::ImportType(import) => import.render(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TsTypeAlias {
    name: String,
    params: Vec<String>,
    value: TsType,
}

impl TsTypeAlias {
    fn render(&self) -> String {
        let params = if self.params.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.params.join(", "))
        };
        format!(
            "export type {}{} = {};",
            self.name,
            params,
            self.value.render()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TsConst {
    name: String,
    annotation: Option<TsType>,
    value: TsValue,
}

impl TsConst {
    fn render(&self) -> String {
        let annotation = self
            .annotation
            .as_ref()
            .map(|value| format!(": {}", value.render()))
            .unwrap_or_default();
        format!(
            "export const {}{} = {};",
            self.name,
            annotation,
            self.value.render()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TsImportType {
    names: Vec<String>,
    from: String,
}

impl TsImportType {
    fn render(&self) -> String {
        if self.names.len() == 1 {
            return format!(
                "import type {{ {} }} from {};",
                self.names[0],
                quote_string(&self.from)
            );
        }
        let names = self
            .names
            .iter()
            .map(|name| format!("    {name},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "import type {{\n{}\n}} from {};",
            names,
            quote_string(&self.from)
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TsType {
    Primitive(TsPrimitive),
    Reference { name: String, args: Vec<TsType> },
    Array(Box<TsType>),
    Tuple { readonly: bool, items: Vec<TsType> },
    Object(Vec<TsField>),
    Union(Vec<TsType>),
    StringLiteral(String),
    NumberLiteral(i64),
}

impl TsType {
    pub fn render(&self) -> String {
        match self {
            Self::Primitive(value) => value.render().to_owned(),
            Self::Reference { name, args } => {
                if args.is_empty() {
                    name.clone()
                } else {
                    format!(
                        "{}<{}>",
                        name,
                        args.iter()
                            .map(TsType::render)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            Self::Array(item) => format!("Array<{}>", item.render()),
            Self::Tuple { readonly, items } => {
                let prefix = if *readonly { "readonly " } else { "" };
                format!(
                    "{}[{}]",
                    prefix,
                    items
                        .iter()
                        .map(TsType::render)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::Object(fields) => {
                if fields.is_empty() {
                    return "{}".to_owned();
                }
                format!(
                    "{{ {}, }}",
                    fields
                        .iter()
                        .map(TsField::render)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::Union(items) => {
                if items.is_empty() {
                    return "never".to_owned();
                }
                items
                    .iter()
                    .map(TsType::render)
                    .collect::<Vec<_>>()
                    .join(" | ")
            }
            Self::StringLiteral(value) => quote_string(value),
            Self::NumberLiteral(value) => value.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TsPrimitive {
    String,
    Number,
    Boolean,
    BigInt,
    Null,
}

impl TsPrimitive {
    fn render(&self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::BigInt => "bigint",
            Self::Null => "null",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TsField {
    name: String,
    optional: bool,
    value: TsType,
}

impl TsField {
    fn render(&self) -> String {
        let optional = if self.optional { "?" } else { "" };
        format!(
            "{}{}: {}",
            render_property_name(&self.name),
            optional,
            self.value.render()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TsValue {
    Number(i64),
    String(String),
    Boolean(bool),
    Array(Vec<TsValue>),
}

impl TsValue {
    fn render(&self) -> String {
        match self {
            Self::Number(value) => value.to_string(),
            Self::String(value) => quote_string(value),
            Self::Boolean(value) => value.to_string(),
            Self::Array(values) => format!(
                "[{}]",
                values
                    .iter()
                    .map(TsValue::render)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

pub fn module(declarations: Vec<TsDeclaration>) -> TsModule {
    TsModule::new(declarations)
}

pub fn import_type(names: &[&str], from: &str) -> TsDeclaration {
    TsDeclaration::ImportType(TsImportType {
        names: names.iter().map(|name| (*name).to_owned()).collect(),
        from: from.to_owned(),
    })
}

pub fn type_alias(name: &str, value: TsType) -> TsDeclaration {
    type_alias_params(name, &[], value)
}

pub fn type_alias_params(name: &str, params: &[&str], value: TsType) -> TsDeclaration {
    TsDeclaration::TypeAlias(TsTypeAlias {
        name: name.to_owned(),
        params: params.iter().map(|param| (*param).to_owned()).collect(),
        value,
    })
}

pub fn const_number(name: &str, value: i64) -> TsDeclaration {
    const_decl(name, None, TsValue::Number(value))
}

pub fn const_string_array(name: &str, annotation: TsType, values: &[&str]) -> TsDeclaration {
    const_decl(
        name,
        Some(annotation),
        TsValue::Array(
            values
                .iter()
                .map(|value| TsValue::String((*value).to_owned()))
                .collect(),
        ),
    )
}

pub fn const_decl(name: &str, annotation: Option<TsType>, value: TsValue) -> TsDeclaration {
    TsDeclaration::Const(TsConst {
        name: name.to_owned(),
        annotation,
        value,
    })
}

pub fn string() -> TsType {
    TsType::Primitive(TsPrimitive::String)
}

pub fn number() -> TsType {
    TsType::Primitive(TsPrimitive::Number)
}

pub fn boolean() -> TsType {
    TsType::Primitive(TsPrimitive::Boolean)
}

pub fn bigint() -> TsType {
    TsType::Primitive(TsPrimitive::BigInt)
}

pub fn null() -> TsType {
    TsType::Primitive(TsPrimitive::Null)
}

pub fn reference(name: &str) -> TsType {
    TsType::Reference {
        name: name.to_owned(),
        args: Vec::new(),
    }
}

pub fn generic(name: &str, args: Vec<TsType>) -> TsType {
    TsType::Reference {
        name: name.to_owned(),
        args,
    }
}

pub fn array(item: TsType) -> TsType {
    TsType::Array(Box::new(item))
}

pub fn tuple(items: Vec<TsType>) -> TsType {
    TsType::Tuple {
        readonly: false,
        items,
    }
}

pub fn readonly_tuple(items: Vec<TsType>) -> TsType {
    TsType::Tuple {
        readonly: true,
        items,
    }
}

pub fn object(fields: Vec<TsField>) -> TsType {
    TsType::Object(fields)
}

pub fn union(items: Vec<TsType>) -> TsType {
    let mut flattened = Vec::new();
    for item in items {
        match item {
            TsType::Union(items) => flattened.extend(items),
            item => flattened.push(item),
        }
    }
    TsType::Union(flattened)
}

pub fn nullable(item: TsType) -> TsType {
    union(vec![item, null()])
}

pub fn string_literal(value: &str) -> TsType {
    TsType::StringLiteral(value.to_owned())
}

pub fn number_literal(value: i64) -> TsType {
    TsType::NumberLiteral(value)
}

pub fn field(name: &str, value: TsType) -> TsField {
    TsField {
        name: name.to_owned(),
        optional: false,
        value,
    }
}

pub fn optional_field(name: &str, value: TsType) -> TsField {
    TsField {
        name: name.to_owned(),
        optional: true,
        value,
    }
}

fn render_property_name(value: &str) -> String {
    if is_identifier(value) {
        value.to_owned()
    } else {
        quote_string(value)
    }
}

fn is_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) if first == '_' || first == '$' || first.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|ch| ch == '_' || ch == '$' || ch.is_ascii_alphanumeric())
}

fn quote_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            ch => escaped.push(ch),
        }
    }
    escaped.push('"');
    escaped
}

#[cfg(test)]
mod tests {
    use super::{
        array, const_number, field, import_type, module, nullable, number, object, reference,
        string, string_literal, type_alias, type_alias_params, union,
    };

    #[test]
    fn renders_type_aliases() {
        let module = module(vec![
            type_alias("Name", string()),
            type_alias_params(
                "Result",
                &["T"],
                object(vec![field("result", reference("T"))]),
            ),
        ]);
        assert_eq!(
            module.render(),
            "export type Name = string;\n\nexport type Result<T> = { result: T, };"
        );
    }

    #[test]
    fn renders_imports_constants_and_unions() {
        let module = module(vec![
            import_type(&["A", "B"], "@radroots/example"),
            type_alias(
                "Status",
                union(vec![string_literal("ready"), nullable(array(number()))]),
            ),
            const_number("KIND_READY", 1),
        ]);
        assert!(module.render().contains("import type {\n    A,\n    B,\n}"));
        assert!(
            module
                .render()
                .contains("export type Status = \"ready\" | Array<number> | null;")
        );
        assert!(module.render().contains("export const KIND_READY = 1;"));
    }
}
