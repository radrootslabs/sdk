pub use radroots_types as upstream;

use dto_bindgen_core::{
    DescribeCtx, Dto, FieldDef, GenericParam, IdentName, Primitive, RootDescriptor, RustTypeId,
    SourceSpan, StructDef, TargetFieldNames, TypeDef, TypeRef, WireFieldNames,
};

struct IErrorDto;
struct IResultDto;
struct IResultListDto;
struct IResultPassDto;

pub fn dto_roots() -> Vec<RootDescriptor> {
    vec![
        RootDescriptor::new::<IErrorDto>(),
        RootDescriptor::new::<IResultDto>(),
        RootDescriptor::new::<IResultListDto>(),
        RootDescriptor::new::<IResultPassDto>(),
    ]
}

impl Dto for IErrorDto {
    fn describe(ctx: &mut DescribeCtx) -> TypeRef {
        ctx.register_type(
            rust_id("IError"),
            generic_struct("IError", "err", TypeRef::GenericParam("T".to_owned())),
        )
    }
}

impl Dto for IResultDto {
    fn describe(ctx: &mut DescribeCtx) -> TypeRef {
        ctx.register_type(
            rust_id("IResult"),
            generic_struct("IResult", "result", TypeRef::GenericParam("T".to_owned())),
        )
    }
}

impl Dto for IResultListDto {
    fn describe(ctx: &mut DescribeCtx) -> TypeRef {
        ctx.register_type(
            rust_id("IResultList"),
            generic_struct(
                "IResultList",
                "results",
                TypeRef::vec(TypeRef::GenericParam("T".to_owned())),
            ),
        )
    }
}

impl Dto for IResultPassDto {
    fn describe(ctx: &mut DescribeCtx) -> TypeRef {
        ctx.register_type(
            rust_id("IResultPass"),
            TypeDef::Struct(
                StructDef::new("IResultPass", "IResultPass", source_span())
                    .with_field(field("pass", TypeRef::Primitive(Primitive::Bool))),
            ),
        )
    }
}

fn generic_struct(export_name: &str, field_name: &str, field_type: TypeRef) -> TypeDef {
    let mut def = StructDef::new(export_name, export_name, source_span())
        .with_field(field(field_name, field_type));
    def.generics.push(GenericParam::new("T"));
    TypeDef::Struct(def)
}

fn field(name: &str, ty: TypeRef) -> FieldDef {
    FieldDef::new(
        IdentName::new(name),
        WireFieldNames::same(name),
        TargetFieldNames::new(name, name),
        ty,
        source_span(),
    )
}

fn rust_id(name: &'static str) -> RustTypeId {
    RustTypeId::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_NAME"), name)
}

fn source_span() -> SourceSpan {
    SourceSpan::new(file!(), line!(), column!())
}

#[cfg(test)]
mod tests {
    use super::dto_roots;
    use dto_bindgen_core::{TypeDef, TypeRef, build_registry};

    #[test]
    fn preserves_result_wrapper_roots() {
        let registry = build_registry(dto_roots());
        let actual = registry
            .types_by_id
            .values()
            .map(|type_def| match type_def {
                TypeDef::Struct(def) => def.export_name.as_str(),
                TypeDef::Enum(def) => def.export_name.as_str(),
            })
            .collect::<Vec<_>>();

        assert_eq!(actual, ["IError", "IResult", "IResultList", "IResultPass"]);
    }

    #[test]
    fn preserves_generic_result_wrapper_fields() {
        let registry = build_registry(dto_roots());
        let result_list = registry
            .types_by_id
            .values()
            .find_map(|type_def| match type_def {
                TypeDef::Struct(def) if def.export_name == "IResultList" => Some(def),
                _ => None,
            })
            .expect("IResultList descriptor");

        assert_eq!(result_list.generics[0].name, "T");
        assert_eq!(result_list.fields[0].target.typescript, "results");
        assert_eq!(
            result_list.fields[0].ty,
            TypeRef::vec(TypeRef::GenericParam("T".to_owned()))
        );
    }
}
