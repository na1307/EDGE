# EDGE Mods

This directory contains several Mods.

## How to build an .asi mod
### On Windows

**Requirements**:

* [Meson](https://mesonbuild.com/) Build System
* MSBuild or [Ninja](https://ninja-build.org/)
* [vcpkg](https://vcpkg.io/)
* Visual C++ (2026 Recommended)
* LLVM with Clang-CL compiler

The following commands are assumed to be run from the Visual Studio Developer Command Prompt.

First of all, you need to install the vcpkg packages for `EdgeMinHook`.

```
vcpkg install --triplet=x86-windows-static-custom
```

Then do any of the following:

#### Using Visual Studio

Set the build directory by entering the following command:

```
meson setup --backend=vs --native-file=../clang_win.ini buildDir
```

Then open the generated solution in Visual Studio.

#### Using Ninja

Set the build directory by entering the following command:

```
meson setup --backend=ninja --native-file=../clang_win.ini buildDir
```

To build, enter the following command:

```
meson compile -C buildDir
```

### On Linux (Cross Compilation)

**Requirements**:

* [Meson](https://mesonbuild.com/) and [Ninja](https://ninja-build.org/) Build System
* [xwin](https://github.com/Jake-Shadle/xwin)
* [vcpkg](https://vcpkg.io/)
* Clang-CL Compiler

First, you need to get MSVC CRT and Windows SDK via xwin.

```
xwin --arch x86 --accept-license splat --include-debug-libs --output <path>
```

Copy the cross_linux.ini.template file to cross_linux.ini and copy the windows-linuxcross.cmake.template file to windows-linuxcross.cmake. Then, modify the xwin paths in both files as appropriate.

Then you need to install the vcpkg packages.

```
vcpkg install --triplet=x86-windows-static-linuxcross
```

Set the build directory by entering the following command:

```
meson setup --cross-file=../cross_linux.ini buildDir
```

To build, enter the following command:

```
meson compile -C buildDir
```

#### CLion Integration

Integrating CLion with cross-compiling is a bit more complex, but not impossible.

Open Settings and go to Build, Execution, Deployment.

If the clang-cl toolchain isn't already set up, you'll need to do so first. Go to Toolchains. Click the plus button and select System. Then, configure the following:
  * Name: clang-cl
  * C Compiler: clang-cl
  * C++ Compiler: clang-cl

Go to Meson and configure it as follows:
  * Toolchain: clang-cl
  * Setup options: `--cross-file=../cross_linux.ini`
  * Compiler resolution: Host

CLion will now recognize the compiler.

### Switching between debug and release

To switch to release mode:

```
meson setup --buildtype=release buildDir --reconfigure
```

To switch to debug mode:

```
meson setup --buildtype=debug buildDir --reconfigure
```
