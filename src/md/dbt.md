## Dbt Notes

### Incremental models:

They need:
- a **filter** to select just the new or updated records
- a **conditional** block that wraps our filter and only applies it when we want it
- **configuration** that tells dbt we want to build incrementally and helps apply the conditional filter when needed

```sql

select * from orders

where
  updated_at > (select max(updated_at) from {{ this }})
  
```

**Config for incremental models**

Note: order_id can also be an array ["a","b"] or you can define a surrogate key.

```
{{
    config(
        materialized='incremental',
        unique_key='order_id'
    )
}}

select ...
```

We’ve added a new config option unique_key, that tells dbt that **if it finds a record in our previous run** — the data in the warehouse already — with the same unique id (in our case order_id for our orders table) that exists in the new data we’re adding incrementally, **to update that record instead of adding it as a separate row.**

**Full example**

```sql
{{
    config(
        materialized='incremental',
        unique_key='order_id'
    )
}}

select * from orders

{% if is_incremental() %}

where
  updated_at > (select max(updated_at) from {{ this }})

{% endif %}
```

**Strategies**

**Append**: only use if duplicates aren't a problem. Takes records and inserts them; it can't update or delete records.

![](https://miro.medium.com/v2/resize:fit:640/format:webp/1*f2dyZA9N_j8ox9RkOwegUg.png)

**Merge**: Need to specify a unique key. If it exists, it will merge the update record.

![](https://miro.medium.com/v2/resize:fit:640/format:webp/1*o59bXBWxVHQ7-8vTcQGHEA.png)

**Summary of Strategies**

![](https://miro.medium.com/v2/resize:fit:640/format:webp/1*MTBNKDCa_csd--9qk4b9pA.png)

### is_incremental()
The is_incremental() macro will return True if all of the following conditions are met:
1. the destination table already exists in the database
2. dbt is not running in full-refresh mode
3. the running model is configured with materialized='incremental'

**Considerations**
- Run full-refresh on the weekends or on a weekly or monthly to prevent data drift (dbt build --ful-refresh -s orders)

