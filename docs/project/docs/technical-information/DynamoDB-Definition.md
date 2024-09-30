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

### Manage Unzipped Files

| Key | Detail     | Note                                |
|:----|:-----------|:------------------------------------|
| PK  | Unzipped   |                                     |
| SK  | Epoch time | The datetime that should be removed |
