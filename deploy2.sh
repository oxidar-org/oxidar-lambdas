#!/bin/bash

cargo lambda build --release

ENV_REDIS_URL="rediss://lambda-demo-permissions-x3duz1.serverless.sae1.cache.amazonaws.com:6379"
ENV_JWKS_URL="https://hkpktejt2fjgm4to4ihskvuiha0uxxau.lambda-url.sa-east-1.on.aws/"
LAMBDA_ROLE="arn:aws:iam::619071353885:role/lambda-execution-role"

echo "deploying validate_jwt lambda..."
cargo lambda deploy validate_jwt \
    --role arn:aws:iam::619071353885:role/lambda-execution-role \
    --env-var REDIS_URL="$ENV_REDIS_URL" \
    --env-var JWKS_URL="$ENV_JWKS_URL"

