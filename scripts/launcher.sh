#!/usr/bin/env bash

# We're using a shell script to act as our launcher, but you
# could use anything you like whether it be a Python script,
# a binary written in Rust, etc.

# This launcher just takes an argument specifying which
# binary in our bundle to invoke and how to do so. The first
# argument specifies which binary to execute while the rest
# of the arguments are passed to the binary.

# Exit on error.
set -o errexit
# Exit on error inside any functions/shells.
set -o errtrace
# Do not allow undefined vars.
set -o nounset
# Catch the error in case pipe expression failure
set -o pipefail

# Extract your current directory where this script is house.
CURRENT_HOME="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

function llog() {
    echo "[launcher] $@"
}

function info() {
    llog "Please invoke one of the following executables:"
    llog "  compute"
    llog "  web"
    llog ""
    llog "You can do so using the following syntax:"
    llog "./launcher compute | web ...[arg]"
    exit 1
}

function compute() {
    llog "Running compute application with configuration ${2}"
    "${CURRENT_HOME}/compute" "${2}"
    llog "FINISHED EXECUTING"
}

function web() {
  llog "Running web application with configuration ${2}"
  "${CURRENT_HOME}/web" "${2}"
  llog "FINISHED EXECUTING"
}

case "${1:-}" in
  compute|-c)
    compute "$@"
    ;;
  web|-w)
    web "$@"
    ;;
  *)
    llog "executing default action: info"
    info
esac
