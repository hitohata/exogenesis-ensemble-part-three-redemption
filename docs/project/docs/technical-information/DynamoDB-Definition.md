---
sidebar_position: 2
---

# DynamoDB Definition

## Key

* PK: String
* SK: Number

## Collection

### Manage Files

| Key        | Detail     | Note                           |
|:-----------|:-----------|:-------------------------------|
| PK         | Year       |                                |
| SK         | Epoch time |                                |
| IsUnzipped | boolean    | If the file is unzipped or not |
| Vault      | String     | Glacier vault                  |
| KeyName    | String     | S3 prefix                      |

### Date look up

For the search sake.

| Key       | Derail                                 | Note                   |
|:----------|:---------------------------------------|:-----------------------|
| PK        | date                                   | `{Year}-{month}-{day}` |
| SK        | Dummy                                  | fixed number "0"       |
| SavedDate | The list of the years, months, or days |                        |

The key is defined based on the data.
If the `{year}` has months in the year.
If the `{year}-{month}` has days in the month of the year.

### Manage Unzipped Files

This is for the unzipped files

| Key     | Detail             | Note                                    |
|:--------|:-------------------|:----------------------------------------|
| PK      | Unzipped           | Fixed string "Unzipped"                 |
| SK      | Epoch time         | The datetime that the file is retrieved |
| KeyName | name of object key | The object key name                     |
