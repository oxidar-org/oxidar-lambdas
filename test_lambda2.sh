#!/bin/bash
for ((i=1; i<=200; i++)); do
    curl -i -X POST https://lwmywod41h.execute-api.sa-east-1.amazonaws.com/production/echo -H "Authorization: $1" -H "Content-Type: application/json" -d '{ "message": "Holaa" }'
done
