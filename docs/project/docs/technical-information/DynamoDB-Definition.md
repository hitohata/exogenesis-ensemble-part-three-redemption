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

### Manage Years

The year is the PK, but there is a requirement that you get all years.
To hold years in one place, this is used.

| Key   | Derail    | Note               |
|:------|:----------|:-------------------|
| PK    | Years     | fixed name "Years" |
| SK    | Dummy     | fixed number "0"   |
| Years | all years | the list of years  |


### Manage Unzipped Files

This is for the unzipped files

| Key      | Detail             | Note                                    |
|:---------|:-------------------|:----------------------------------------|
| PK       | Unzipped           | Fixed string "Unzipped"                 |
| SK       | Epoch time         | The datetime that the file is retrieved |
| FileName | name of object key | The object key name                     |
