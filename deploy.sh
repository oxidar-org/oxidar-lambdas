#!/bin/bash
cargo lambda build --release

ENV_REDIS_URL="rediss://lambda-demo-permissions-x3duz1.serverless.sae1.cache.amazonaws.com:6379"
ENV_JWKS_URL="https://hkpktejt2fjgm4to4ihskvuiha0uxxau.lambda-url.sa-east-1.on.aws/"
LAMBDA_ROLE="arn:aws:iam::619071353885:role/lambda-execution-role"
RUST_LOG="WARN"

echo "deploying add_path lambda..."
cargo lambda deploy add_path \
    --disable-function-url \
    --role $LAMBDA_ROLE \
    --env-var RUST_LOG="$RUST_LOG" \
    --env-var REDIS_URL="$ENV_REDIS_URL"

echo "deploying remove_path lambda..."
cargo lambda deploy remove_path \
    --disable-function-url \
    --role $LAMBDA_ROLE \
    --env-var RUST_LOG="$RUST_LOG" \
    --env-var REDIS_URL="$ENV_REDIS_URL"

echo "deploying jwks lambda..."
cargo lambda deploy jwks \
    --enable-function-url \
    --env-var RUST_LOG="$RUST_LOG" \
    --role $LAMBDA_ROLE

echo "deploying validate_jwt lambda..."
cargo lambda deploy validate_jwt \
    --role arn:aws:iam::619071353885:role/lambda-execution-role \
    --env-var RUST_LOG="$RUST_LOG" \
    --env-var REDIS_URL="$ENV_REDIS_URL" \
    --env-var JWKS_URL="$ENV_JWKS_URL"

echo "deploying echo lambda..."
cargo lambda deploy echo \
    --env-var RUST_LOG="$RUST_LOG" \
    --role $LAMBDA_ROLE

