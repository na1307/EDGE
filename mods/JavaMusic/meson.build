project('JavaMusic', 'cpp', default_options: ['c_std=c17', 'cpp_std=vc++20'], meson_version: '>=1.3')

cpp = meson.get_compiler('cpp')
bt = get_option('buildtype')

if bt == 'debug'
    add_project_arguments('/D_DEBUG', language: ['c', 'cpp'])
    add_project_link_arguments('/incremental', language: ['c', 'cpp'])
    vscrt = 'mtd'
elif bt == 'release'
    add_project_arguments('/DNDEBUG', '-flto=thin', language: ['c', 'cpp'])
    add_project_link_arguments('/safeseh', '/opt:icf', language: ['c', 'cpp'])
    vscrt = 'mt'
endif

add_project_arguments('/DWIN32', '/DJAVAMUSIC_EXPORTS', '/D_WINDOWS', '/D_USRDLL', '/D_WINDLL', '/D_UNICODE', '/DUNICODE', '/Gd', language: ['c', 'cpp'])
add_project_link_arguments('/subsystem:windows', language: ['c', 'cpp'])

src = [
    'src/dllmain.cpp',
    'src/MidiPlayer.cpp',
]

edgeminhook = dependency('EdgeMinHook', default_options: ['vcpkg_triplet=' + get_option('vcpkg_triplet'), 'b_vscrt=' + vscrt], fallback: ['EdgeMinHook', 'edgeminhook_dep'])

deps = [edgeminhook]

shared_library('JavaMusic', src, dependencies: deps, name_suffix: 'asi', win_subsystem: 'windows', override_options: ['b_vscrt=' + vscrt])
