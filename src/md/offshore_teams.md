
## Notes on working with offshore teams

Outline: Why you can't oursource your architecture
- I joined a company that had used an offshore team for all its architecture
- It was taxing to understand the architecture and the decisions; the documentation was not good
- We decided to redesign the architecture in a way that made sense
- Slowly we architected the ingestion and transformation using better tools
- We still engage them but we use templates, async meetings, and PRs to align everyone remotely
- Offshore teams can be great if you know how to use them: give some examples

- Things that work
    - Empowering them 
        - Including them in discussions
    - Making them part of your team
    - Guiding their work, having an established review process
        - We didn't have PRs at first since it was all developed offshore
        - We slowly moved the key archiectural components onshore
    - Using tools that allow for PR reviews
        - e.g. avoiding stored procedures going to DBT
        - e.g., using ADF without version control at some point
        - Helps to avoid blaming others for issues and pointing fingers
    - Establishing templates for key parts of the work you want to review
        - PR templates
        - Pipeline and code templates
        - Python code templates
    - Setting up cadences to review code during major releases
        - Review major engineering decisions together
    - Communicate async and delegate work overnight
        - Taking advantage of the one perk of working with offshore teams
    - Decide what kind of work is better done offshore vs. onshore
        - e.g., high interaction with business teams is sometimes better onshore
        - e.g., routine processes like new ingestion pipelines can be better offshore
    - Know who does what offshore and get to know them one on one
        - Evaluate the quality of the work and set expectations
        - Sometimes if its a staffing company they can replace people

- Things that don't work
    - Delegating architecture to their team
    - Leaving key parts of your skills or technology away
