# Cloud System

## Architecture

- This Cloud System hjas an Adapter system that allows the user to use the system with diffrent hosting methods.
- The default methods are: `docker` and `local`.

## How to use

- Setup the manager on your main server.
- If you want the manager to be a node as well, you can do that in the setup.

## Plugins

- Rest Plugin

## Adapters

- Docker
- Local (like on the traditional cloud systems)

## Multi Node

- Every Adapter is responsible for node management, so the docker adapter will manage the docker nodes and the local adapter will manage the local nodes.
