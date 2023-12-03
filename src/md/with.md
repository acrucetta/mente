with
    source as (
        select *
        from {{ source("nadac", "nadac_raw") }}
        where gpi is not null and classification_for_rate_setting = 'G'
    )

select
    -- identifiers
    'NADAC' as source_type,
    'cerpass' as data_type,
    'nadac_mac' as mac_name,
    trim(gpi) as gpi,

    -- pricing
    0 as admin_fee,
    0 as gpi_level_nadac_price,
    cast(nadac_per_unit as real) as price
from source
