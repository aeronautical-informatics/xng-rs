[![Workflow Status](https://github.com/aeronautical-informatics/xng-rs/workflows/main/badge.svg)](https://github.com/aeronautical-informatics/xng-rs/actions?query=workflow%3A%22main%22)
[![Percentage of issues still open](https://isitmaintained.com/badge/open/aeronautical-informatics/xng-rs.svg)](https://isitmaintained.com/project/aeronautical-informatics/xng-rs "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)


# xng-rs

A Rust wrapper for the XNG API

This crate provides a thin wrapper for the C ABI of [fentISS'](https://fentiss.com/)
[Xtratum Next Generation (XNG)](https://fentiss.com/products/hypervisor/)
separation kernel/type 1 hypervisor. It allows the implementation of bare metal
(`no_std`) partitions for XNG, using the XNG Runtime Environment (XRE).


# Usage

In order to compile this crate, a compiler must be able to pick up your XNG
header files. An easy way to achieve this is to set the `C_INCLUDE_PATH` env
var to the folder containing the fentiss header files when running `cargo`.
Example:

```console
$ C_INCLUDE_PATH=/my/xng/installation/include cargo build
```


## About the Project

This is by no means ready - it is an ongoing progress. While we've already used this together
with FentISS' Separation Kernel Emulator (SKE), it was __not__ throughfully tested. While
we are engaged with FentISS, there is no official support for this neither from FentISS nor
from us. However, if you encounter any problems, please open up an issue. The chances are that
we care and try to fix the issue.
