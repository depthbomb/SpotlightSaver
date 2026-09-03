#define WIN32_LEAN_AND_MEAN
#define NOCOMM

#include "jpeg_dimensions.h"
#include "spotlight_image.h"

#include <iostream>
#include <filesystem>
#include <stdexcept>
#include <windows.h>
#include <ShlObj.h>

namespace fs = std::filesystem;

fs::path get_spotlight_assets_dir() {
    PWSTR path = nullptr;
    if (SUCCEEDED(SHGetKnownFolderPath(FOLDERID_LocalAppData, 0, nullptr, &path))) {
        const fs::path result = path;
        CoTaskMemFree(path);

        return result / "Packages" / "Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy" / "LocalState" / "Assets";
    }

    throw std::runtime_error("Failed to get LOCALAPPDATA path");
}

fs::path get_destination_dir() {
    PWSTR path = nullptr;
    if (SUCCEEDED(SHGetKnownFolderPath(FOLDERID_Pictures, 0, nullptr, &path))) {
        fs::path result = path;
        CoTaskMemFree(path);

        result = result / "Windows Spotlight";
        return result;
    }

    throw std::runtime_error("Failed to get Pictures path");
}

int main() {
    const auto assets_dir = get_spotlight_assets_dir();
    if (!fs::exists(assets_dir)) {
        std::cerr << "Could not find assets path: " << assets_dir << std::endl;
        return 1;
    }

    const auto destination_dir = get_destination_dir();
    if (!fs::exists(destination_dir)) {
        fs::create_directories(destination_dir);
    }

    int saved_count = 0;

    for (const auto& entry : fs::directory_iterator(assets_dir)) {
        if (!fs::is_regular_file(entry)) {
            continue;
        }

        if (!is_spotlight_image_resolution(get_image_resolution(entry.path()))) {
            continue;
        }

        auto destination_path = destination_dir / (entry.path().filename().string() + ".jpg");
        if (fs::exists(destination_path)) {
            std::cout << "Image " << entry.path() << " has already been saved" << std::endl;
            continue;
        }

        try {
            fs::copy(entry.path(), destination_path);
            std::cout << "Copied " << entry.path() << " to " << destination_path << std::endl;
            saved_count++;
        } catch (std::filesystem::filesystem_error& e) {
            std::cerr << e.what() << std::endl;
        }
    }

    if (saved_count > 0) {
        std::cout << "Saved " << saved_count << " image(s)" << std::endl;
    }

    return 0;
}
