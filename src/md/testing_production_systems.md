# All you need is *love* (high quality data)

## I. It's hard to test what you don't know

There's things you know, things that you know that you don't know, and the things that you don't know that you don't know or so goes the famous phrase.

Working in the position of data integrity at a company can be filled with things that you don't know you don't know. We receive many files from different clients. Each file has a certain format we expected, but if something goes wrong. We have to figure out what to do about it.

I've found applying a layer of tests to your ingestion pipelines can be somewhat helpful. Add checks on your unique identifiers and `not null` tests on your most valuable columns. 

It's hard to know, however, what to do when you receive data that meets your expectations but fails in other ways. What happens when the schema changes halfway through. Or when the data lands in a different folder.

You have to go back to your client and ask what went wrong. And you have to figure out how to keep your operations going through these issues.

You can only test what you know and what you know you don't know. But you can't test what you don't know you don't know. Those are the aspects that can bring your service down overnight. Or result in wrong data for the users of your service.

## II. Throwing a wrench at your systems, voluntarily 

## III. Learning from the results


...

## Content to add
[Add content on data reliability as a whole]
[Add content on integration tests]
[Add notes on Chaos Monkeys and why you need randomness in your testing]
[Add content on smoke gun tests]
[Add content on DBT tests and what some of that can help]


___
https://www.montecarlodata.com/blog-data-quality-checks-in-etl/
https://www.metaplane.dev/state-of-data-quality-monitoring-2023
https://www.datafold.com/blog/7-dbt-testing-best-practices
*Testing in production blog*