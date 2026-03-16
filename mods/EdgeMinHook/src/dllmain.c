#include <windows.h>
#include <MinHook.h>
#include "EdgeMinHook.h"

HANDLE g_hHookEvent = NULL;

void RegisterEdgeHook(void *target, void *detour, void **original) {
    if (MH_CreateHook(target, detour, original) == MH_OK) {
        if (g_hHookEvent != NULL) {
            SetEvent(g_hHookEvent);
        }
    }
}

DWORD WINAPI EnableHooksThread(LPVOID lpParam) {
    // ReSharper disable once CppDFAEndlessLoop
    while (TRUE) {
        if (WaitForSingleObject(g_hHookEvent, INFINITE) != WAIT_OBJECT_0) {
            ExitThread(1);
        }

        Sleep(100);

        if (MH_EnableHook(MH_ALL_HOOKS) != MH_OK) {
            MessageBoxW(NULL, L"Failed to enable hooks", NULL, MB_OK);
        }
    }
}

BOOL APIENTRY DllMain(HMODULE hModule, DWORD ul_reason_for_call, LPVOID lpReserved) {
    switch (ul_reason_for_call) {
        case DLL_PROCESS_ATTACH:
            DisableThreadLibraryCalls(hModule);

            if (MH_Initialize() != MH_OK) {
                MessageBoxW(NULL, L"Failed to initialize EdgeMinHook", NULL, MB_OK);

                return FALSE;
            }

            g_hHookEvent = CreateEventW(NULL, FALSE, FALSE, NULL);

            if (g_hHookEvent == NULL) {
                return FALSE;
            }

            HANDLE g_hThread = CreateThread(NULL, 0, EnableHooksThread, NULL, 0, NULL);

            if (g_hThread == NULL) {
                return FALSE;
            }

            CloseHandle(g_hThread);

            break;

        case DLL_PROCESS_DETACH:
            if (lpReserved != NULL) {
                if (MH_Uninitialize() != MH_OK) {
                    MessageBoxW(NULL, L"Failed to uninitialize EdgeMinHook", NULL, MB_OK);

                    return FALSE;
                }

                if (g_hHookEvent != NULL) {
                    if (CloseHandle(g_hHookEvent) == FALSE) {
                        return FALSE;
                    }
                }
            }

            break;

        default:
            break; // Explicit break
    }

    return TRUE;
}
