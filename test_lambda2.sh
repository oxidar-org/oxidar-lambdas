#!/bin/bash
for ((i=1; i<=200; i++)); do
    curl -i -X POST https://lwmywod41h.execute-api.sa-east-1.amazonaws.com/production/echo -H "Authorization: eyJhbGciOiJSUzI1NiIsImtpZCI6InIzZ0NCWnBBcUliWWxMdG05TGxPV2Z0TFZYTE9FNXU2bTh4ZW12Z2JlSEkiLCJ0eXAiOiJKV1QifQ.eyJleHAiOjE3NDE5ODMxMDksInJvbGVzIjpbInVzZXIiXSwic3ViIjoidHZocnBkZGtwdEBnbWFpbC5jb20ifQ.BZ_mRFor8JKlYU8DlQ-nSECUJXMNAss9kLyfNs9ZvCzPtk0vD7yATJ0fDfmyYls_XgAIUCmyb7bHB2BrcdwSF89PYsCK9REnWR9u6Sv7Wq55PN2AwdRYWijcIdjT1Hk-E3FmEop08TZT8EGGaPcYO0ONEaMTppRI81x7hxl9JwfY0kpuRcKQLBPRiDgCDUVxeTld7tkTD4ubha-HWHKN0gW0fXZmyqOzKwl4jftb20ZXIWmu8-Jut6MEYsYVPgCF7Bh6rFFjHELv7CWvd0T2wLqYeg-i01pkjEoF_92JW5F4HzgKC4OWZ71QMtjjqb3iHEx74WEOofSRM1t_dIFgRw" -H "Content-Type: application/json" -d '{ "message": "Holaa" }'
done
