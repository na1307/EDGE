includes("../include.lua")
add_rules("mode.debug", "mode.release")
set_defaultmode("debug")
set_plat("windows")
set_arch("x86")
set_toolchains("clang-cl-asi")
set_languages("c17", "cxx23")

set_runtimes("MT")

add_repositories("EdgeMinHook ../EdgeMinHook/build")
add_requires("edgeminhook", {plat = "windows", arch = "x86"})

target("Bluehill's Loader")
    add_rules("asi")
    add_files("src/dllmain.cpp")
    add_packages("edgeminhook")
