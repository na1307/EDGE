#pragma once

#ifndef EDGEMINHOOK_EDGEMINHOOK_H
#define EDGEMINHOOK_EDGEMINHOOK_H

#ifdef EDGEMINHOOK_EXPORTS
#define EDGEMINHOOK_API __declspec(dllexport)
#else //EDGEMINHOOK_EXPORTS
#define EDGEMINHOOK_API __declspec(dllimport)
#endif //EDGEMINHOOK_EXPORTS

#ifdef __cplusplus
extern "C" {

#endif //__cplusplus

EDGEMINHOOK_API void RegisterEdgeHook(void *target, void *detour, void **original);

#ifdef __cplusplus
}
#endif //__cplusplus

#endif //EDGEMINHOOK_EDGEMINHOOK_H
