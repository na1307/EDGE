toolchain("clang-cl-dependencies")
    set_kind("standalone")

    set_toolset("cc", "clang-cl")
    set_toolset("cxx", "clang-cl")
    set_toolset("ld", "lld-link")
    set_toolset("sh", "lld-link")
    set_toolset("ar", "llvm-ar")
    set_toolset("mrc", "llvm-rc")
    set_toolset("dlltool", "llvm-dlltool")
    set_toolset("as", "llvm-ml")

    on_check(function (toolchain)
        return import("lib.detect.find_tool")("clang-cl")
    end)

    on_load(function (toolchain)
        toolchain:add("cxflags", "-m32", "--target=i686-pc-windows-msvc")
        toolchain:add("mxflags", "-m32", "--target=i686-pc-windows-msvc")
        if is_mode("release") then
            toolchain:add("cxflags", "-flto=thin")
        end
    end)

rule("edgeplugin")
    on_load(function (target)
        target:set("kind", "shared")
        if is_mode("debug") and not has_config("nomtd") then
            target:set("runtimes", "MTd")
        else
            target:set("runtimes", "MT")
        end
        target:set("policy", "build.c++.modules", true)
        target:set("toolset", "mrc", "llvm-rc")
        target:set("toolset", "ld", "lld-link")
        target:set("toolset", "sh", "lld-link")
        target:set("toolset", "ar", "llvm-ar")
        target:set("toolset", "as", "llvm-ml")
        target:add("shflags", "-machine:x86", {force = true})
    end)

    after_load(function (target)
        if not target:values("windows.subsystem") then
            target:values_set("windows.subsystem", "windows")
        end

        target:add("defines", "WIN32", "_WINDOWS", "_USRDLL", "_WINDLL", "_UNICODE", "UNICODE")
        target:add("cxflags", "-Gd")
        if is_mode("debug") then
            if not has_config("nomtd") then
                target:add("defines", "DEBUG", "_DEBUG")
            else
                target:add("defines", "NDEBUG")
            end
        elseif is_mode("release") then
            target:add("cxflags", "-Gw")
        end

        target:add("syslinks", "kernel32", "user32", "gdi32", "winspool", "comdlg32", "advapi32")
        target:add("syslinks", "shell32", "ole32", "oleaut32", "uuid", "odbc32", "odbccp32", "comctl32")
        target:add("syslinks", "comdlg32", "setupapi", "shlwapi")
        if not target:is_plat("mingw") then
            target:add("syslinks", "strsafe")
        end
    end)
