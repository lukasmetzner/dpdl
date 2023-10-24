# DPDL: Data Pipeline Description Language

![Tests](https://github.com/lukasmetzner/dpdl/workflows/Tests/badge.svg)

## Description

Design a domain-specific language (DSL) for educational purposes that encapsulates the process of data ingestion, transformation, and storage/transmission, accompanied by a parser and execution engine.

## Building
The application has only a few dependencies and can be easily build by running.
``` bash
cargo build
```

## Example Pipeline
This example pipeline will:
1. Read the contents of the file tests/test_file.txt.
2. Encode the contents using base64.
3. Return the content as `String`

``` bash
lukasmetzner@lukasmetzner dpdl % cat tests/test_file.txt
test_file
```
``` bash
lukasmetzner@lukasmetzner dpdl % cat main-pipeline.xml
<pipeline>
  <input>
    <file>./tests/test_file.txt</file>
  </input>
  <step>
    <base64/>
  </step>
</pipeline>
```
``` bash
lukasmetzner@lukasmetzner dpdl % ./dpdl main-pipeline.xml
GroupInstruction
├── GroupInstruction
│   └── FileInstruction
└── GroupInstruction
    └── Base64Instruction

dGVzdF9maWxl
```
The main function first converts the XML file into its unique instruction tree. Once this is done, if debugging is enabled, the tree structure is displayed. The instruction tree is then executed, with the results being output to stdout.