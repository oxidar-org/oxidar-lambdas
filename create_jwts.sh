#!/bin/bash

generate_random_email() {
    local domains=("gmail.com" "yahoo.com" "outlook.com" "protonmail.com" "example.com")
    local domain=${domains[$RANDOM % ${#domains[@]}]}
     # Generate a random alphanumeric string (letters and numbers only)
    local username=$(od -An -N10 -tuC /dev/urandom | awk '{for(i=1;i<=NF;i++) printf "%c", ($i%26)+97; print ""}')

    echo "${username}@${domain}"

}

get_rol() {
    local roles=("admin" "super_user" "user" )
    rol=${roles[$RANDOM % ${#roles[@]}]}

    echo "$rol"
}

# rol -> url admitida

for ((i=1; i<=20; i++)); do
    # Configuration
    SERVICE_URL="http://localhost:8080/jwt/sign"

    user=$(generate_random_email)
    rol=$(get_rol)

    # Fetch data from the service
    response=$(curl -s -X POST -H "Content-Type: application/json" -d "{ \"sub\": \"$user\", \"roles\": [\"$rol\"] }" $SERVICE_URL)


    # Extract value (assuming JSON response like { "key": "value" })
    value=$(echo "$response" | jq -r '.jwt')

    echo "$user|$value" >> users.txt

    # Check if value is valid
    if [[ -n "$value" && "$value" != "null" ]]; then
        # Store in Redis
        # redis-cli -h "$REDIS_HOST" -p "$REDIS_PORT" SET "$REDIS_KEY" "$value"
        docker exec -t oxidar-redis redis-cli SET "$user" "$rol"
        echo "Stored in Redis: $user -> $rol"
    else
        echo "Failed to extract value from response"
        exit 1
    fi
done
