<div align="center">
  <h1> Albert </h1>

  <img src="./docs/icons/icon-512.png" width="300" height="300" />

  <p>
    <strong> Rust Transpiler that converts a Data Mode like JSON/YAML to source code like Go, C++, and other data model like Graphql </strong>
  </p>

  <p>
   <img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/vincenzopalazzo/monkeyc/Sanity%20Check%20codebase?style=flat-square">
  </p>
</div>

> A computer program can write a computer program.                                    

## Table of Content

> This software is under development, please contribute with idea and/or development time.

- Introduction
- How to use
- Why to Use
- Appendix
- License

## Introduction

This traspiler born from an idea to override some difficult and boring process like convert a data model into a data structure
that need to be coded inside the source code.

A common case is to generate the source code model of a rest endpoint, or a typed client for an application that use json schema to
to define request and response.

However, this transpiler have big ambition, and we would like to support a large group of data model, and give the possibility
to hook the serializer to decode the final Abstract Syntax Tree into any source language specified by the end user.

## Appendix

The source of the name Albert is stolen from the name of the first monkey went into space on June 11, 1948, before do launch with humans NASA use
send this cool monkey on the space!

N.B: I love the monkey!
minimal procedural macros parser that produce a convenient AST
