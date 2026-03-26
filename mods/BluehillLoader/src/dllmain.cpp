// ReSharper disable CppTooWideScopeInitStatement
#include <windows.h>
#include <EdgeMinHook.h>
#include <filesystem>

typedef void* (__cdecl *malloc_t)(size_t);
typedef void (__thiscall *sub_474160_t)(void *, int);
typedef void (*sub_4A63F0_t)();
typedef void (__cdecl *sub_4A75F0_t)(int);

constexpr char mods_font_bin[] = "mods/font.bin";
std::string mods_root("mods/");
malloc_t malloc_msvcr100 = nullptr;
auto sub_44A490 = reinterpret_cast<bool (__cdecl *)(std::string *, const char *)>(0x44a490);
auto sub_473E60 = reinterpret_cast<const char*(__cdecl *)(int)>(0x473e60);
sub_474160_t org474160 = nullptr;
auto sub_49BBF0 = reinterpret_cast<const char*(__cdecl *)(int)>(0x49bbf0);
sub_4A63F0_t org4A63F0 = nullptr;
sub_4A75F0_t org4A75F0 = nullptr;
auto sub_4F17C0 = reinterpret_cast<bool (__thiscall *)(void *, std::string *)>(0x4f17c0);
auto sub_4F21B0 = reinterpret_cast<short (__thiscall *)(void *, std::string *)>(0x4f21b0);
auto sub_4F2760 = reinterpret_cast<void *(__thiscall *)(void *, std::string *, std::string *, bool)>(0x4f2760);

inline bool file_exists(const std::string &path) {
    return std::filesystem::exists(path) && std::filesystem::is_regular_file(path);
}

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

void __cdecl sub_4A75F0(int lang_code) {
    auto text_loc_path = mods_root;

    text_loc_path.append("localization/text.loc");

    if (file_exists(text_loc_path)) {
        if (!sub_44A490(&text_loc_path, nullptr)) {
            org4A75F0(lang_code);

            return;
        }

        *reinterpret_cast<int*>(0x5f426c) = lang_code;
        std::string lang_name(sub_49BBF0(lang_code));
        const auto ptr = malloc_msvcr100(84);

        if (ptr != nullptr) {
            *reinterpret_cast<void**>(0x5f9220) = sub_4F2760(ptr, &text_loc_path, &lang_name, false);
        }

        if (!sub_4F17C0(ptr, &lang_name)) {
            std::string en("en");

            sub_4F21B0(ptr, &en);
        }
    } else {
        org4A75F0(lang_code);
    }
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
        RegisterEdgeHook(reinterpret_cast<void*>(0x4a63f0), reinterpret_cast<void*>(sub_4A63F0), reinterpret_cast<void**>(&org4A63F0));
        RegisterEdgeHook(reinterpret_cast<void*>(0x4a75f0), reinterpret_cast<void*>(sub_4A75F0), reinterpret_cast<void**>(&org4A75F0));
    }

    return TRUE;
}
