namespace SpotlightSaver
{
    public static class Helpers
    {
        /// <summary>
        /// Creates a <paramref name="directory"/> if it does not been created
        /// </summary>
        /// <param name="directory">Directory to check</param>
        public static void CreateIfNotExists(this string directory)
            => directory.CreateIfNotExists();

        /// <summary>
        /// Creates a <paramref name="directory"/> if it has not been <paramref name="created"/>
        /// </summary>
        /// <param name="directory">Directory to check</param>
        /// <param name="created"></param>
        public static void CreateIfNotExists(this string directory, out bool created)
        {
            created = System.IO.Directory.Exists(directory);

            if (!created)
            {
                System.IO.Directory.CreateDirectory(directory);
            }
        }
    }
}
