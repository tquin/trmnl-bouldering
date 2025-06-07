#!/usr/bin/env bash

source /etc/os-release

if [[ "$PRETTY_NAME" =~ "Fedora Linux 42" ]]; then
    dnf install \
        pkgconf perl-FindBin perl-IPC-Cmd openssl-devel
else
    echo "Unsupported OS version"
    exit 1
fi
