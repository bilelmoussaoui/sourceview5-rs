#!/bin/sh

NAME=docker.io/bilelmoussaoui/gtk4

podman build . -t "${NAME}"
podman push "${NAME}"
