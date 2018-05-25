# The following are targers that do not exist in the filesystem as real files and should be always executed by make
.PHONY: default deps login base build dev shell start stop test image

# Name of this service/application
SERVICE_NAME := message_indexer_bot

# Name of the service used to run the shell (overridable)
SHELL_SERVICE ?= $(SERVICE_NAME)

# Docker image name for this project
IMAGE_NAME := jdvr/$(SERVICE_NAME)


# Shell to use for running scripts
SHELL := $(shell which bash)

# Get docker path or an empty string
DOCKER := $(shell command -v docker)

# Get docker-compose path or an empty string
DOCKER_COMPOSE := $(shell command -v docker-compose)

# Get the main unix group for the user running make (to be used by docker-compose later)
GID := $(shell id -g)

# Get the unix user id for the user running make (to be used by docker-compose later)
UID := $(shell id -u)


# Get the username of theuser running make. On the devbox, we give priority to /etc/username
USERNAME ?= $(shell ( [ -f /etc/username ] && cat /etc/username  ) || whoami)

# Compose project name
export COMPOSE_PROJECT_NAME = $(SERVICE_NAME)

# The default action of this Makefile is to build the development docker image
default: build

# Test if the dependencies we need to run this Makefile are installed
deps:
ifndef DOCKER
	@echo "Docker is not available. Please install docker"
	@exit 1
endif
ifndef DOCKER_COMPOSE
	@echo "docker-compose is not available. Please install docker-compose"
	@exit 1
endif

# Build the development docker image
build: base
	cd env && docker-compose build

run: build
	cd env && docker-compose up


# Run a shell into the development docker image
shell: build
	cd env && \
	docker-compose run --rm $(SHELL_SERVICE) /bin/bash

# Run test
test: build
	cd env && \
	docker-compose run --rm $(SERVICE_NAME) \
	bash -c "cargo test"

# Stop the development environment (background and/or foreground)
stop:
	cd environment/dev && ( \
		docker-compose stop; \
		docker-compose rm -f; \
		)

# Clean project containers
clean:
	cd environment/dev && \
	docker-compose stop && \
	docker-compose rm --all -f
