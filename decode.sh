#!/usr/bin/env bash

echo "$(<src/euler/encoded_problems.ers)" | openssl aes-256-cbc -d -a > src/euler/euler_problems.rs
echo "$(<src/euler/encoded_tests.ers)" | openssl aes-256-cbc -d -a > src/euler/test_answers.rs
