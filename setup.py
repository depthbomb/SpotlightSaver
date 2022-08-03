from setuptools import setup
# noinspection PyUnresolvedReferences
import py2exe

options = dict(
    dist_dir="./dist",
    optimize=2,
    dll_excludes=['msvcr71.dll'],
    excludes=['_ssl', 'pyreadline', 'difflib', 'doctest', 'locale', 'optparse', 'pickle', 'calendar'],
)

setup(
    version="2.0.0",
    description="Saves the pretty Windows 10 lockscreen wallpapers to your pictures folder",
    author="Caprine Logic",
    console=[{
        "script": "main.py",
        "dest_base": "spotlight_saver",
        "icon_resources": [(1, "icon.ico")]
    }],
    options={
        "py2exe": options
    }
)
