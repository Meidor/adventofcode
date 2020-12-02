using System.IO;
using System.Reflection;

namespace AOC2020
{
    public abstract class Puzzle
    {
        protected string[] InputLines { get; }
        protected Puzzle()
        {
            int day = int.Parse(GetType().Name[3..]);
            var dir = Path.GetDirectoryName(Assembly.GetEntryAssembly().Location);
            InputLines = File.ReadAllLines(Path.Combine(dir, "input", $"{day}.puzzle"));
        }

        public abstract string Puzzle1();
        public abstract string Puzzle2();
    }
}