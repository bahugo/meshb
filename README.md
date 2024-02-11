# Meshb

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
- [ ] SEG3 cell
- [ ] SEG4 cell
- [ ] TRIA3 cell
- [ ] TRIA6 cell
- [ ] TRIA7 cell
- [ ] QUAD4 cell
- [ ] QUAD8 cell
- [ ] QUAD9 cell
- [ ] HEXA8 cell
- [ ] HEXA20 cell
- [ ] HEXA27 cell
- [ ] PENTA6 cell
- [ ] PENTA15 cell
- [ ] PENTA18 cell
- [ ] TETRA4 cell
- [ ] TETRA10 cell
- [ ] PYRAM5 cell
- [ ] PYRAM13 cell

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

