# Software Requirements Document - 'alternatives'

## Preliminary Statements
This document has no ambition to align with IEEE 830-1998 or later specification. 

## Purpose
This document serves mainly as a storage of requirements for this project. Such requirements shall be specified to enable full backwards compatibility between this project and 'update alternatives' code. In order to attain such functionality, the requirements shall be collected, described and stored here. The document is created namely for better work division among the contributors to this project.

## Scope
This project is a reimplementation of 'update-alternatives', a program to create, delete and maintain a link system which enables switching between different implementations of other software products within one instance of a linux-based operating system.

## Collected Requirements
[this section shall be deleted later]: #
The future implementation (i.e. program developed in this repository) shall fully comply with the following list of requirements:  
- fully compatible with `update-alternatives` program (i.e. 'alternatives' package used in Fedora and its downstream OSes) on input/output level

- no calls from specfiles -> drop-ins
- readable config format (probably JSON/YAML)
- readable code
- nice to have: dbus integration (namespaces)
- recognize per-user/per-system alternatives
- remove the leader/follower paradigm

## Overall Description
### Product Perspective
#### System Interfaces
#### User Interfaces

## Specific Requirements

### External Interfaces
### Functional Requirements
[add alternative]: #
[remove alternative]: #
[switch alternative]: #
[modify alternative]: #
    [family]: #
    [initscripts]: #
### Database Requirements