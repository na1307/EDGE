// ReSharper disable CppTooWideScopeInitStatement
// ReSharper disable CppParameterMayBeConst
#include <windows.h>
#include <EdgeMinHook.h>
#include <filesystem>

typedef void* (__cdecl *malloc_t)(size_t);
typedef void (__thiscall *sub_474160_t)(void *, int);
typedef int (*sub_4947E0_t)();
typedef void (*sub_4A63F0_t)();
typedef void (__cdecl *sub_4A75F0_t)(int);

std::string mods_root("mods/");
constexpr char mods_font_bin[] = "mods/font.bin";
constexpr char mods_localization_text_loc[] = "mods/localization/text.loc";
malloc_t malloc_msvcr100 = nullptr;
auto sub_473E60 = reinterpret_cast<const char* (__cdecl *)(int)>(0x473e60);
sub_474160_t org474160 = nullptr;
sub_4947E0_t org4947E0 = nullptr;
sub_4A63F0_t org4A63F0 = nullptr;
sub_4A75F0_t org4A75F0 = nullptr;

inline bool file_exists(const std::string &path) {
    return std::filesystem::exists(path) && std::filesystem::is_regular_file(path);
}

// Replace a music to mods
void __thiscall sub_474160(void *thisptr, int music) {
    const auto music_name = sub_473E60(music);
    auto music_path = mods_root;

    music_path.append("music/");
    music_path.append(music_name);
    music_path.append(".ogg");

    if (file_exists(music_path)) {
        const auto ptr = *(static_cast<void**>(thisptr) + 2);
        const auto music_function = reinterpret_cast<bool (__thiscall *)(void *, std::string *, bool)>(*(*static_cast<int**>(ptr) + 1));

        music_function(ptr, &music_path, true);
    } else {
        org474160(thisptr, music);
    }
}

// Bypass Bonus levels
int sub_4947E0() {
    const auto value = org4947E0();
    *(*reinterpret_cast<bool**>(0x5f9080) + 724) = true;

    return value;
}

// Replace font.bin to mods
void sub_4A63F0() {
    static bool is_patched = false;

    if (!is_patched && file_exists(mods_font_bin)) {
        const auto target_start = reinterpret_cast<LPVOID>(0x4a6402);
        DWORD old_protect;

        if (VirtualProtect(target_start, 10, PAGE_EXECUTE_READWRITE, &old_protect) != 0) {
            constexpr uintptr_t size_addr = 0x4a6403;
            constexpr uintptr_t string_addr = 0x4a6407;
            *reinterpret_cast<unsigned char*>(size_addr) = static_cast<unsigned char>(sizeof(mods_font_bin) - 1);
            *reinterpret_cast<const char**>(string_addr) = mods_font_bin;

            VirtualProtect(target_start, 10, old_protect, &old_protect);

            is_patched = true;
        }
    }

    org4A63F0();
}

// Replace text.loc to mods
void __cdecl sub_4A75F0(int lang_code) {
    static bool is_patched = false;

    if (!is_patched && file_exists(mods_localization_text_loc)) {
        const auto target_start = reinterpret_cast<LPVOID>(0x4a7606);
        DWORD old_protect;

        if (VirtualProtect(target_start, 10, PAGE_EXECUTE_READWRITE, &old_protect) != 0) {
            constexpr uintptr_t size_addr = 0x4a7608;
            constexpr uintptr_t string_addr = 0x4a760a;
            *reinterpret_cast<unsigned char*>(size_addr) = static_cast<unsigned char>(sizeof(mods_localization_text_loc) - 1);
            *reinterpret_cast<const char**>(string_addr) = mods_localization_text_loc;

            VirtualProtect(target_start, 10, old_protect, &old_protect);

            is_patched = true;
        }
    }

    org4A75F0(lang_code);
}

inline void initialize_failed() {
    MessageBoxW(nullptr, L"Failed to initialize Bluehill's Loader.", nullptr, MB_OK);
}

BOOL APIENTRY DllMain(HMODULE hModule, DWORD ul_reason_for_call, LPVOID) {
    if (ul_reason_for_call == DLL_PROCESS_ATTACH) {
        const auto msvcr100 = GetModuleHandleW(L"msvcr100.dll");

        if (msvcr100 == nullptr) {
            initialize_failed();

            return FALSE;
        }

        const auto malloc = reinterpret_cast<malloc_t>(GetProcAddress(msvcr100, "malloc"));

        if (malloc == nullptr) {
            initialize_failed();

            return FALSE;
        }

        malloc_msvcr100 = malloc;

        DisableThreadLibraryCalls(hModule);
        RegisterEdgeHook(reinterpret_cast<void*>(0x474160), reinterpret_cast<void*>(sub_474160), reinterpret_cast<void**>(&org474160));
        RegisterEdgeHook(reinterpret_cast<void*>(0x4947e0), reinterpret_cast<void*>(sub_4947E0), reinterpret_cast<void**>(&org4947E0));
        RegisterEdgeHook(reinterpret_cast<void*>(0x4a63f0), reinterpret_cast<void*>(sub_4A63F0), reinterpret_cast<void**>(&org4A63F0));
        RegisterEdgeHook(reinterpret_cast<void*>(0x4a75f0), reinterpret_cast<void*>(sub_4A75F0), reinterpret_cast<void**>(&org4A75F0));
    }

    return TRUE;
}
