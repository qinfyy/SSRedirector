#pragma once
#include "Util.h"

inline PVOID o_uw2 = nullptr;
inline PVOID o_uw3 = nullptr;

inline std::wstring g_ServerIP;

void uw2_Hook(
    void* tp,
    Il2CppString* url,
    void* sm,
    void* m);

void uw3_Hook(
    void* tp,
    Il2CppString* url,
    void* sm,
    void* dh,
    void* uh,
    void* m);
