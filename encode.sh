#!/usr/bin/env bash

echo "$(<src/euler/euler_problems.rs)" | openssl aes-256-cbc -a -salt > src/euler/encoded_problems.ers
echo "$(<src/euler/test_answers.rs)" | openssl aes-256-cbc -a -salt > src/euler/encoded_tests.ers
echo "$(<src/abbey/abbey_problems.rs)" | openssl aes-256-cbc -a -salt > src/abbey/encoded_problems.ers
