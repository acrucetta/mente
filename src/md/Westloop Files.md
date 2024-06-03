# Westloop Files

## Tasks
- Review Drug Search JSON and CSV; what are we loading
- Receive Autocomplete file as a CSV and load to Elastic
- Review the raw files and what fields we're using in the MPS portals

### Notes
- They send part of the drug search file as a CSV; don't know if its used or not
	- He knows what the JSON does, but not the CSV
	- There's multiple drug search files

### Mapping NDC File
- What NDCs related to which NDC; how pricing gets rolled up
- Similar pricing NDCs get priced the same; e.g., diff manufacturers of the same drug might not matter
	- Many NDCs can roll up 

### Autocomplete File
- He doesn't know if we use the rank; it used to show Opioids first
- Data
	- Base ID
	- Rank - rank things 
		- Based on utilization; not sure if its used
	- Name (what is used for search)
	- Display Name (what shows in the display under the drug)
	- NDC
	- Discontinued
	- AlternativeSearchTerms
		- Walmart cared about this so we added
- How it works
	- When we type a drug, it shows the drugs available
- Now west loop passes autocomplete; they created the index
- It became a separate file because we wanted to add discontinued stuff and alternative search terms

### Drug Search (JSON)
- Each drug has a base_id (e.g., brand id)
- Default Search Name
- Search Names
	- Drives different drug names? Not sure if we use it
	- Drives the Drug Name drop down
- Default NDC
- All Drugs
	- Each item in the array corresponds to one drug
	- Contains information about each drug 
	- It contains default quantity and all quantities
	- Package name drives the name attached to each quantity e.g., Bottle or Bottles
	- They send the all_drugs as a CSV
- The CSV for Drug Search won't be useful without the CSV

