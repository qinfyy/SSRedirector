#include "pch.h"
#include "Util.h"
#include <string>
#include <codecvt>
#include <iomanip>

std::wstring Il2cppToWstring(Il2CppString* str) {
    if (!str || str->length <= 0)
        return {};

    return std::wstring(str->chars, str->length);
}

Il2CppString* CreateIl2CppString(const std::wstring& ws, Il2CppString* original)
{
    if (!original) return nullptr;

    int32_t len = static_cast<int32_t>(ws.size());

    size_t size = sizeof(Il2CppString);
    if (len > 32)
        size += (len - 32) * sizeof(wchar_t);

    Il2CppString* newStr = (Il2CppString*)malloc(size);
    if (!newStr) return nullptr;

    newStr->m_pClass = original->m_pClass;
    newStr->monitor = nullptr;
    newStr->length = len;

    memcpy(newStr->chars, ws.data(), len * sizeof(wchar_t));

    return newStr;
}

bool ReplaceIl2CppStringChars(Il2CppString* target, const std::wstring& ws)
{
    if (!target)
        return false;

    const size_t capacity = static_cast<size_t>(target->length);

    if (ws.size() > capacity)
        return false;

    std::memcpy(target->chars, ws.data(), ws.size() * sizeof(wchar_t));

    if (ws.size() < capacity) {
        std::memset(target->chars + ws.size(), 0, (capacity - ws.size()) * sizeof(wchar_t));
    }

    target->length = static_cast<int32_t>(ws.size());
    return true;
}
