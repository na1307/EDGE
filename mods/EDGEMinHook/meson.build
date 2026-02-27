project('EDGEMinHook', 'c', default_options: ['c_std=c17', 'cpp_std=vc++20'], meson_version: '>=1.3')

c = meson.get_compiler('c')
c_id = c.get_id()

bt = get_option('buildtype')
vcpkg_triplet = get_option('vcpkg_triplet')

if bt == 'debug'
    add_project_arguments('/D_DEBUG', language: ['c', 'cpp'])
    add_project_link_arguments('/incremental', language: ['c', 'cpp'])
    vcpkg_libdir = meson.current_source_dir() / 'vcpkg_installed' / vcpkg_triplet / 'debug' / 'lib'
    d = 'd'
elif bt == 'release'
    add_project_arguments('/DNDEBUG', language: ['c', 'cpp'])
    add_project_link_arguments('/safeseh', '/opt:icf', language: ['c', 'cpp'])

    if c_id == 'msvc'
        add_project_arguments('/GL', language: ['c', 'cpp'])
        add_project_link_arguments('/LTCG:incremental', language: ['c', 'cpp'])
    elif c_id == 'clang-cl'
        add_project_arguments('-flto=thin', language: ['c', 'cpp'])
    endif

    vcpkg_libdir = meson.current_source_dir() / 'vcpkg_installed' / vcpkg_triplet / 'lib'
    d = ''
endif

add_project_arguments('/DWIN32', '/DEDGEMINHOOK_EXPORTS', '/D_WINDOWS', '/D_USRDLL', '/D_WINDLL', '/D_UNICODE', '/DUNICODE', '/Gd', language: ['c', 'cpp'])
add_project_link_arguments('/subsystem:windows', language: ['c', 'cpp'])

src = [
    'src/dllmain.c',
]

vcpkg_incdir = include_directories('vcpkg_installed' / vcpkg_triplet / 'include')
edgeminhook_incdir = include_directories('include')

minhook = c.find_library('minhook.x32' + d, dirs: [vcpkg_libdir], static: true)

inc = [vcpkg_incdir, edgeminhook_incdir]
lib = [minhook]

dll = shared_library('EDGEMinHook', src, include_directories: inc, dependencies: lib, win_subsystem: 'windows')

edgeminhook_dep = declare_dependency(include_directories: [edgeminhook_incdir], link_with: [dll])
