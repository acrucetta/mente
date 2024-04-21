

Some notes:
- We move cash card pricing and medispan data from SFTP and Blob to Postgres; then we move it to Snowflake and Elastic Search
- We move the cashwrap claims data from SFTP and Blob to Snowflake directly, bypassing Postgres
- We move the ISP hide prices feed from Snowflake to Postgres and ultimately Elastic Search