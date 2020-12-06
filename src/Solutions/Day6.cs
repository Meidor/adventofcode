using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace AOC2020
{
    public sealed class Day6 : Puzzle
    {
        public record CustomsGroup(int GroupSize, Dictionary<char, int> Answers)
        {
            public int CountAny => Answers.Keys.Count;
            public int CountAll => Answers.Values.Count(a => a == GroupSize);
        };

        public static IEnumerable<CustomsGroup> ParseAnswers(StreamReader puzzleReader)
        {
            var groupSize = 0;
            var dict = new Dictionary<char, int>();
            while (!puzzleReader.EndOfStream)
            {
                string line = puzzleReader.ReadLine()!;
                if(line.Length == 0)
                {
                    yield return new CustomsGroup(groupSize, dict);
                    dict = new Dictionary<char, int>();
                    groupSize = 0;
                    continue;
                }
                foreach(var answer in line)
                {
                    dict.AddOrUpdate(answer, 1, (acc, value) => acc + value);
                }
                groupSize++;
            }
            yield return new CustomsGroup(groupSize, dict);
            puzzleReader.Rewind();
        }

        public override string Puzzle1()
        {
            return $"{ParseAnswers(PuzzleReader).Sum(c => c.CountAny)}";
        }

        public override string Puzzle2()
        {
            return $"{ParseAnswers(PuzzleReader).Sum(c => c.CountAll)}";
        }
    }
}
