#!/usr/bin/env bash

# We're using a shell script to act as our launcher, but you
# could use anything you like whether it be a Python script,
# a binary written in Rust, etc.

# This launcher just takes an argument specifying which
# binary in our bundle to invoke and how to do so. The first
# argument specifies which binary to execute while the rest
# of the arguments are passed to the binary.

function info() {
    echo "Please invoke one of the following executables:"
    echo "  compute"
    echo "  web"
    echo ""
    echo "You can do so using the following syntax:"
    echo "./launcher {executable} ...[arg]"
    exit 1
}

function run() {
    $(pwd)/$1 ${@:2}
}

case "$1" in
  compute|web)
    run $@
    ;;
  *)
    info
    ;;
esac
