#!/bin/bash
awslocal s3 mb s3://test-bucket
awslocal s3api put-object --bucket test-bucket --key 1984/04/04/1984-04-04-12-34-50.MOV --body /data/test.MOV
awslocal s3api put-object --bucket test-bucket --key 1984/04/04/1984-04-04-12-34-51.MOV --body /data/test.MOV
awslocal s3api put-object --bucket test-bucket --key 1984/04/05/1984-04-05-12-34-50.MOV --body /data/test.MOV
awslocal s3api put-object --bucket test-bucket --key 1984/05/04/1984-05-04-12-34-50.MOV --body /data/test.MOV
awslocal s3api put-object --bucket test-bucket --key 1985/04/04/1985-04-04-12-34-50.MOV --body /data/test.MOV