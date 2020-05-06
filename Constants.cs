using System;
using System.IO;

namespace SpotlightSaver.Constants
{
    public class Paths
    {
        public static readonly string ImagePath = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "Packages", "Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy", "LocalState", "Assets");
        public static readonly string OutputPath = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.MyPictures), "Windows 10 Spotlight");
    }
}
