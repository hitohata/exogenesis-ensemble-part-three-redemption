---
sidebar_position: 2
---

# DynamoDB Definition

## Key

* PK: String
* SK: Number

## Collection

### Manage Files

| Key        | Detail     | Note                                                     |
|:-----------|:-----------|:---------------------------------------------------------|
| PK         | datetime   | The key is `{year}-{month}-{day}-{hour}-{min}-{sec}`[^1] |
| SK         | Epoch time |                                                          |
| IsUnzipped | boolean    | If the file is unzipped or not                           |
| Vault      | String     | Glacier vault                                            |
| KeyName    | String     | S3 prefix                                                |

### Date Lookup

For the search sake.

| Key       | Derail                                              | Note                   |
|:----------|:----------------------------------------------------|:-----------------------|
| PK        | date                                                | `{Year}-{month}-{day}` |
| SK         | Epoch time | must be zero           |
| SavedDate | The list of the years, months, days, or objects key |                        |


:::note
For the year, the top is set a fixed string as `root`
:::


The key is defined based on the data.
If the `{year}` has months in the year.
If the `{year}-{month}` has days in the month of the year.

### Manage Unzipped Files

This is for the unzipped files

| Key         | Detail             | Note                                               |
|:------------|:-------------------|:---------------------------------------------------|
| PK          | Unzipped           | Fixed string "Unzipped"                            |
| CreatedDate | Epoch time         | The datetime that the file is retrieved or created |
| KeyName     | name of object key | The object key name                                |


### Manage Unzipping Files

This is for the unzipping files

| Key         | Detail             | Note                                            |
|:------------|:-------------------|:------------------------------------------------|
| PK          | Unzipped           | Fixed string "Unzipping"                        |
| CreatedDate | Epoch time         | The datetime that the retrieval request is made |
| KeyName     | name of object key | The object key name                             |


## Access Pattern

### Form client

The client stat reading from the root, then goto year, month, and day, and eventually, gets object lists on the specific day.

| OperationName                 | Input                            | Output                    | Description         |
|:------------------------------|:---------------------------------|:--------------------------|:--------------------|
| get years                     | None                             | list of years             | Get to Data look up | 
| get month                     | year                             | list of month             | Get to Data look up | 
| get days                      | year, month                      | list of days              | Get to Data look up | 
| get objects                   | year, month, day                 | list of objects           | Get to Data look up | 
| get archived file information | year, month, day, hour, min, sec | archived file information | Get to Manage Files |

### From Scheduler

| OperationName       | Input | Output                    | Description                         |
|:--------------------|:------|:--------------------------|:------------------------------------|
| list unzipped files | None  | unzipped file information | Query to the Managed Unzipped Files |


### When save a new object

1. Get years, months, days, objects from the Date Lookup
2. Check if the date exists in the list
3. If exists, do nothing. If not, append and save it 
4. save the object to the Manage File

This process can asynchronously
