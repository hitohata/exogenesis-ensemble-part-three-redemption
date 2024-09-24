---
sidebar_position: 1
---

## Structure

![structure-overview](/img/structure/structure-overvies.drawio.svg)

1. The client App sends a request to the lambda, then the lambda sign and returns a pre-signed URL.
2. The client App uploads assets to the S3.
3. Once the assets are uploaded, the lambda is triggered with the uploaded asset information. 
4. The lambda archives the assets to the glacier
5. The lambda get a vault to the DynamoDB