use radroots_sdk_binding_model as ts;

pub fn types_module() -> ts::TsModule {
    ts::module(vec![
        ts::type_alias(
            "Farm",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("d_tag", ts::string()),
                ts::field("pubkey", ts::string()),
                ts::field("name", ts::string()),
                ts::field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::field("website", ts::union(vec![ts::string(), ts::null()])),
                ts::field("picture", ts::union(vec![ts::string(), ts::null()])),
                ts::field("banner", ts::union(vec![ts::string(), ts::null()])),
                ts::field(
                    "location_primary",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
                ts::field("location_city", ts::union(vec![ts::string(), ts::null()])),
                ts::field("location_region", ts::union(vec![ts::string(), ts::null()])),
                ts::field(
                    "location_country",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
            ]),
        ),
        ts::type_alias(
            "FarmGcsLocation",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("farm_id", ts::string()),
                ts::field("gcs_location_id", ts::string()),
                ts::field("role", ts::string()),
            ]),
        ),
        ts::type_alias(
            "FarmGcsLocationQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("farm_id", ts::string())]),
                ts::object(vec![ts::field("gcs_location_id", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "FarmMember",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("farm_id", ts::string()),
                ts::field("member_pubkey", ts::string()),
                ts::field("role", ts::string()),
            ]),
        ),
        ts::type_alias(
            "FarmMemberClaim",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("member_pubkey", ts::string()),
                ts::field("farm_pubkey", ts::string()),
            ]),
        ),
        ts::type_alias(
            "FarmMemberClaimQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("member_pubkey", ts::string())]),
                ts::object(vec![ts::field("farm_pubkey", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "FarmMemberQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("farm_id", ts::string())]),
                ts::object(vec![ts::field("member_pubkey", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "FarmQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("d_tag", ts::string())]),
                ts::object(vec![ts::field("pubkey", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "FarmTag",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("farm_id", ts::string()),
                ts::field("tag", ts::string()),
            ]),
        ),
        ts::type_alias(
            "FarmTagQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("farm_id", ts::string())]),
                ts::object(vec![ts::field("tag", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "GcsLocation",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("d_tag", ts::string()),
                ts::field("lat", ts::number()),
                ts::field("lng", ts::number()),
                ts::field("geohash", ts::string()),
                ts::field("point", ts::string()),
                ts::field("polygon", ts::string()),
                ts::field("accuracy", ts::union(vec![ts::number(), ts::null()])),
                ts::field("altitude", ts::union(vec![ts::number(), ts::null()])),
                ts::field("tag_0", ts::union(vec![ts::string(), ts::null()])),
                ts::field("label", ts::union(vec![ts::string(), ts::null()])),
                ts::field("area", ts::union(vec![ts::number(), ts::null()])),
                ts::field("elevation", ts::union(vec![ts::number(), ts::null()])),
                ts::field("soil", ts::union(vec![ts::string(), ts::null()])),
                ts::field("climate", ts::union(vec![ts::string(), ts::null()])),
                ts::field("gc_id", ts::union(vec![ts::string(), ts::null()])),
                ts::field("gc_name", ts::union(vec![ts::string(), ts::null()])),
                ts::field("gc_admin1_id", ts::union(vec![ts::string(), ts::null()])),
                ts::field("gc_admin1_name", ts::union(vec![ts::string(), ts::null()])),
                ts::field("gc_country_id", ts::union(vec![ts::string(), ts::null()])),
                ts::field("gc_country_name", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "GcsLocationFarmArgs",
            ts::object(vec![ts::field("id", ts::string())]),
        ),
        ts::type_alias(
            "GcsLocationFindManyRel",
            ts::union(vec![
                ts::object(vec![ts::field(
                    "on_trade_product",
                    ts::reference("GcsLocationTradeProductArgs"),
                )]),
                ts::object(vec![ts::field(
                    "off_trade_product",
                    ts::reference("GcsLocationTradeProductArgs"),
                )]),
                ts::object(vec![ts::field(
                    "on_farm",
                    ts::reference("GcsLocationFarmArgs"),
                )]),
                ts::object(vec![ts::field(
                    "off_farm",
                    ts::reference("GcsLocationFarmArgs"),
                )]),
                ts::object(vec![ts::field(
                    "on_plot",
                    ts::reference("GcsLocationPlotArgs"),
                )]),
                ts::object(vec![ts::field(
                    "off_plot",
                    ts::reference("GcsLocationPlotArgs"),
                )]),
            ]),
        ),
        ts::type_alias(
            "GcsLocationPlotArgs",
            ts::object(vec![ts::field("id", ts::string())]),
        ),
        ts::type_alias(
            "GcsLocationQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("d_tag", ts::string())]),
                ts::object(vec![ts::field("geohash", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "GcsLocationTradeProductArgs",
            ts::object(vec![ts::field("id", ts::string())]),
        ),
        ts::type_alias("IFarmCreate", ts::reference("IFarmFields")),
        ts::type_alias(
            "IFarmCreateResolve",
            ts::generic("IResult", vec![ts::reference("Farm")]),
        ),
        ts::type_alias("IFarmDelete", ts::reference("IFarmFindOne")),
        ts::type_alias(
            "IFarmDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "IFarmFields",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("pubkey", ts::string()),
                ts::field("name", ts::string()),
                ts::optional_field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("website", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("picture", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("banner", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "location_primary",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
                ts::optional_field("location_city", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("location_region", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "location_country",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
            ]),
        ),
        ts::type_alias(
            "IFarmFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("d_tag", ts::string()),
                ts::optional_field("pubkey", ts::string()),
                ts::optional_field("name", ts::string()),
                ts::optional_field("about", ts::string()),
                ts::optional_field("website", ts::string()),
                ts::optional_field("picture", ts::string()),
                ts::optional_field("banner", ts::string()),
                ts::optional_field("location_primary", ts::string()),
                ts::optional_field("location_city", ts::string()),
                ts::optional_field("location_region", ts::string()),
                ts::optional_field("location_country", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IFarmFieldsPartial",
            ts::object(vec![
                ts::optional_field("d_tag", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("pubkey", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("website", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("picture", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("banner", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "location_primary",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
                ts::optional_field("location_city", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("location_region", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "location_country",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
            ]),
        ),
        ts::type_alias(
            "IFarmFindMany",
            ts::object(vec![ts::field(
                "filter",
                ts::union(vec![ts::reference("IFarmFieldsFilter"), ts::null()]),
            )]),
        ),
        ts::type_alias(
            "IFarmFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("Farm")]),
        ),
        ts::type_alias("IFarmFindOne", ts::reference("IFarmFindOneArgs")),
        ts::type_alias(
            "IFarmFindOneArgs",
            ts::object(vec![ts::field("on", ts::reference("FarmQueryBindValues"))]),
        ),
        ts::type_alias(
            "IFarmFindOneResolve",
            ts::generic("IResult", vec![ts::reference("Farm")]),
        ),
        ts::type_alias(
            "IFarmGcsLocationCreate",
            ts::reference("IFarmGcsLocationFields"),
        ),
        ts::type_alias(
            "IFarmGcsLocationCreateResolve",
            ts::generic("IResult", vec![ts::reference("FarmGcsLocation")]),
        ),
        ts::type_alias(
            "IFarmGcsLocationDelete",
            ts::reference("IFarmGcsLocationFindOne"),
        ),
        ts::type_alias(
            "IFarmGcsLocationDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "IFarmGcsLocationFields",
            ts::object(vec![
                ts::field("farm_id", ts::string()),
                ts::field("gcs_location_id", ts::string()),
                ts::field("role", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IFarmGcsLocationFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("farm_id", ts::string()),
                ts::optional_field("gcs_location_id", ts::string()),
                ts::optional_field("role", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IFarmGcsLocationFieldsPartial",
            ts::object(vec![
                ts::optional_field("farm_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gcs_location_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("role", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "IFarmGcsLocationFindMany",
            ts::object(vec![ts::field(
                "filter",
                ts::union(vec![
                    ts::reference("IFarmGcsLocationFieldsFilter"),
                    ts::null(),
                ]),
            )]),
        ),
        ts::type_alias(
            "IFarmGcsLocationFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("FarmGcsLocation")]),
        ),
        ts::type_alias(
            "IFarmGcsLocationFindOne",
            ts::reference("IFarmGcsLocationFindOneArgs"),
        ),
        ts::type_alias(
            "IFarmGcsLocationFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("FarmGcsLocationQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "IFarmGcsLocationFindOneResolve",
            ts::generic("IResult", vec![ts::reference("FarmGcsLocation")]),
        ),
        ts::type_alias(
            "IFarmGcsLocationUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("FarmGcsLocationQueryBindValues")),
                ts::field("fields", ts::reference("IFarmGcsLocationFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "IFarmGcsLocationUpdateResolve",
            ts::generic("IResult", vec![ts::reference("FarmGcsLocation")]),
        ),
        ts::type_alias(
            "IFarmMemberClaimCreate",
            ts::reference("IFarmMemberClaimFields"),
        ),
        ts::type_alias(
            "IFarmMemberClaimCreateResolve",
            ts::generic("IResult", vec![ts::reference("FarmMemberClaim")]),
        ),
        ts::type_alias(
            "IFarmMemberClaimDelete",
            ts::reference("IFarmMemberClaimFindOne"),
        ),
        ts::type_alias(
            "IFarmMemberClaimDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "IFarmMemberClaimFields",
            ts::object(vec![
                ts::field("member_pubkey", ts::string()),
                ts::field("farm_pubkey", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IFarmMemberClaimFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("member_pubkey", ts::string()),
                ts::optional_field("farm_pubkey", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IFarmMemberClaimFieldsPartial",
            ts::object(vec![
                ts::optional_field("member_pubkey", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("farm_pubkey", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "IFarmMemberClaimFindMany",
            ts::object(vec![ts::field(
                "filter",
                ts::union(vec![
                    ts::reference("IFarmMemberClaimFieldsFilter"),
                    ts::null(),
                ]),
            )]),
        ),
        ts::type_alias(
            "IFarmMemberClaimFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("FarmMemberClaim")]),
        ),
        ts::type_alias(
            "IFarmMemberClaimFindOne",
            ts::reference("IFarmMemberClaimFindOneArgs"),
        ),
        ts::type_alias(
            "IFarmMemberClaimFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("FarmMemberClaimQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "IFarmMemberClaimFindOneResolve",
            ts::generic("IResult", vec![ts::reference("FarmMemberClaim")]),
        ),
        ts::type_alias(
            "IFarmMemberClaimUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("FarmMemberClaimQueryBindValues")),
                ts::field("fields", ts::reference("IFarmMemberClaimFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "IFarmMemberClaimUpdateResolve",
            ts::generic("IResult", vec![ts::reference("FarmMemberClaim")]),
        ),
        ts::type_alias("IFarmMemberCreate", ts::reference("IFarmMemberFields")),
        ts::type_alias(
            "IFarmMemberCreateResolve",
            ts::generic("IResult", vec![ts::reference("FarmMember")]),
        ),
        ts::type_alias("IFarmMemberDelete", ts::reference("IFarmMemberFindOne")),
        ts::type_alias(
            "IFarmMemberDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "IFarmMemberFields",
            ts::object(vec![
                ts::field("farm_id", ts::string()),
                ts::field("member_pubkey", ts::string()),
                ts::field("role", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IFarmMemberFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("farm_id", ts::string()),
                ts::optional_field("member_pubkey", ts::string()),
                ts::optional_field("role", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IFarmMemberFieldsPartial",
            ts::object(vec![
                ts::optional_field("farm_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("member_pubkey", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("role", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "IFarmMemberFindMany",
            ts::object(vec![ts::field(
                "filter",
                ts::union(vec![ts::reference("IFarmMemberFieldsFilter"), ts::null()]),
            )]),
        ),
        ts::type_alias(
            "IFarmMemberFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("FarmMember")]),
        ),
        ts::type_alias(
            "IFarmMemberFindOne",
            ts::reference("IFarmMemberFindOneArgs"),
        ),
        ts::type_alias(
            "IFarmMemberFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("FarmMemberQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "IFarmMemberFindOneResolve",
            ts::generic("IResult", vec![ts::reference("FarmMember")]),
        ),
        ts::type_alias(
            "IFarmMemberUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("FarmMemberQueryBindValues")),
                ts::field("fields", ts::reference("IFarmMemberFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "IFarmMemberUpdateResolve",
            ts::generic("IResult", vec![ts::reference("FarmMember")]),
        ),
        ts::type_alias("IFarmTagCreate", ts::reference("IFarmTagFields")),
        ts::type_alias(
            "IFarmTagCreateResolve",
            ts::generic("IResult", vec![ts::reference("FarmTag")]),
        ),
        ts::type_alias("IFarmTagDelete", ts::reference("IFarmTagFindOne")),
        ts::type_alias(
            "IFarmTagDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "IFarmTagFields",
            ts::object(vec![
                ts::field("farm_id", ts::string()),
                ts::field("tag", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IFarmTagFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("farm_id", ts::string()),
                ts::optional_field("tag", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IFarmTagFieldsPartial",
            ts::object(vec![
                ts::optional_field("farm_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("tag", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "IFarmTagFindMany",
            ts::object(vec![ts::field(
                "filter",
                ts::union(vec![ts::reference("IFarmTagFieldsFilter"), ts::null()]),
            )]),
        ),
        ts::type_alias(
            "IFarmTagFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("FarmTag")]),
        ),
        ts::type_alias("IFarmTagFindOne", ts::reference("IFarmTagFindOneArgs")),
        ts::type_alias(
            "IFarmTagFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("FarmTagQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "IFarmTagFindOneResolve",
            ts::generic("IResult", vec![ts::reference("FarmTag")]),
        ),
        ts::type_alias(
            "IFarmTagUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("FarmTagQueryBindValues")),
                ts::field("fields", ts::reference("IFarmTagFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "IFarmTagUpdateResolve",
            ts::generic("IResult", vec![ts::reference("FarmTag")]),
        ),
        ts::type_alias(
            "IFarmUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("FarmQueryBindValues")),
                ts::field("fields", ts::reference("IFarmFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "IFarmUpdateResolve",
            ts::generic("IResult", vec![ts::reference("Farm")]),
        ),
        ts::type_alias("IGcsLocationCreate", ts::reference("IGcsLocationFields")),
        ts::type_alias(
            "IGcsLocationCreateResolve",
            ts::generic("IResult", vec![ts::reference("GcsLocation")]),
        ),
        ts::type_alias("IGcsLocationDelete", ts::reference("IGcsLocationFindOne")),
        ts::type_alias(
            "IGcsLocationDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "IGcsLocationFields",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("lat", ts::number()),
                ts::field("lng", ts::number()),
                ts::field("geohash", ts::string()),
                ts::field("point", ts::string()),
                ts::field("polygon", ts::string()),
                ts::optional_field("accuracy", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("altitude", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("tag_0", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("label", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("area", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("elevation", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("soil", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("climate", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_admin1_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_admin1_name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_country_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_country_name", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "IGcsLocationFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("d_tag", ts::string()),
                ts::optional_field("lat", ts::number()),
                ts::optional_field("lng", ts::number()),
                ts::optional_field("geohash", ts::string()),
                ts::optional_field("point", ts::string()),
                ts::optional_field("polygon", ts::string()),
                ts::optional_field("accuracy", ts::number()),
                ts::optional_field("altitude", ts::number()),
                ts::optional_field("tag_0", ts::string()),
                ts::optional_field("label", ts::string()),
                ts::optional_field("area", ts::number()),
                ts::optional_field("elevation", ts::number()),
                ts::optional_field("soil", ts::string()),
                ts::optional_field("climate", ts::string()),
                ts::optional_field("gc_id", ts::string()),
                ts::optional_field("gc_name", ts::string()),
                ts::optional_field("gc_admin1_id", ts::string()),
                ts::optional_field("gc_admin1_name", ts::string()),
                ts::optional_field("gc_country_id", ts::string()),
                ts::optional_field("gc_country_name", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IGcsLocationFieldsPartial",
            ts::object(vec![
                ts::optional_field("d_tag", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("lat", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("lng", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("geohash", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("point", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("polygon", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("accuracy", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("altitude", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("tag_0", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("label", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("area", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("elevation", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("soil", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("climate", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_admin1_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_admin1_name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_country_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gc_country_name", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "IGcsLocationFindMany",
            ts::union(vec![
                ts::object(vec![ts::field(
                    "filter",
                    ts::union(vec![ts::reference("IGcsLocationFieldsFilter"), ts::null()]),
                )]),
                ts::object(vec![ts::field(
                    "rel",
                    ts::reference("GcsLocationFindManyRel"),
                )]),
            ]),
        ),
        ts::type_alias(
            "IGcsLocationFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("GcsLocation")]),
        ),
        ts::type_alias(
            "IGcsLocationFindOne",
            ts::union(vec![
                ts::reference("IGcsLocationFindOneArgs"),
                ts::reference("IGcsLocationFindOneRelArgs"),
            ]),
        ),
        ts::type_alias(
            "IGcsLocationFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("GcsLocationQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "IGcsLocationFindOneRelArgs",
            ts::object(vec![ts::field(
                "rel",
                ts::reference("GcsLocationFindManyRel"),
            )]),
        ),
        ts::type_alias(
            "IGcsLocationFindOneResolve",
            ts::generic("IResult", vec![ts::reference("GcsLocation")]),
        ),
        ts::type_alias(
            "IGcsLocationUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("GcsLocationQueryBindValues")),
                ts::field("fields", ts::reference("IGcsLocationFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "IGcsLocationUpdateResolve",
            ts::generic("IResult", vec![ts::reference("GcsLocation")]),
        ),
        ts::type_alias("ILogErrorCreate", ts::reference("ILogErrorFields")),
        ts::type_alias(
            "ILogErrorCreateResolve",
            ts::generic("IResult", vec![ts::reference("LogError")]),
        ),
        ts::type_alias("ILogErrorDelete", ts::reference("ILogErrorFindOne")),
        ts::type_alias(
            "ILogErrorDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "ILogErrorFields",
            ts::object(vec![
                ts::field("error", ts::string()),
                ts::field("message", ts::string()),
                ts::optional_field("stack_trace", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("cause", ts::union(vec![ts::string(), ts::null()])),
                ts::field("app_system", ts::string()),
                ts::field("app_version", ts::string()),
                ts::field("nostr_pubkey", ts::string()),
                ts::optional_field("data", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "ILogErrorFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("error", ts::string()),
                ts::optional_field("message", ts::string()),
                ts::optional_field("stack_trace", ts::string()),
                ts::optional_field("cause", ts::string()),
                ts::optional_field("app_system", ts::string()),
                ts::optional_field("app_version", ts::string()),
                ts::optional_field("nostr_pubkey", ts::string()),
                ts::optional_field("data", ts::string()),
            ]),
        ),
        ts::type_alias(
            "ILogErrorFieldsPartial",
            ts::object(vec![
                ts::optional_field("error", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("message", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("stack_trace", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("cause", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("app_system", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("app_version", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("nostr_pubkey", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("data", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "ILogErrorFindMany",
            ts::object(vec![ts::field(
                "filter",
                ts::union(vec![ts::reference("ILogErrorFieldsFilter"), ts::null()]),
            )]),
        ),
        ts::type_alias(
            "ILogErrorFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("LogError")]),
        ),
        ts::type_alias("ILogErrorFindOne", ts::reference("ILogErrorFindOneArgs")),
        ts::type_alias(
            "ILogErrorFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("LogErrorQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "ILogErrorFindOneResolve",
            ts::generic("IResult", vec![ts::reference("LogError")]),
        ),
        ts::type_alias(
            "ILogErrorUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("LogErrorQueryBindValues")),
                ts::field("fields", ts::reference("ILogErrorFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "ILogErrorUpdateResolve",
            ts::generic("IResult", vec![ts::reference("LogError")]),
        ),
        ts::type_alias("IMediaImageCreate", ts::reference("IMediaImageFields")),
        ts::type_alias(
            "IMediaImageCreateResolve",
            ts::generic("IResult", vec![ts::reference("MediaImage")]),
        ),
        ts::type_alias("IMediaImageDelete", ts::reference("IMediaImageFindOne")),
        ts::type_alias(
            "IMediaImageDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "IMediaImageFields",
            ts::object(vec![
                ts::field("file_path", ts::string()),
                ts::field("mime_type", ts::string()),
                ts::field("res_base", ts::string()),
                ts::field("res_path", ts::string()),
                ts::optional_field("label", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("description", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "IMediaImageFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("file_path", ts::string()),
                ts::optional_field("mime_type", ts::string()),
                ts::optional_field("res_base", ts::string()),
                ts::optional_field("res_path", ts::string()),
                ts::optional_field("label", ts::string()),
                ts::optional_field("description", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IMediaImageFieldsPartial",
            ts::object(vec![
                ts::optional_field("file_path", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("mime_type", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("res_base", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("res_path", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("label", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("description", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "IMediaImageFindMany",
            ts::union(vec![
                ts::object(vec![ts::field(
                    "filter",
                    ts::union(vec![ts::reference("IMediaImageFieldsFilter"), ts::null()]),
                )]),
                ts::object(vec![ts::field(
                    "rel",
                    ts::reference("MediaImageFindManyRel"),
                )]),
            ]),
        ),
        ts::type_alias(
            "IMediaImageFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("MediaImage")]),
        ),
        ts::type_alias(
            "IMediaImageFindOne",
            ts::union(vec![
                ts::reference("IMediaImageFindOneArgs"),
                ts::reference("IMediaImageFindOneRelArgs"),
            ]),
        ),
        ts::type_alias(
            "IMediaImageFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("MediaImageQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "IMediaImageFindOneRelArgs",
            ts::object(vec![ts::field(
                "rel",
                ts::reference("MediaImageFindManyRel"),
            )]),
        ),
        ts::type_alias(
            "IMediaImageFindOneResolve",
            ts::generic("IResult", vec![ts::reference("MediaImage")]),
        ),
        ts::type_alias(
            "IMediaImageUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("MediaImageQueryBindValues")),
                ts::field("fields", ts::reference("IMediaImageFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "IMediaImageUpdateResolve",
            ts::generic("IResult", vec![ts::reference("MediaImage")]),
        ),
        ts::type_alias(
            "INostrEventStateCreate",
            ts::reference("INostrEventStateFields"),
        ),
        ts::type_alias(
            "INostrEventStateCreateResolve",
            ts::generic("IResult", vec![ts::reference("NostrEventState")]),
        ),
        ts::type_alias(
            "INostrEventStateDelete",
            ts::reference("INostrEventStateFindOne"),
        ),
        ts::type_alias(
            "INostrEventStateDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "INostrEventStateFields",
            ts::object(vec![
                ts::field("key", ts::string()),
                ts::field("kind", ts::number()),
                ts::field("pubkey", ts::string()),
                ts::field("d_tag", ts::string()),
                ts::field("last_event_id", ts::string()),
                ts::field("last_created_at", ts::number()),
                ts::field("content_hash", ts::string()),
            ]),
        ),
        ts::type_alias(
            "INostrEventStateFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("key", ts::string()),
                ts::optional_field("kind", ts::number()),
                ts::optional_field("pubkey", ts::string()),
                ts::optional_field("d_tag", ts::string()),
                ts::optional_field("last_event_id", ts::string()),
                ts::optional_field("last_created_at", ts::number()),
                ts::optional_field("content_hash", ts::string()),
            ]),
        ),
        ts::type_alias(
            "INostrEventStateFieldsPartial",
            ts::object(vec![
                ts::optional_field("key", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("kind", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("pubkey", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("d_tag", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("last_event_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("last_created_at", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("content_hash", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "INostrEventStateFindMany",
            ts::object(vec![ts::field(
                "filter",
                ts::union(vec![
                    ts::reference("INostrEventStateFieldsFilter"),
                    ts::null(),
                ]),
            )]),
        ),
        ts::type_alias(
            "INostrEventStateFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("NostrEventState")]),
        ),
        ts::type_alias(
            "INostrEventStateFindOne",
            ts::reference("INostrEventStateFindOneArgs"),
        ),
        ts::type_alias(
            "INostrEventStateFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("NostrEventStateQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "INostrEventStateFindOneResolve",
            ts::generic("IResult", vec![ts::reference("NostrEventState")]),
        ),
        ts::type_alias(
            "INostrEventStateUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("NostrEventStateQueryBindValues")),
                ts::field("fields", ts::reference("INostrEventStateFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "INostrEventStateUpdateResolve",
            ts::generic("IResult", vec![ts::reference("NostrEventState")]),
        ),
        ts::type_alias("INostrProfileCreate", ts::reference("INostrProfileFields")),
        ts::type_alias(
            "INostrProfileCreateResolve",
            ts::generic("IResult", vec![ts::reference("NostrProfile")]),
        ),
        ts::type_alias("INostrProfileDelete", ts::reference("INostrProfileFindOne")),
        ts::type_alias(
            "INostrProfileDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "INostrProfileFields",
            ts::object(vec![
                ts::field("public_key", ts::string()),
                ts::field("profile_type", ts::string()),
                ts::field("name", ts::string()),
                ts::optional_field("display_name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("website", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("picture", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("banner", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("nip05", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("lud06", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("lud16", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "INostrProfileFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("public_key", ts::string()),
                ts::optional_field("profile_type", ts::string()),
                ts::optional_field("name", ts::string()),
                ts::optional_field("display_name", ts::string()),
                ts::optional_field("about", ts::string()),
                ts::optional_field("website", ts::string()),
                ts::optional_field("picture", ts::string()),
                ts::optional_field("banner", ts::string()),
                ts::optional_field("nip05", ts::string()),
                ts::optional_field("lud06", ts::string()),
                ts::optional_field("lud16", ts::string()),
            ]),
        ),
        ts::type_alias(
            "INostrProfileFieldsPartial",
            ts::object(vec![
                ts::optional_field("public_key", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("profile_type", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("display_name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("website", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("picture", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("banner", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("nip05", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("lud06", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("lud16", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "INostrProfileFindMany",
            ts::union(vec![
                ts::object(vec![ts::field(
                    "filter",
                    ts::union(vec![ts::reference("INostrProfileFieldsFilter"), ts::null()]),
                )]),
                ts::object(vec![ts::field(
                    "rel",
                    ts::reference("NostrProfileFindManyRel"),
                )]),
            ]),
        ),
        ts::type_alias(
            "INostrProfileFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("NostrProfile")]),
        ),
        ts::type_alias(
            "INostrProfileFindOne",
            ts::union(vec![
                ts::reference("INostrProfileFindOneArgs"),
                ts::reference("INostrProfileFindOneRelArgs"),
            ]),
        ),
        ts::type_alias(
            "INostrProfileFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("NostrProfileQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "INostrProfileFindOneRelArgs",
            ts::object(vec![ts::field(
                "rel",
                ts::reference("NostrProfileFindManyRel"),
            )]),
        ),
        ts::type_alias(
            "INostrProfileFindOneResolve",
            ts::generic("IResult", vec![ts::reference("NostrProfile")]),
        ),
        ts::type_alias(
            "INostrProfileRelayRelation",
            ts::object(vec![
                ts::field(
                    "nostr_profile",
                    ts::reference("NostrProfileQueryBindValues"),
                ),
                ts::field("nostr_relay", ts::reference("NostrRelayQueryBindValues")),
            ]),
        ),
        ts::type_alias("INostrProfileRelayResolve", ts::reference("IResultPass")),
        ts::type_alias(
            "INostrProfileUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("NostrProfileQueryBindValues")),
                ts::field("fields", ts::reference("INostrProfileFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "INostrProfileUpdateResolve",
            ts::generic("IResult", vec![ts::reference("NostrProfile")]),
        ),
        ts::type_alias("INostrRelayCreate", ts::reference("INostrRelayFields")),
        ts::type_alias(
            "INostrRelayCreateResolve",
            ts::generic("IResult", vec![ts::reference("NostrRelay")]),
        ),
        ts::type_alias("INostrRelayDelete", ts::reference("INostrRelayFindOne")),
        ts::type_alias(
            "INostrRelayDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "INostrRelayFields",
            ts::object(vec![
                ts::field("url", ts::string()),
                ts::optional_field("relay_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("description", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("pubkey", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("contact", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("supported_nips", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("software", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("version", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("data", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "INostrRelayFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("url", ts::string()),
                ts::optional_field("relay_id", ts::string()),
                ts::optional_field("name", ts::string()),
                ts::optional_field("description", ts::string()),
                ts::optional_field("pubkey", ts::string()),
                ts::optional_field("contact", ts::string()),
                ts::optional_field("supported_nips", ts::string()),
                ts::optional_field("software", ts::string()),
                ts::optional_field("version", ts::string()),
                ts::optional_field("data", ts::string()),
            ]),
        ),
        ts::type_alias(
            "INostrRelayFieldsPartial",
            ts::object(vec![
                ts::optional_field("url", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("relay_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("description", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("pubkey", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("contact", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("supported_nips", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("software", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("version", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("data", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "INostrRelayFindMany",
            ts::union(vec![
                ts::object(vec![ts::field(
                    "filter",
                    ts::union(vec![ts::reference("INostrRelayFieldsFilter"), ts::null()]),
                )]),
                ts::object(vec![ts::field(
                    "rel",
                    ts::reference("NostrRelayFindManyRel"),
                )]),
            ]),
        ),
        ts::type_alias(
            "INostrRelayFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("NostrRelay")]),
        ),
        ts::type_alias(
            "INostrRelayFindOne",
            ts::union(vec![
                ts::reference("INostrRelayFindOneArgs"),
                ts::reference("INostrRelayFindOneRelArgs"),
            ]),
        ),
        ts::type_alias(
            "INostrRelayFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("NostrRelayQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "INostrRelayFindOneRelArgs",
            ts::object(vec![ts::field(
                "rel",
                ts::reference("NostrRelayFindManyRel"),
            )]),
        ),
        ts::type_alias(
            "INostrRelayFindOneResolve",
            ts::generic("IResult", vec![ts::reference("NostrRelay")]),
        ),
        ts::type_alias(
            "INostrRelayUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("NostrRelayQueryBindValues")),
                ts::field("fields", ts::reference("INostrRelayFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "INostrRelayUpdateResolve",
            ts::generic("IResult", vec![ts::reference("NostrRelay")]),
        ),
        ts::type_alias("IPlotCreate", ts::reference("IPlotFields")),
        ts::type_alias(
            "IPlotCreateResolve",
            ts::generic("IResult", vec![ts::reference("Plot")]),
        ),
        ts::type_alias("IPlotDelete", ts::reference("IPlotFindOne")),
        ts::type_alias(
            "IPlotDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "IPlotFields",
            ts::object(vec![
                ts::field("d_tag", ts::string()),
                ts::field("farm_id", ts::string()),
                ts::field("name", ts::string()),
                ts::optional_field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "location_primary",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
                ts::optional_field("location_city", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("location_region", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "location_country",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
            ]),
        ),
        ts::type_alias(
            "IPlotFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("d_tag", ts::string()),
                ts::optional_field("farm_id", ts::string()),
                ts::optional_field("name", ts::string()),
                ts::optional_field("about", ts::string()),
                ts::optional_field("location_primary", ts::string()),
                ts::optional_field("location_city", ts::string()),
                ts::optional_field("location_region", ts::string()),
                ts::optional_field("location_country", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IPlotFieldsPartial",
            ts::object(vec![
                ts::optional_field("d_tag", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("farm_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("name", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "location_primary",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
                ts::optional_field("location_city", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("location_region", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "location_country",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
            ]),
        ),
        ts::type_alias(
            "IPlotFindMany",
            ts::object(vec![ts::field(
                "filter",
                ts::union(vec![ts::reference("IPlotFieldsFilter"), ts::null()]),
            )]),
        ),
        ts::type_alias(
            "IPlotFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("Plot")]),
        ),
        ts::type_alias("IPlotFindOne", ts::reference("IPlotFindOneArgs")),
        ts::type_alias(
            "IPlotFindOneArgs",
            ts::object(vec![ts::field("on", ts::reference("PlotQueryBindValues"))]),
        ),
        ts::type_alias(
            "IPlotFindOneResolve",
            ts::generic("IResult", vec![ts::reference("Plot")]),
        ),
        ts::type_alias(
            "IPlotGcsLocationCreate",
            ts::reference("IPlotGcsLocationFields"),
        ),
        ts::type_alias(
            "IPlotGcsLocationCreateResolve",
            ts::generic("IResult", vec![ts::reference("PlotGcsLocation")]),
        ),
        ts::type_alias(
            "IPlotGcsLocationDelete",
            ts::reference("IPlotGcsLocationFindOne"),
        ),
        ts::type_alias(
            "IPlotGcsLocationDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "IPlotGcsLocationFields",
            ts::object(vec![
                ts::field("plot_id", ts::string()),
                ts::field("gcs_location_id", ts::string()),
                ts::field("role", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IPlotGcsLocationFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("plot_id", ts::string()),
                ts::optional_field("gcs_location_id", ts::string()),
                ts::optional_field("role", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IPlotGcsLocationFieldsPartial",
            ts::object(vec![
                ts::optional_field("plot_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("gcs_location_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("role", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "IPlotGcsLocationFindMany",
            ts::object(vec![ts::field(
                "filter",
                ts::union(vec![
                    ts::reference("IPlotGcsLocationFieldsFilter"),
                    ts::null(),
                ]),
            )]),
        ),
        ts::type_alias(
            "IPlotGcsLocationFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("PlotGcsLocation")]),
        ),
        ts::type_alias(
            "IPlotGcsLocationFindOne",
            ts::reference("IPlotGcsLocationFindOneArgs"),
        ),
        ts::type_alias(
            "IPlotGcsLocationFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("PlotGcsLocationQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "IPlotGcsLocationFindOneResolve",
            ts::generic("IResult", vec![ts::reference("PlotGcsLocation")]),
        ),
        ts::type_alias(
            "IPlotGcsLocationUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("PlotGcsLocationQueryBindValues")),
                ts::field("fields", ts::reference("IPlotGcsLocationFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "IPlotGcsLocationUpdateResolve",
            ts::generic("IResult", vec![ts::reference("PlotGcsLocation")]),
        ),
        ts::type_alias("IPlotTagCreate", ts::reference("IPlotTagFields")),
        ts::type_alias(
            "IPlotTagCreateResolve",
            ts::generic("IResult", vec![ts::reference("PlotTag")]),
        ),
        ts::type_alias("IPlotTagDelete", ts::reference("IPlotTagFindOne")),
        ts::type_alias(
            "IPlotTagDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "IPlotTagFields",
            ts::object(vec![
                ts::field("plot_id", ts::string()),
                ts::field("tag", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IPlotTagFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("plot_id", ts::string()),
                ts::optional_field("tag", ts::string()),
            ]),
        ),
        ts::type_alias(
            "IPlotTagFieldsPartial",
            ts::object(vec![
                ts::optional_field("plot_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("tag", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "IPlotTagFindMany",
            ts::object(vec![ts::field(
                "filter",
                ts::union(vec![ts::reference("IPlotTagFieldsFilter"), ts::null()]),
            )]),
        ),
        ts::type_alias(
            "IPlotTagFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("PlotTag")]),
        ),
        ts::type_alias("IPlotTagFindOne", ts::reference("IPlotTagFindOneArgs")),
        ts::type_alias(
            "IPlotTagFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("PlotTagQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "IPlotTagFindOneResolve",
            ts::generic("IResult", vec![ts::reference("PlotTag")]),
        ),
        ts::type_alias(
            "IPlotTagUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("PlotTagQueryBindValues")),
                ts::field("fields", ts::reference("IPlotTagFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "IPlotTagUpdateResolve",
            ts::generic("IResult", vec![ts::reference("PlotTag")]),
        ),
        ts::type_alias(
            "IPlotUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("PlotQueryBindValues")),
                ts::field("fields", ts::reference("IPlotFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "IPlotUpdateResolve",
            ts::generic("IResult", vec![ts::reference("Plot")]),
        ),
        ts::type_alias("ITradeProductCreate", ts::reference("ITradeProductFields")),
        ts::type_alias(
            "ITradeProductCreateResolve",
            ts::generic("IResult", vec![ts::reference("TradeProduct")]),
        ),
        ts::type_alias("ITradeProductDelete", ts::reference("ITradeProductFindOne")),
        ts::type_alias(
            "ITradeProductDeleteResolve",
            ts::generic("IResult", vec![ts::string()]),
        ),
        ts::type_alias(
            "ITradeProductFields",
            ts::object(vec![
                ts::field("key", ts::string()),
                ts::field("category", ts::string()),
                ts::field("title", ts::string()),
                ts::field("summary", ts::string()),
                ts::field("process", ts::string()),
                ts::field("lot", ts::string()),
                ts::field("profile", ts::string()),
                ts::field("year", ts::bigint()),
                ts::field("qty_amt", ts::number()),
                ts::field("qty_amt_exact", ts::string()),
                ts::field("qty_unit", ts::string()),
                ts::optional_field("qty_label", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("qty_avail", ts::union(vec![ts::number(), ts::null()])),
                ts::field("price_amt", ts::number()),
                ts::field("price_amt_exact", ts::string()),
                ts::field("price_currency", ts::string()),
                ts::field("price_qty_amt", ts::number()),
                ts::field("price_qty_amt_exact", ts::string()),
                ts::field("price_qty_unit", ts::string()),
                ts::optional_field("listing_addr", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("primary_bin_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "verified_primary_bin_id",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
                ts::optional_field("notes", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "ITradeProductFieldsFilter",
            ts::object(vec![
                ts::optional_field("id", ts::string()),
                ts::optional_field("created_at", ts::string()),
                ts::optional_field("updated_at", ts::string()),
                ts::optional_field("key", ts::string()),
                ts::optional_field("category", ts::string()),
                ts::optional_field("title", ts::string()),
                ts::optional_field("summary", ts::string()),
                ts::optional_field("process", ts::string()),
                ts::optional_field("lot", ts::string()),
                ts::optional_field("profile", ts::string()),
                ts::optional_field("year", ts::bigint()),
                ts::optional_field("qty_amt", ts::number()),
                ts::optional_field("qty_amt_exact", ts::string()),
                ts::optional_field("qty_unit", ts::string()),
                ts::optional_field("qty_label", ts::string()),
                ts::optional_field("qty_avail", ts::bigint()),
                ts::optional_field("price_amt", ts::number()),
                ts::optional_field("price_amt_exact", ts::string()),
                ts::optional_field("price_currency", ts::string()),
                ts::optional_field("price_qty_amt", ts::number()),
                ts::optional_field("price_qty_amt_exact", ts::string()),
                ts::optional_field("price_qty_unit", ts::string()),
                ts::optional_field("listing_addr", ts::string()),
                ts::optional_field("primary_bin_id", ts::string()),
                ts::optional_field("verified_primary_bin_id", ts::string()),
                ts::optional_field("notes", ts::string()),
            ]),
        ),
        ts::type_alias(
            "ITradeProductFieldsPartial",
            ts::object(vec![
                ts::optional_field("key", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("category", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("title", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("summary", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("process", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("lot", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("profile", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("year", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("qty_amt", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("qty_amt_exact", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("qty_unit", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("qty_label", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("qty_avail", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("price_amt", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field("price_amt_exact", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("price_currency", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("price_qty_amt", ts::union(vec![ts::number(), ts::null()])),
                ts::optional_field(
                    "price_qty_amt_exact",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
                ts::optional_field("price_qty_unit", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("listing_addr", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field("primary_bin_id", ts::union(vec![ts::string(), ts::null()])),
                ts::optional_field(
                    "verified_primary_bin_id",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
                ts::optional_field("notes", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "ITradeProductFindMany",
            ts::object(vec![ts::field(
                "filter",
                ts::union(vec![ts::reference("ITradeProductFieldsFilter"), ts::null()]),
            )]),
        ),
        ts::type_alias(
            "ITradeProductFindManyResolve",
            ts::generic("IResultList", vec![ts::reference("TradeProduct")]),
        ),
        ts::type_alias(
            "ITradeProductFindOne",
            ts::reference("ITradeProductFindOneArgs"),
        ),
        ts::type_alias(
            "ITradeProductFindOneArgs",
            ts::object(vec![ts::field(
                "on",
                ts::reference("TradeProductQueryBindValues"),
            )]),
        ),
        ts::type_alias(
            "ITradeProductFindOneResolve",
            ts::generic("IResult", vec![ts::reference("TradeProduct")]),
        ),
        ts::type_alias(
            "ITradeProductLocationRelation",
            ts::object(vec![
                ts::field(
                    "trade_product",
                    ts::reference("TradeProductQueryBindValues"),
                ),
                ts::field("gcs_location", ts::reference("GcsLocationQueryBindValues")),
            ]),
        ),
        ts::type_alias("ITradeProductLocationResolve", ts::reference("IResultPass")),
        ts::type_alias(
            "ITradeProductMediaRelation",
            ts::object(vec![
                ts::field(
                    "trade_product",
                    ts::reference("TradeProductQueryBindValues"),
                ),
                ts::field("media_image", ts::reference("MediaImageQueryBindValues")),
            ]),
        ),
        ts::type_alias("ITradeProductMediaResolve", ts::reference("IResultPass")),
        ts::type_alias(
            "ITradeProductUpdate",
            ts::object(vec![
                ts::field("on", ts::reference("TradeProductQueryBindValues")),
                ts::field("fields", ts::reference("ITradeProductFieldsPartial")),
            ]),
        ),
        ts::type_alias(
            "ITradeProductUpdateResolve",
            ts::generic("IResult", vec![ts::reference("TradeProduct")]),
        ),
        ts::type_alias(
            "LogError",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("error", ts::string()),
                ts::field("message", ts::string()),
                ts::field("stack_trace", ts::union(vec![ts::string(), ts::null()])),
                ts::field("cause", ts::union(vec![ts::string(), ts::null()])),
                ts::field("app_system", ts::string()),
                ts::field("app_version", ts::string()),
                ts::field("nostr_pubkey", ts::string()),
                ts::field("data", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "LogErrorQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("nostr_pubkey", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "MediaImage",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("file_path", ts::string()),
                ts::field("mime_type", ts::string()),
                ts::field("res_base", ts::string()),
                ts::field("res_path", ts::string()),
                ts::field("label", ts::union(vec![ts::string(), ts::null()])),
                ts::field("description", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "MediaImageFindManyRel",
            ts::union(vec![
                ts::object(vec![ts::field(
                    "on_trade_product",
                    ts::reference("MediaImageTradeProductArgs"),
                )]),
                ts::object(vec![ts::field(
                    "off_trade_product",
                    ts::reference("MediaImageTradeProductArgs"),
                )]),
            ]),
        ),
        ts::type_alias(
            "MediaImageQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("file_path", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "MediaImageTradeProductArgs",
            ts::object(vec![ts::field("id", ts::string())]),
        ),
        ts::type_alias(
            "NostrEventState",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("key", ts::string()),
                ts::field("kind", ts::number()),
                ts::field("pubkey", ts::string()),
                ts::field("d_tag", ts::string()),
                ts::field("last_event_id", ts::string()),
                ts::field("last_created_at", ts::number()),
                ts::field("content_hash", ts::string()),
            ]),
        ),
        ts::type_alias(
            "NostrEventStateQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("key", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "NostrProfile",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("public_key", ts::string()),
                ts::field("profile_type", ts::string()),
                ts::field("name", ts::string()),
                ts::field("display_name", ts::union(vec![ts::string(), ts::null()])),
                ts::field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::field("website", ts::union(vec![ts::string(), ts::null()])),
                ts::field("picture", ts::union(vec![ts::string(), ts::null()])),
                ts::field("banner", ts::union(vec![ts::string(), ts::null()])),
                ts::field("nip05", ts::union(vec![ts::string(), ts::null()])),
                ts::field("lud06", ts::union(vec![ts::string(), ts::null()])),
                ts::field("lud16", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "NostrProfileFindManyRel",
            ts::union(vec![
                ts::object(vec![ts::field(
                    "on_relay",
                    ts::reference("NostrProfileRelayArgs"),
                )]),
                ts::object(vec![ts::field(
                    "off_relay",
                    ts::reference("NostrProfileRelayArgs"),
                )]),
            ]),
        ),
        ts::type_alias(
            "NostrProfileQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("public_key", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "NostrProfileRelayArgs",
            ts::object(vec![ts::field("id", ts::string())]),
        ),
        ts::type_alias(
            "NostrRelay",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("url", ts::string()),
                ts::field("relay_id", ts::union(vec![ts::string(), ts::null()])),
                ts::field("name", ts::union(vec![ts::string(), ts::null()])),
                ts::field("description", ts::union(vec![ts::string(), ts::null()])),
                ts::field("pubkey", ts::union(vec![ts::string(), ts::null()])),
                ts::field("contact", ts::union(vec![ts::string(), ts::null()])),
                ts::field("supported_nips", ts::union(vec![ts::string(), ts::null()])),
                ts::field("software", ts::union(vec![ts::string(), ts::null()])),
                ts::field("version", ts::union(vec![ts::string(), ts::null()])),
                ts::field("data", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "NostrRelayFindManyRel",
            ts::union(vec![
                ts::object(vec![ts::field(
                    "on_profile",
                    ts::reference("NostrRelayProfileArgs"),
                )]),
                ts::object(vec![ts::field(
                    "off_profile",
                    ts::reference("NostrRelayProfileArgs"),
                )]),
            ]),
        ),
        ts::type_alias(
            "NostrRelayProfileArgs",
            ts::object(vec![ts::field("public_key", ts::string())]),
        ),
        ts::type_alias(
            "NostrRelayQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("url", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "Plot",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("d_tag", ts::string()),
                ts::field("farm_id", ts::string()),
                ts::field("name", ts::string()),
                ts::field("about", ts::union(vec![ts::string(), ts::null()])),
                ts::field(
                    "location_primary",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
                ts::field("location_city", ts::union(vec![ts::string(), ts::null()])),
                ts::field("location_region", ts::union(vec![ts::string(), ts::null()])),
                ts::field(
                    "location_country",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
            ]),
        ),
        ts::type_alias(
            "PlotGcsLocation",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("plot_id", ts::string()),
                ts::field("gcs_location_id", ts::string()),
                ts::field("role", ts::string()),
            ]),
        ),
        ts::type_alias(
            "PlotGcsLocationQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("plot_id", ts::string())]),
                ts::object(vec![ts::field("gcs_location_id", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "PlotQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("d_tag", ts::string())]),
                ts::object(vec![ts::field("farm_id", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "PlotTag",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("plot_id", ts::string()),
                ts::field("tag", ts::string()),
            ]),
        ),
        ts::type_alias(
            "PlotTagQueryBindValues",
            ts::union(vec![
                ts::object(vec![ts::field("id", ts::string())]),
                ts::object(vec![ts::field("plot_id", ts::string())]),
                ts::object(vec![ts::field("tag", ts::string())]),
            ]),
        ),
        ts::type_alias(
            "TradeProduct",
            ts::object(vec![
                ts::field("id", ts::string()),
                ts::field("created_at", ts::string()),
                ts::field("updated_at", ts::string()),
                ts::field("key", ts::string()),
                ts::field("category", ts::string()),
                ts::field("title", ts::string()),
                ts::field("summary", ts::string()),
                ts::field("process", ts::string()),
                ts::field("lot", ts::string()),
                ts::field("profile", ts::string()),
                ts::field("year", ts::bigint()),
                ts::field("qty_amt", ts::number()),
                ts::field("qty_amt_exact", ts::union(vec![ts::string(), ts::null()])),
                ts::field("qty_unit", ts::string()),
                ts::field("qty_label", ts::union(vec![ts::string(), ts::null()])),
                ts::field("qty_avail", ts::union(vec![ts::bigint(), ts::null()])),
                ts::field("price_amt", ts::number()),
                ts::field("price_amt_exact", ts::union(vec![ts::string(), ts::null()])),
                ts::field("price_currency", ts::string()),
                ts::field("price_qty_amt", ts::number()),
                ts::field(
                    "price_qty_amt_exact",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
                ts::field("price_qty_unit", ts::string()),
                ts::field("listing_addr", ts::union(vec![ts::string(), ts::null()])),
                ts::field("primary_bin_id", ts::union(vec![ts::string(), ts::null()])),
                ts::field(
                    "verified_primary_bin_id",
                    ts::union(vec![ts::string(), ts::null()]),
                ),
                ts::field("notes", ts::union(vec![ts::string(), ts::null()])),
            ]),
        ),
        ts::type_alias(
            "TradeProductQueryBindValues",
            ts::object(vec![ts::field("id", ts::string())]),
        ),
    ])
}
