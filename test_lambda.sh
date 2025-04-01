#!/bin/bash
get_path() {
    local paths=("/admin" "/echo" )
    path=${paths[$RANDOM % ${#paths[@]}]}

    echo "$path"
}

for ((i=1; i<=200; i++)); do
    user=$(awk -v seed=$RANDOM 'BEGIN {srand(seed)} {lines[++count] = $0} END {print lines[int(rand() * count) + 1]}' users.txt)
    email=$(echo $user | cut -d '|' -f1)
    token=$(echo $user | cut -d '|' -f2)

    cargo lambda invoke validate_jwt \
        --data-ascii "{ \
            \"headers\": { \
                \"authorization\": \"Bearer $token\" \
            }, \
            \"path\": \"$(get_path)\", \
        \"methodArn\": \"arn:aws:execute-api:sa-east-1:123456789012:api-id/stage/GET/echo\" \
    }" | jq
done
