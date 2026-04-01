includes("../include.lua")
add_rules("mode.debug", "mode.release")
set_defaultmode("debug")
set_plat("windows")
set_arch("x86")
set_toolchains("clang-cl-asi")
set_languages("c17", "cxx23")
set_config("asiextension", false)

if is_mode("debug") then
    set_runtimes("MTd")
elseif is_mode("release") then
    set_runtimes("MT")
end

add_requires("minhook", {plat = "windows", arch = "x86"})

target("EdgeMinHook")
    add_rules("asi")
    add_files("src/dllmain.c", "src/Resource.rc")
    add_includedirs("include")
    add_headerfiles("include/EdgeMinHook.h")
    add_defines("EDGEMINHOOK_EXPORTS")
    add_packages("minhook")
