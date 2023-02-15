#!/bin/bash
cd ui
yarn build-prod
cd ../rbc-rs
exec cargo run --release --bin web