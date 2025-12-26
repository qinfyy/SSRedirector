#include "pch.h"
#include "Hooks.h"
#include "PrintHelper.h"
#include <chrono>
#include <iostream>
#include <string>
#include <thread>

bool ProcessIp(std::wstring& oUrl, std::wstring& nUrl) {
    std::wstring configFilePath = L".\\Config.ini";
    std::wstring section = L"SSRedirector, Made by Cyt";
    std::wstring key = L"ServerIP";

    std::wstring defaultIP = L"http://160.202.238.172:1146";

    wchar_t ipBuffer[512] = { 0 };
    DWORD readLen = GetPrivateProfileString(section.c_str(), key.c_str(), L"", ipBuffer, sizeof(ipBuffer) / sizeof(wchar_t), configFilePath.c_str());

    if (readLen == 0 || ipBuffer[0] == L'\0') {
        wcscpy_s(ipBuffer, defaultIP.c_str());

        std::wstring sectionCopy = section;
        std::wstring keyCopy = key;
        std::wstring valueCopy = ipBuffer;
        std::wstring pathCopy = configFilePath;

        std::thread([sectionCopy, keyCopy, valueCopy, pathCopy]() {
            WritePrivateProfileString(sectionCopy.c_str(), keyCopy.c_str(), valueCopy.c_str(), pathCopy.c_str());
        }).detach();
    }

    if (oUrl.find(L"yostarplat.com") != std::wstring::npos || oUrl.find(L"stellasora.global") != std::wstring::npos ||
        oUrl.find(L"stellasora.kr") != std::wstring::npos || oUrl.find(L"stellasora.jp") != std::wstring::npos ||
        oUrl.find(L"stargazer-games.com") != std::wstring::npos || oUrl.find(L"yostar.cn") != std::wstring::npos)
    {
        nUrl = ipBuffer;
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
