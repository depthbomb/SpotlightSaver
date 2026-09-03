#pragma once

#include <utility>
#include <filesystem>

std::pair<int, int> get_image_resolution(const std::filesystem::path& path);
