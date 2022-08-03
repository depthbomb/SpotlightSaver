import platform
from sys import exit
from PIL import Image
from shutil import copyfile
from os import path, mkdir, listdir
from constants import SEARCH_PATH, OUTPUT_PATH


def on_valid_os() -> bool:
    """Checks if the OS the script is running on is at least Windows 10 Anniversary (10.0.1607)."""
    version: int = int(platform.version().split(".")[2])
    return platform.system() == "Windows" and platform.release() == "10" and version >= 1607


if not on_valid_os():
    print("This script only works on Windows 10 Anniversary Update (10.0.1607)")
    exit(1)


def run() -> None:
    """Searches for Windows 10 Spotlight images and saves them to the local Pictures folder."""
    mkdir(OUTPUT_PATH) if not path.exists(OUTPUT_PATH) else None

    for wallpaper in get_wallpaper_paths():
        wallpaper_filename: str = path.basename(wallpaper)
        output_filepath:    str = path.join(OUTPUT_PATH, f"{wallpaper_filename}.png")
        if not path.exists(output_filepath):
            copyfile(wallpaper, output_filepath)
            print("Copied file %s to %s" % (wallpaper, output_filepath))
        else:
            print("Wallpaper %s already exists" % output_filepath)

    exit(0)


def get_wallpaper_paths() -> list[str]:
    """Returns a list of paths to valid wallpaper-like files."""
    wallpaper_paths: list[str] = []
    for file in listdir(SEARCH_PATH):
        filepath:   str = path.join(SEARCH_PATH, file)
        img:      Image = Image.open(filepath)
        width:      int = img.size[0]
        if width == 1920:
            wallpaper_paths.append(filepath)

    return wallpaper_paths


if __name__ == "__main__":
    run()
