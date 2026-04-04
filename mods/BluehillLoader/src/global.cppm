export module global;

import std;

export typedef void* (__cdecl *malloc_t)(size_t);
export malloc_t malloc_msvcr100 = nullptr;
export std::string mods_root("mods/");
