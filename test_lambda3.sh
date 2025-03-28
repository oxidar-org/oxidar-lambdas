#!/bin/bash

# Define ANSI color codes
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m'  # No Color

get_path() {
    local paths=("/admin" "/echo")
    path=${paths[$RANDOM % ${#paths[@]}]}
    echo "$path"
}

for ((i=1; i<=200; i++)); do
    user=$(awk -v seed=$RANDOM 'BEGIN {srand(seed)} {lines[++count] = $0} END {print lines[int(rand() * count) + 1]}' users.txt)
    email=$(echo "$user" | cut -d '|' -f1)
    token=$(echo "$user" | cut -d '|' -f2)
    role=$(echo "$user" | cut -d '|' -f3)

    # Assign color based on role
    case "$role" in
        "admin") color=$GREEN ;;
        "super_user") color=$YELLOW ;;
        "user") color=$BLUE ;;
        *) color=$NC ;;  # Default (no color)
    esac

    echo -e "Hitting /echo with user $email and role ${color}$role${NC}"

    curl -X POST \
        https://lwmywod41h.execute-api.sa-east-1.amazonaws.com/production/echo \
        -H "Authorization: Bearer $token" \
        -H "Content-Type: application/json" -d '{ "message": "Holaa" }'

    echo ""
done

