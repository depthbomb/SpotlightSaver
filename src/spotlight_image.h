#pragma once

#include <utility>

constexpr int spotlight_image_width = 1920;
constexpr int spotlight_image_height = 1080;

constexpr bool is_spotlight_image_resolution(const std::pair<int, int> resolution) {
    return resolution.first == spotlight_image_width && resolution.second == spotlight_image_height;
}
