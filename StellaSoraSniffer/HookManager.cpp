#include "pch.h"
#include "HookManager.h"
#include "Hooks.h"
#include "PrintHelper.h"
#include <detours.h>

const uintptr_t RVA_readMessage = 0x12F2140;
const uintptr_t RVA_BuildMessage = 0x12EE290;

void InstallHooks() {
    auto base = (uintptr_t)GetModuleHandleA("GameAssembly.dll");

    o_readMessage = (void*)(base + RVA_readMessage);
    o_BuildMessage = (void*)(base + RVA_BuildMessage);

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());

    DetourAttach(&(PVOID&)o_readMessage, readMessage_Hook);
    DetourAttach(&(PVOID&)o_BuildMessage, BuildMessage_Hook);

    DetourTransactionCommit();

    DebugPrintLockA("[INFO] Hooks installed\n");
}

void UninstallHooks() {
    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());

	// here need uninstall hooks

    DetourTransactionCommit();
}

DWORD WINAPI WaitForGAModule(LPVOID) {
    DebugPrintLockA("[INFO] Waiting for GameAssembly.dll...\n");
    while (!GetModuleHandleA("GameAssembly.dll")) Sleep(200);

    DebugPrintLockA("[INFO] GameAssembly.dll loaded, installing hooks...\n");
    InstallHooks();
    return 0;
}
