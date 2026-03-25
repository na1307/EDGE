#include <windows.h>
#include <EdgeMinHook.h>
#include <filesystem>

typedef void (__thiscall*sub_474160_t)(void *, int);
typedef void (*sub_4A63F0_t)();
typedef void (__cdecl*sub_4A75F0_t)(int);

auto sub_473e60 = reinterpret_cast<const char*(__cdecl*)(int)>(0x473e60);
sub_474160_t org474160 = nullptr;
sub_4A63F0_t org4A63F0 = nullptr;
sub_4A75F0_t org4A75F0 = nullptr;

void __thiscall sub_474160(void *thisptr, int music) {
    const auto music_name = sub_473e60(music);
    const auto music_path = new std::string("mods/music/");

    music_path->append(music_name);
    music_path->append(".ogg");

    if (std::filesystem::exists(*music_path) && std::filesystem::is_regular_file(*music_path)) {
        const auto ptr = *(static_cast<void**>(thisptr) + 2);
        const auto music_function = reinterpret_cast<bool (__thiscall*)(void *, std::string *, bool)>(*(*static_cast<int**>(ptr) + 1));

        music_function(ptr, music_path, true);
    } else {
        org474160(thisptr, music);
    }
}

void sub_4A63F0() {
    org4A63F0();
}

void __cdecl sub_4A75F0(int a1) {
    org4A75F0(a1);
}

BOOL APIENTRY DllMain(HMODULE hModule, DWORD ul_reason_for_call, LPVOID) {
    if (ul_reason_for_call == DLL_PROCESS_ATTACH) {
        DisableThreadLibraryCalls(hModule);
        RegisterEdgeHook(reinterpret_cast<void*>(0x474160), reinterpret_cast<void*>(sub_474160), reinterpret_cast<void**>(&org474160));
        RegisterEdgeHook(reinterpret_cast<void*>(0x4a63f0), reinterpret_cast<void*>(sub_4A63F0), reinterpret_cast<void**>(&org4A63F0));
        RegisterEdgeHook(reinterpret_cast<void*>(0x4a75f0), reinterpret_cast<void*>(sub_4A75F0), reinterpret_cast<void**>(&org4A75F0));
    }

    return TRUE;
}
