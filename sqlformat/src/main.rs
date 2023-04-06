use sqlformat::{format, FormatOptions, Indent, QueryParams};

fn main() {
    let source_sql = r#"
WITH
pg AS (
    SELECT apartment_id, COUNT(DISTINCT(building_type_id)) count FROM dev_demo.apartment_maps
    WHERE at='20220422_100724' AND (
    (building_type_id='a7b5c26b-da87-5d5a-acbe-01512b279ee1' AND building_id IN ('2bd2fee8-b6c5-5f1d-bff4-c3470dfde795','91b471b3-e1b2-5e60-b688-c8900b065e62','4ec4708d-86eb-54f9-b6e5-eddcac3e5c18'))
    OR
    (building_type_id='d34b955b-454f-554f-b486-63b956d31669' AND building_id IN ('00208bd2-8ed5-5cbf-b907-c8150325cfd9','4812cd30-1c11-5496-a8b3-506ae280a71a','4ee91f44-1775-5d87-8c67-238ac2131881'))
    )
    GROUP BY apartment_id
    ORDER BY apartment_id
)
SELECT 
    *,
case
        when "default_risk_score" is null OR "earning_contribution_score" is null then -1
        else
            case when "default_risk_score">=49.2 THEN 1 ELSE 0 END
            +
            case when "earning_contribution_score">=51.2 THEN 2 ELSE 0 END
        end "quadrant"
    FROM "dev_demo"."sales_apartment_agent_v20220301" "detail"
    
INNER JOIN pg ON pg.apartment_id=detail.apartment_id AND pg.count=2
    WHERE "at"='20220422_100724' AND "aggregation_period"='Last7Days' AND "seller_id"='0ad7a7b6-2cda-53f9-a4c2-c0fed0d3a181'
    AND case
        when "default_risk_score" is null OR "earning_contribution_score" is null then -1
        else
            case when "default_risk_score">=49.2 THEN 1 ELSE 0 END
            +
            case when "earning_contribution_score">=51.2 THEN 2 ELSE 0 END
        end IN (3, 1, -1)
    "#;
    let format_option = FormatOptions {
        indent: Indent::Spaces(4),
        uppercase: false,
        lines_between_queries: 1,
    };
    println!("{}", format(source_sql, &QueryParams::None, format_option))
}
