pub use radroots_types as upstream;

pub const TYPES_TS: &str = include_str!("typescript/types.ts");

#[cfg(test)]
mod tests {
    use super::TYPES_TS;

    #[test]
    fn preserves_result_wrapper_exports() {
        assert!(TYPES_TS.contains("export type IError"));
        assert!(TYPES_TS.contains("export type IResultList"));
        assert!(TYPES_TS.contains("export type IResultPass"));
    }
}
