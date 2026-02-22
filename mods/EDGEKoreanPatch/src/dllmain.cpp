#include <windows.h>
#include <MinHook.h>

typedef void (*FUN_004a5bb0_t)();
typedef unsigned short* (__thiscall*FUN_004a7120_t)(void *me, wchar_t *value);

FUN_004a5bb0_t org4a5bb0 = nullptr;

void FUN_004a5bb0() {
    (*org4a5bb0)();

    unsigned short order = 136;
    const auto unknown_type_instance = reinterpret_cast<void*>(0x5f9210);
    const auto FUN_004a7120 = reinterpret_cast<FUN_004a7120_t>(0x4a7120);

    for (auto character :
         L"가간감갖개게경계공과금기끔나너녕노높뉴느는니님다더도동됩뒤드든디딩때래랭레려력로록료를리릭림마막만많머멀메며면명모문뮤받방버벨보북브쁘사새서설세셔셨소속수순쉽스습시십아악안않야어에예오와완요용운원위으은을음이익인임입있자작잘잠재저전점접정제좋주죽중즘지직차체총취커켬코큐크클키킹타터텐통트튼티포표프플하할합해했향화환회효희히") {
        wchar_t *pCharacter = &character;
        unsigned short *pOrder = FUN_004a7120(unknown_type_instance, pCharacter);
        *pOrder = ++order;
    }
}

BOOL APIENTRY DllMain(HMODULE, DWORD ul_reason_for_call, LPVOID) {
    if (ul_reason_for_call == DLL_PROCESS_ATTACH) {
        if (MH_Initialize() != MH_OK) {
            return FALSE;
        }

        if (MH_CreateHook(reinterpret_cast<LPVOID>(0x4a5bb0), reinterpret_cast<LPVOID>(&FUN_004a5bb0),
            reinterpret_cast<LPVOID*>(&org4a5bb0)) != MH_OK) {
            return FALSE;
        }

        if (MH_EnableHook(reinterpret_cast<LPVOID>(0x4a5bb0))) {
            return FALSE;
        }
    }

    if (ul_reason_for_call == DLL_PROCESS_DETACH) {
        // ReSharper disable once CppZeroConstantCanBeReplacedWithNullptr
        if (MH_DisableHook(MH_ALL_HOOKS) != MH_OK || MH_Uninitialize() != MH_OK) {
            return FALSE;
        }
    }

    return TRUE;
}
