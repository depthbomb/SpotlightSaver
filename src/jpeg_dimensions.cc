#include "jpeg_dimensions.h"

#include <istream>
#include <fstream>
#include <cstdint>

namespace {
    bool read_byte(std::istream& input, std::uint8_t& value)
    {
        input.read(reinterpret_cast<char*>(&value), sizeof(value));

        return static_cast<bool>(input);
    }

    bool read_big_endian_uint16(std::istream& input, std::uint16_t& value)
    {
        std::uint8_t high_byte = 0;
        std::uint8_t low_byte = 0;

        if (!read_byte(input, high_byte) || !read_byte(input, low_byte)) {
            return false;
        }

        value = static_cast<std::uint16_t>(high_byte << 8 | low_byte);

        return true;
    }

    bool read_marker(std::istream& input, std::uint8_t& marker)
    {
        std::uint8_t prefix = 0;

        if (!read_byte(input, prefix) || prefix != 0xFF) {
            return false;
        }

        do {
            if (!read_byte(input, marker)) {
                return false;
            }
        } while (marker == 0xFF);

        return marker != 0x00;
    }

    bool is_start_of_frame(const std::uint8_t marker)
    {
        const bool is_frame_range = marker >= 0xC0 && marker <= 0xCF;

        return is_frame_range && marker != 0xC4 && marker != 0xC8 && marker != 0xCC;
    }

    bool is_standalone_marker(const std::uint8_t marker)
    {
        return marker == 0x01 || marker == 0xD8 || marker >= 0xD0 && marker <= 0xD7;
    }

    bool skip_bytes(std::istream& input, const std::uint16_t byte_count)
    {
        input.ignore(byte_count);

        return input.gcount() == byte_count;
    }
}

std::pair<int, int> get_image_resolution(const std::filesystem::path& path)
{
    std::ifstream input(path, std::ios::binary);
    std::uint8_t first_byte = 0;
    std::uint8_t second_byte = 0;

    if (!read_byte(input, first_byte) || !read_byte(input, second_byte)
        || first_byte != 0xFF || second_byte != 0xD8) {
        return {0, 0};
    }

    while (input) {
        std::uint8_t marker = 0;

        if (!read_marker(input, marker) || marker == 0xD9 || marker == 0xDA) {
            return {0, 0};
        }

        if (is_standalone_marker(marker)) {
            continue;
        }

        std::uint16_t segment_length = 0;

        if (!read_big_endian_uint16(input, segment_length) || segment_length < 2) {
            return {0, 0};
        }

        const auto payload_length = static_cast<std::uint16_t>(segment_length - 2);
        if (is_start_of_frame(marker)) {
            std::uint8_t precision = 0;
            std::uint16_t height = 0;
            std::uint16_t width = 0;

            if (payload_length < 6 || !read_byte(input, precision)
                || !read_big_endian_uint16(input, height)
                || !read_big_endian_uint16(input, width)
                || precision == 0 || width == 0 || height == 0) {
                return {0, 0};
            }

            return {width, height};
        }

        if (!skip_bytes(input, payload_length)) {
            return {0, 0};
        }
    }

    return {0, 0};
}
