#!/bin/bash

cargo lambda build --release

echo "deploying add_path lambda..."
cargo lambda deploy add_path \
    --disable-function-url \
    --role arn:aws:iam::619071353885:role/lambda-execution-role \
    --env-var REDIS_URL="redis://lambda-demo-permissions-x3duz1.serverless.sae1.cache.amazonaws.com:6379"

echo "deploying jwks lambda..."
cargo lambda deploy jwks \
    --enable-function-url \
    --role arn:aws:iam::619071353885:role/lambda-execution-role

echo "deploying validate_jwt lambda..."
cargo lambda deploy validate_jwt \
    --role arn:aws:iam::619071353885:role/lambda-execution-role \
    --env-var REDIS_URL="redis://lambda-demo-permissions-x3duz1.serverless.sae1.cache.amazonaws.com:6379" \
    --env-var JWKS_URL="https://hkpktejt2fjgm4to4ihskvuiha0uxxau.lambda-url.sa-east-1.on.aws/"

echo "deploying echo lambda..."
cargo lambda deploy echo \
    --role arn:aws:iam::619071353885:role/lambda-execution-role

