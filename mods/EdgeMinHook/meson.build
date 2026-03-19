project('EdgeMinHook', 'c', default_options: ['c_std=c17', 'cpp_std=vc++20'], meson_version: '>=1.3')

c = meson.get_compiler('c')
bt = get_option('buildtype')
vcpkg_triplet = get_option('vcpkg_triplet')

if bt == 'debug'
    add_project_arguments('/D_DEBUG', language: ['c', 'cpp'])
    add_project_link_arguments('/incremental', language: ['c', 'cpp'])
    vcpkg_libdir = meson.current_source_dir() / 'vcpkg_installed' / vcpkg_triplet / 'debug' / 'lib'
    d = 'd'
    vscrt = 'mtd'
elif bt == 'release'
    add_project_arguments('/DNDEBUG', '-flto=thin', language: ['c', 'cpp'])
    add_project_link_arguments('/safeseh', '/opt:icf', language: ['c', 'cpp'])
    vcpkg_libdir = meson.current_source_dir() / 'vcpkg_installed' / vcpkg_triplet / 'lib'
    d = ''
    vscrt = 'mt'
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

windows = import('windows')

resource = windows.compile_resources('src/Resource.rc')

dll = shared_library('EdgeMinHook', [src, resource], include_directories: inc, dependencies: lib, win_subsystem: 'windows', override_options: ['b_vscrt=' + vscrt])

edgeminhook_dep = declare_dependency(include_directories: [edgeminhook_incdir], link_with: [dll])
