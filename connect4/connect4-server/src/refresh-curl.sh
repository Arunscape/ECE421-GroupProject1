#!/bin/bash

curl --request POST \
  --url http://localhost:8000/api/refresh \
  --header 'authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJkYXRhIjp7InVzZXJuYW1lIjoidXNlcm5hbWUifSwiZXhwIjoxNTg2NzQzMDMyfQ.zPbekGNHM0brde8m8HMSXmfNP34Nm_qb2xCAH67kY3k'
