#!/usr/bin/env bash
# Put this file in your home dir
export PATH="$PATH:/root/.cargo/bin"
cd /root/api
git pull
cargo build --release
systemctl restart api