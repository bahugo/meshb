# Meshb

[![Build Status][build-badge]][build]

[build-badge]: https://img.shields.io/github/actions/workflow/status/bahugo/meshb/rust.yml?style=flat-square&branch=master
[build]: https://github.com/bahugo/meshb/actions/

## Introduction

Meshb is a rust library to handle FEM meshes.

**Its main goal is to be able :**
- load a mesh from a file,
- perform modifications (nodes, cells, groups, ...),
- dump modifications to a file

## Current development status

Meshb is an experimental personnal project. It's not ready for production yet.

**Parsers :**
- [x] .mail format

**Elementary elements :**
- [x] Node
- [x] POI1 cell
- [x] SEG2 cell
- [x] SEG3 cell
- [x] SEG4 cell
- [x] TRIA3 cell
- [x] TRIA6 cell
- [x] TRIA7 cell
- [x] QUAD4 cell
- [x] QUAD8 cell
- [x] QUAD9 cell
- [x] HEXA8 cell
- [x] HEXA20 cell
- [x] HEXA27 cell
- [x] PENTA6 cell
- [x] PENTA15 cell
- [x] PENTA18 cell
- [x] TETRA4 cell
- [x] TETRA10 cell
- [x] PYRAM5 cell
- [x] PYRAM13 cell

**Mesh container :**
- [x] Create nodes
- [x] Edit existing nodes
- [x] Create cells
- [x] Edit existing cells
- [x] Create group of nodes
- [x] Create group of cells
- [ ] Instanciate mesh from parser

**Misc :**
- [ ] Create python bindings

## Available parsers

### Code_aster .mail format

Code_aster is a FEA software. Here is the .mail format documentation :

- [In french](https://code-aster.org/V2/doc/default/en/man_u/u3/u3.01.00.pdf)
- [In english](https://code-aster.org/V2/doc/default/en/man_u/u3/u3.01.00.pdf)
