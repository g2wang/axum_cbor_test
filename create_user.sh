#!/bin/bash

curl -i -X POST 'http://localhost:3550/users' -H 'Content-Type: application/cbor' --data-binary @user.cbor
