pub use radroots_types as upstream;

use radroots_sdk_binding_model as ts;

pub fn types_module() -> ts::TsModule {
    ts::module(vec![
        ts::type_alias_params(
            "IError",
            &["T"],
            ts::object(vec![ts::field("err", ts::reference("T"))]),
        ),
        ts::type_alias_params(
            "IResult",
            &["T"],
            ts::object(vec![ts::field("result", ts::reference("T"))]),
        ),
        ts::type_alias_params(
            "IResultList",
            &["T"],
            ts::object(vec![ts::field("results", ts::array(ts::reference("T")))]),
        ),
        ts::type_alias(
            "IResultPass",
            ts::object(vec![ts::field("pass", ts::boolean())]),
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::types_module;

    #[test]
    fn preserves_result_wrapper_exports() {
        let rendered = types_module().render();
        assert!(rendered.contains("export type IError"));
        assert!(rendered.contains("export type IResultList"));
        assert!(rendered.contains("export type IResultPass"));
    }
}
