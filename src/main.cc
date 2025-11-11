#define WIN32_LEAN_AND_MEAN
#define NOCOMM

#include <string>
#include <fstream>
#include <cstdint>
#include <iostream>
#include <filesystem>
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

std::pair<int, int> get_image_resolution(const fs::path& path) {
    // We only check a specific resolution so it's fine to return 0,0 in the case of errors since we will skip them
    // anyway.
    unsigned int width = 0, height = 0;

    std::ifstream in(path, std::ios::binary);

    uint8_t byte1, byte2;

    in.read(reinterpret_cast<char*>(&byte1), 1);
    in.read(reinterpret_cast<char*>(&byte2), 1);
    if (byte1 != 0xFF || byte2 != 0xD8) {
        // Probably not a JPEG file
        return std::make_pair(width, height);
    }

    while (in) {
        in.read(reinterpret_cast<char*>(&byte1), 1);
        if (byte1 != 0xFF) {
            continue;
        }

        in.read(reinterpret_cast<char*>(&byte2), 1);

        if (byte2 == 0xC0 || byte2 == 0xC2) {
            in.ignore(3);

            uint8_t heightBytes[2], widthBytes[2];

            in.read(reinterpret_cast<char*>(heightBytes), 2);
            in.read(reinterpret_cast<char*>(widthBytes), 2);

            height = heightBytes[0] << 8 | heightBytes[1];
            width  = widthBytes[0] << 8 | widthBytes[1];
        } else {
            uint8_t lengthBytes[2];

            in.read(reinterpret_cast<char*>(lengthBytes), 2);

            const int length = lengthBytes[0] << 8 | lengthBytes[1];

            in.ignore(length - 2);
        }
    }

    return std::make_pair(width, height);
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

        if (auto [width, height] = get_image_resolution(entry.path()); width != 1920 && height != 1080) {
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
