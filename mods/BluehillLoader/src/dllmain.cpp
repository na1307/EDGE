// ReSharper disable CppParameterMayBeConst
#include <windows.h>
import global;
import functions;
import bonus;
import level;
import music;
import fontbin;
import textloc;
import EdgeMinHook;

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
        RegisterEdgeHook(reinterpret_cast<void*>(0x474fa0), reinterpret_cast<void*>(sub_474FA0), reinterpret_cast<void**>(&org474FA0));
        RegisterEdgeHook(reinterpret_cast<void*>(0x4947e0), reinterpret_cast<void*>(sub_4947E0), reinterpret_cast<void**>(&org4947E0));
        RegisterEdgeHook(reinterpret_cast<void*>(0x4a63f0), reinterpret_cast<void*>(sub_4A63F0), reinterpret_cast<void**>(&org4A63F0));
        RegisterEdgeHook(reinterpret_cast<void*>(0x4a75f0), reinterpret_cast<void*>(sub_4A75F0), reinterpret_cast<void**>(&org4A75F0));
    }

    return TRUE;
}
