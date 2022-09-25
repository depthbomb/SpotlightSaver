using System.Drawing;
using Microsoft.Extensions.FileSystemGlobbing;

namespace SpotlightSaver;

internal static class Program
{
    private static readonly Matcher Matcher = new();
    private static readonly string  PicturesFolderPath = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.MyPictures),
        "Windows Spotlight"
    );
    private static readonly string SpotlightFolderPath = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
        @"Packages\Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy\LocalState\Assets"
    );
    
    private static int Main(string[] args)
    {
        if (!Directory.Exists(SpotlightFolderPath))
        {
            Console.WriteLine("Windows Spotlight folder not found");
            return 1;
        }

        Directory.CreateDirectory(PicturesFolderPath);
        
        Matcher.AddInclude("*");

        foreach (string file in Matcher.GetResultsInFullPath(SpotlightFolderPath).ToList())
        {
            var    fileInfo      = new FileInfo(file);
            string wallpaperPath = Path.Combine(PicturesFolderPath, $"{fileInfo.Name}.png");
            if (!File.Exists(wallpaperPath))
            {
                using (var image = new Bitmap(file))
                {
                    if (image.Width < 1920 || image.Height < 1080) 
                        continue;
                    
                    File.Copy(file, wallpaperPath, true);
                        
                    Console.WriteLine("Saved wallpaper {0} to {1}", fileInfo.Name, PicturesFolderPath);
                }
            }
            else
            {
                Console.WriteLine("Wallpaper {0} already saved", fileInfo.Name);
            }
        }

        return 0;
    }
}