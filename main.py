import platform
from sys import exit
from PIL import Image
from pathlib import Path
from shutil import copyfile
from os import getenv, environ

SEARCH_PATH = Path(getenv("LOCALAPPDATA"), "Packages", "Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy", "LocalState", "Assets")
OUTPUT_PATH = Path(environ["USERPROFILE"], "Pictures", "Windows 10 Spotlight")


def on_valid_os() -> bool:
    """Checks if the OS the script is running on is at least Windows 10 Anniversary (10.0.1607)."""
    version = int(platform.version().split(".")[2])
    return platform.system() == "Windows" and platform.release() == "10" and version >= 1607


if not on_valid_os():
    print("This script only works on Windows 10 Anniversary Update (10.0.1607)")
    exit(1)


def get_wallpaper_paths() -> list[Path]:
    """Returns a list of paths to valid wallpaper-like files."""
    wallpaper_paths: list[Path] = []
    for file in SEARCH_PATH.iterdir():
        filepath = SEARCH_PATH.joinpath(str(file))
        with Image.open(filepath) as img:
            width = img.size[0]
            if width == 1920:
                wallpaper_paths.append(filepath)

    return wallpaper_paths


def main() -> None:
    OUTPUT_PATH.mkdir() if not OUTPUT_PATH.exists() else None

    for wallpaper in get_wallpaper_paths():
        wallpaper_filename = wallpaper.name
        output_filepath = OUTPUT_PATH.joinpath(f"{wallpaper_filename}.jpg")
        if not output_filepath.exists():
            copyfile(wallpaper, output_filepath)
            print("Copied file %s to %s" % (wallpaper, output_filepath))
        else:
            print("Wallpaper %s already exists" % output_filepath)

    exit(0)


if __name__ == "__main__":
    main()
