# EDGE Mods

This directory contains several Mods.

## How to build an .asi mod

**Requirements**:

* [Xmake](https://xmake.io/) Build System
* Visual C++ (2026 Recommended)
* LLVM with Clang-CL compiler

The following commands are assumed to be run from the Visual Studio Developer Command Prompt.

First of all, you need to generate local package for `EdgeMinHook`. Just run `generate.cmd`.

Then set the build directory by entering the following command:

```
xmake f
```

Then build the project:

```
xmake build
```

### Switching between debug and release

To switch to release mode:

```
xmake f -m release
```

To switch to debug mode:

```
xmake f -m debug
```
