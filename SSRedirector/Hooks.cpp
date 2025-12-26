#include "pch.h"
#include "Hooks.h"
#include "PrintHelper.h"
#include <chrono>
#include <iostream>
#include <string>

bool ProcessIp(std::wstring& oUrl, std::wstring& nUrl) {
    if (oUrl.find(L"yostarplat.com") != std::wstring::npos || oUrl.find(L"stellasora.global") != std::wstring::npos ||
        oUrl.find(L"stellasora.kr") != std::wstring::npos || oUrl.find(L"stellasora.jp") != std::wstring::npos ||
        oUrl.find(L"stargazer-games.com") != std::wstring::npos || oUrl.find(L"yostar.cn") != std::wstring::npos)
    {
        nUrl = g_ServerIP;
        size_t pos = oUrl.find(L'/', oUrl.find(L"://") + 3);
        if (pos != std::wstring::npos) {
            nUrl += oUrl.substr(pos);
        }
        return true;
    }
    return false;
}

void uw2_Hook(
    void* tp,
    Il2CppString* url,
    void* sm,
    void* m) {
    std::wstring oUrl = Il2cppToWstring(url);

    std::wstring nUrl;

    if (ProcessIp(oUrl, nUrl)) {
        DebugPrintLockA("[uw2] %ls -> %ls\n", oUrl.c_str(), nUrl.c_str());
        url = CreateIl2CppString(nUrl, url);
    }

    return ((void (*)(void*, Il2CppString*, void*, void*))o_uw2)(tp, url, sm, m);
}

void uw3_Hook(
    void* tp,
    Il2CppString* url,
    void* sm,
    void* dh,
    void* uh,
    void* m) {
    std::wstring oUrl = Il2cppToWstring(url);

    std::wstring nUrl;

    if (ProcessIp(oUrl, nUrl)) {
        DebugPrintLockA("[uw3] %ls -> %ls\n", oUrl.c_str(), nUrl.c_str());
        url = CreateIl2CppString(nUrl, url);
    }

    return ((void (*)(void*, Il2CppString*, void*, void*, void*, void*))o_uw3)(tp, url, sm, dh, uh, m);
}
