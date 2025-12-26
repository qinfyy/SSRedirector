#include "pch.h"
#include "HookManager.h"
#include "Hooks.h"
#include "PrintHelper.h"
#include <detours.h>
#include "Memory.h"

void InstallHooks() {
    auto base = GetModuleHandleA("GameAssembly.dll");

    uintptr_t uw2Addr = Scan(base, "48 89 5C 24 ?? 48 89 74 24 ?? 57 48 83 EC 20 48 8B F2 49 8B F8 33 D2 48 8B D9 E8 ?? ?? ?? ?? 48 8B 05 ?? ?? ?? ??");
    uintptr_t uw3Addr = Scan(base, "48 89 5C 24 ?? 48 89 6C 24 ?? 48 89 74 24 ?? 57 48 83 EC ?? 48 8B EA 49 8B F9 33 D2 49 8B F0 48 8B D9 E8 ?? ?? ?? ?? 48 8B 05 ?? ?? ?? ??");
    
    if (!uw2Addr) {
        DebugPrintA("[ERROR] Failed to find u1.\n");
        return;
    }

    if (!uw3Addr) {
        DebugPrintA("[ERROR] Failed to find u2.\n");
        return;
    }

    DebugPrintA("[INFO] uw2: 0x%lX\n", uw2Addr);
    DebugPrintA("[INFO] uw3: 0x%lX\n", uw3Addr);

    o_uw2 = (PVOID)uw2Addr;
    o_uw3 = (PVOID)uw3Addr;

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());

    DetourAttach(&o_uw2, uw2_Hook);
    DetourAttach(&o_uw3, uw3_Hook);

    DetourTransactionCommit();

    DebugPrintLockA("[INFO] Hooks installed\n");
}

void UninstallHooks() {
    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());

    // here need uninstall hooks

    DetourTransactionCommit();
}

void InitServerIP() {
    std::wstring configFilePath = L".\\Config.ini";
    std::wstring section = L"SSRedirector, Made by Cyt";
    std::wstring key = L"ServerIP";
    std::wstring defaultIP = L"http://160.202.238.172:1146";

    wchar_t buffer[512] = { 0 };
	DWORD bufferSize = sizeof(buffer) / sizeof(wchar_t);
    DWORD readLen = GetPrivateProfileString(section.c_str(), key.c_str(), L"", buffer, bufferSize, configFilePath.c_str());

    if (readLen == 0 || buffer[0] == L'\0') {
        g_ServerIP = defaultIP;
        WritePrivateProfileString(section.c_str(), key.c_str(), g_ServerIP.c_str(), configFilePath.c_str());
    } else {
        g_ServerIP = buffer;
    }
}

DWORD WINAPI WaitForGAModule(LPVOID) {
    DebugPrintLockA("[INFO] SSRedirector, Made by Cyt\n");
    DebugPrintLockA("[INFO] Waiting for GameAssembly.dll...\n");
    while (!GetModuleHandleA("GameAssembly.dll")) Sleep(200);

    DebugPrintLockA("[INFO] GameAssembly.dll loaded, installing hooks...\n");
    InitServerIP();
    InstallHooks();
    return 0;
}
