using System.IO;

namespace AOC2019
{
    public abstract class Puzzle
    {
        protected string[] InputLines { get; }
        protected Puzzle()
        {
            int day = int.Parse(this.GetType().Name.Replace("Day", ""));
            InputLines = File.ReadAllLines(Path.Combine("input", $"{day}.puzzle"));
        }

        public abstract string Puzzle1();
        public abstract string Puzzle2();
    }
}