# Spotlight Saver

This app saves the beautiful Windows 10 spotlight wallpapers (seen on lockscreen) to your pictures folders. These images are switched out every few days so you can run this program every so often to get the new content.

## Download
Grab the latest Release.zip [here](https://github.com/depthbomb/SpotlightSaver/releases/latest), extract it, run it.

## Running automatically
Starting with version **1.0.1.0** you can start the program with the `--silent` launch option to save any available images and close automatically. To set the program to run on startup:
* Make a shortcut to the exe in `C:\Users\USERNAME\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup`
* Right-click the shortcut and click *Properties*
* Modify *Target* and add `--silent` after the path (with a space between it and the path)

Yes I know that it isn't really silent mode, but it runs so quickly that it might as well be.

## Caveats
* If you've somehow disabled `Microsoft.Windows.ContentDeliveryManager` then you will probably not get any new images and thus this app will not work very well
* This only works on Windows 10 (post-Anniversary update)

## Requirements
* You will need .NET Framework 4.5+ installed, but you probably already have this.