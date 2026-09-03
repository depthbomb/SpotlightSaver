#include "jpeg_dimensions.h"
#include "spotlight_image.h"

#include <string>
#include <vector>
#include <chrono>
#include <fstream>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <filesystem>
#include <stdexcept>
#include <system_error>

namespace fs = std::filesystem;

class TemporaryDirectory {
public:
    TemporaryDirectory() {
        const auto unique_value = std::chrono::steady_clock::now().time_since_epoch().count();

        path_ = fs::temp_directory_path() / ("spotlight_saver_tests_" + std::to_string(unique_value));

        fs::create_directories(path_);
    }

    ~TemporaryDirectory() {
        std::error_code error;

        fs::remove_all(path_, error);
    }

    const fs::path& path() const {
        return path_;
    }

private:
    fs::path path_;
};

class TestRunner {
public:
    void expect(const bool condition, const std::string& name) {
        if (!condition) {
            std::cerr << "FAILED: " << name << '\n';
            failure_count_++;
        }
    }

    void expect_resolution(
        const std::pair<int, int> actual,
        const std::pair<int, int> expected,
        const std::string& name
    ) {
        if (actual != expected) {
            std::cerr << "FAILED: " << name << " (expected " << expected.first << 'x' << expected.second
                      << ", got " << actual.first << 'x' << actual.second << ")\n";
            failure_count_++;
        }
    }

    int result() const {
        if (failure_count_ == 0) {
            std::cout << "All tests passed\n";

            return EXIT_SUCCESS;
        }

        std::cerr << failure_count_ << " test(s) failed\n";

        return EXIT_FAILURE;
    }

private:
    int failure_count_ = 0;
};

std::vector<std::uint8_t> make_jpeg(
    const std::uint8_t frame_marker,
    const std::uint16_t width,
    const std::uint16_t height
) {
    return {
        0xFF, 0xD8,
        0xFF, 0xE0, 0x00, 0x04, 0xDE, 0xAD,
        0xFF, 0xFF, frame_marker,
        0x00, 0x0B, 0x08,
        static_cast<std::uint8_t>(height >> 8), static_cast<std::uint8_t>(height),
        static_cast<std::uint8_t>(width >> 8), static_cast<std::uint8_t>(width),
        0x01, 0x01, 0x11, 0x00,
        0xFF, 0xD9,
    };
}

fs::path write_file(
    const fs::path& directory,
    const std::string& name,
    const std::vector<std::uint8_t>& contents
) {
    const auto path = directory / name;
    std::ofstream output(path, std::ios::binary);

    output.write(reinterpret_cast<const char*>(contents.data()), static_cast<std::streamsize>(contents.size()));
    if (!output) {
        throw std::runtime_error("Failed to create test file");
    }

    return path;
}

void test_valid_dimensions(TestRunner& runner, const fs::path& directory) {
    const auto baseline_path = write_file(directory, "baseline.jpg", make_jpeg(0xC0, 1920, 1080));
    const auto extended_path = write_file(directory, "extended.jpg", make_jpeg(0xC1, 1280, 720));
    const auto progressive_path = write_file(directory, "progressive.jpg", make_jpeg(0xC2, 1080, 1920));

    runner.expect_resolution(get_image_resolution(baseline_path), {1920, 1080}, "baseline dimensions");
    runner.expect_resolution(get_image_resolution(extended_path), {1280, 720}, "extended dimensions");
    runner.expect_resolution(get_image_resolution(progressive_path), {1080, 1920}, "progressive dimensions");
}

void test_early_return(TestRunner& runner, const fs::path& directory) {
    auto contents = make_jpeg(0xC0, 1920, 1080);
    const auto conflicting_frame = make_jpeg(0xC0, 1, 1);

    contents.resize(contents.size() - 2);
    contents.insert(contents.end(), conflicting_frame.begin() + 2, conflicting_frame.end());

    const auto path = write_file(directory, "trailing-frame.jpg", contents);

    runner.expect_resolution(get_image_resolution(path), {1920, 1080}, "first frame wins");
}

void test_malformed_files(TestRunner& runner, const fs::path& directory) {
    const auto empty_path = write_file(directory, "empty.jpg", {});
    const auto non_jpeg_path = write_file(directory, "not-jpeg.jpg", {0x00, 0x01});
    const auto truncated_length_path = write_file(directory, "truncated-length.jpg", {0xFF, 0xD8, 0xFF, 0xE0, 0x00});
    const auto invalid_length_path = write_file(directory, "invalid-length.jpg", {0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x01});
    const auto truncated_payload_path = write_file(directory, "truncated-payload.jpg", {0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x05, 0x00});
    const auto scan_path = write_file(directory, "scan-before-frame.jpg", {0xFF, 0xD8, 0xFF, 0xDA, 0x00, 0x02});
    const auto missing_path = directory / "missing.jpg";

    runner.expect_resolution(get_image_resolution(empty_path), {0, 0}, "empty file");
    runner.expect_resolution(get_image_resolution(non_jpeg_path), {0, 0}, "non-JPEG file");
    runner.expect_resolution(get_image_resolution(truncated_length_path), {0, 0}, "truncated segment length");
    runner.expect_resolution(get_image_resolution(invalid_length_path), {0, 0}, "invalid segment length");
    runner.expect_resolution(get_image_resolution(truncated_payload_path), {0, 0}, "truncated segment payload");
    runner.expect_resolution(get_image_resolution(scan_path), {0, 0}, "scan before frame");
    runner.expect_resolution(get_image_resolution(missing_path), {0, 0}, "missing file");
}

void test_spotlight_resolution(TestRunner& runner) {
    runner.expect(is_spotlight_image_resolution({1920, 1080}), "accept exact Spotlight dimensions");
    runner.expect(!is_spotlight_image_resolution({1920, 720}), "reject incorrect height");
    runner.expect(!is_spotlight_image_resolution({1280, 1080}), "reject incorrect width");
    runner.expect(!is_spotlight_image_resolution({1080, 1920}), "reject portrait dimensions");
}

int run_tests() {
    const TemporaryDirectory directory;
    TestRunner runner;

    test_valid_dimensions(runner, directory.path());
    test_early_return(runner, directory.path());
    test_malformed_files(runner, directory.path());
    test_spotlight_resolution(runner);

    return runner.result();
}

int main() {
    try {
        return run_tests();
    } catch (const std::exception& error) {
        std::cerr << "FAILED: unexpected exception: " << error.what() << '\n';

        return EXIT_FAILURE;
    }
}
