#!/bin/bash

# first install cbor util using:
# sudo npm install -g cbor-cli
#
curl 'http://localhost:3550/users/userid1' | cbor2json
