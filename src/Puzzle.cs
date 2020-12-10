using System;
using System.IO;
using System.Reflection;

namespace AOC2020
{
    public abstract class Puzzle : IDisposable
    {
        private bool disposedValue;

        protected string[] InputLines { get; }
        protected string RawText { get; }
        protected StreamReader PuzzleReader { get; }

        protected Puzzle()
        {
            int day = int.Parse(GetType().Name[^2..]);
            var dir = Path.GetDirectoryName(Assembly.GetEntryAssembly()?.Location);
            if (dir == null) throw new ArgumentNullException(nameof(dir));

            var path = Path.Combine(dir, "input", $"{day:D2}.puzzle");
            InputLines = File.ReadAllLines(path);
            RawText = File.ReadAllText(path);
            PuzzleReader = new StreamReader(path);
        }

        public abstract string Puzzle1();
        public abstract string Puzzle2();

        protected virtual void Dispose(bool disposing)
        {
            if (!disposedValue)
            {
                if (disposing)
                {
                    PuzzleReader.Dispose();
                }
                disposedValue = true;
            }
        }

        public void Dispose()
        {
            Dispose(disposing: true);
            GC.SuppressFinalize(this);
        }
    }
}