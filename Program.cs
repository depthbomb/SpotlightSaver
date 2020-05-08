using System;
using System.IO;
using System.Linq;
using System.Drawing;
using System.Collections.Generic;

using Serilog;
using Serilog.Core;
using Serilog.Events;

using SpotlightSaver.Constants;

namespace SpotlightSaver
{
    class Program
    {
        static bool SilentMode = false;

        static string InPath = Paths.ImagePath;
        static string OutPath = Paths.OutputPath;

        static List<string> WallpapersSaved = new List<string>();

        static Logger Logger;

        static void Main(string[] args)
        {
            bool verbose;
#if DEBUG
            verbose = true;
#else
            verbose = args.Any(a => a.Contains("-v") || a.Contains("--verbose"));
#endif

            SilentMode = args.Any(a => a.Contains("--silent"));

            Logger = CreateLogger(verbose);

            if (SilentMode)
            {
                Work();

                Environment.Exit(0);
            }
            else
            {
                Intro();

                Work();

                if (WallpapersSaved.Count > 0)
                {
                    Logger.Information("Saved {Num} wallpaper(s)!", WallpapersSaved.Count);
                }
                else
                {
                    Logger.Information("No new wallpapers were saved, try again in a few days!");
                }

                ExitState();
            }
        }

        /// <summary>
        /// Shows a set of intro messages if this is the user's first time running the app
        /// </summary>
        static void Intro()
        {
            if (Properties.Settings.Default.FirstRun)
            {
                Console.WriteLine();
                Console.WriteLine(" This tool processes the image assets that are used in the Windows 10 lockscreen.");
                Console.WriteLine(" These images are changed every so often so you can run this program every few days to get the latest files.");
                Console.Write(" All of the images are saved to ");
                Console.ForegroundColor = ConsoleColor.Cyan;
                Console.Write(Paths.OutputPath);
                Console.ResetColor();
                Console.WriteLine("\n");
                Console.WriteLine(" Press Enter to continue (this message will only be shown just this time)");
                Console.ReadLine();
                Console.Clear();

                Properties.Settings.Default.FirstRun = false;
                Properties.Settings.Default.Save();
            }
        }

        /// <summary>
        /// Do work!
        /// </summary>
        static void Work()
        {
            if (!Directory.Exists(InPath)) // Just in case...
            {
                Logger.Fatal("Spotlight image folder {Folder} was not found.", Paths.ImagePath);
                ExitState();
            }
            else
            {
                OutPath.CreateIfNotExists(out bool dirCreated);

                if (dirCreated) Logger.Debug("Created output directory {Dir}", OutPath);

                foreach (string file in Directory.EnumerateFiles(InPath))
                {
                    FileInfo info = new FileInfo(file);
                    string filename = Path.Combine(OutPath, $"{info.Name}.jpg");

                    if (!File.Exists(filename))
                    {
                        Logger.Debug("Processing file {File}", info.Name);

                        // There are many non-wallpaper images in the folder so let's try to weed them out.
                        // This likely won't catch all of them but we will vet them more later.
                        if (info.Length >= 150000)
                        {
                            Logger.Information("{File} seems like a wallpaper", info.Name);
                            using (var image = new Bitmap(file))
                            {
                                // All of the wallpapers seem to be at least 1920x1080
                                if (image.Width >= 1920)
                                {
                                    File.Copy(file, filename, true);
                                    WallpapersSaved.Add(info.Name);
                                }
                                else
                                {
                                    Logger.Information("{File} is a phone wallpaper ({Width}x{Height}), skipping", info.Name, image.Width, image.Height);
                                }
                            }
                        }
                    }
                }
            }
        }

        /// <summary>
        /// Creates a <see cref="Serilog.Core.Logger">Serilog</see> instance
        /// <para><paramref name="verbose"/> determines whether debug messages as logged as well</para>
        /// </summary>
        /// <param name="verbose">Whether to enable verbose logging</param>
        /// <remarks>Should be called as soon as possible</remarks>
        static Logger CreateLogger(bool verbose)
        {
            return new LoggerConfiguration()
                .WriteTo.Console(
                    restrictedToMinimumLevel: verbose ? LogEventLevel.Debug : LogEventLevel.Information,
                    outputTemplate: "[{Timestamp:HH:mm:ss}] [{Level:u3}] {Message:l}{NewLine}{Exception}"
                 )
                .CreateLogger();
        }

        /// <summary>
        /// Puts the application in an "exit state" where input will close the application
        /// </summary>
        static void ExitState()
        {
            Console.WriteLine();
            Console.WriteLine("Press Enter to exit");
            Console.ReadLine();
            Environment.Exit(0);
        }
    }
}
