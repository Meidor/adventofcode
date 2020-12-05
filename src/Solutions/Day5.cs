using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace AOC2020
{
    public sealed class Day5 : Puzzle
    {
        public record BoardingPass(int Row, int Column)
        {
            public int Id => (Row * 8) + Column;
        }

        public static IEnumerable<BoardingPass> ParseBoardingPasses(StreamReader puzzleReader)
        {
            while (!puzzleReader.EndOfStream)
            {
                var pass = puzzleReader.ReadLine();
                var rowRange = Enumerable.Range(0, 128);
                var columnRange = Enumerable.Range(0, 8);
                foreach (var chr in pass!)
                {
                    switch (chr)
                    {
                        case 'F':
                            rowRange = rowRange.Take(rowRange.Count() / 2);
                            break;
                        case 'B':
                            rowRange = rowRange.Skip(rowRange.Count() / 2);
                            break;
                        case 'L':
                            columnRange = columnRange.Take(columnRange.Count() / 2);
                            break;
                        case 'R':
                            columnRange = columnRange.Skip(columnRange.Count() / 2);
                            break;
                    }
                }
                var row = rowRange.Single();
                var column = columnRange.Single();
                yield return new BoardingPass(row, column);
            }
            puzzleReader.Rewind();
        }

        public static IEnumerable<BoardingPass> GetPossiblePasses()
        {
            for (int row = 0; row < 128; row++)
            {
                for (int column = 0; column < 8; column++)
                {
                    yield return new BoardingPass(row, column);
                }
            }
        }

        public override string Puzzle1()
        {
            var passes = ParseBoardingPasses(PuzzleReader);
            return $"{passes.Max(p => p.Id)}";
        }

        public override string Puzzle2()
        {
            var passes = ParseBoardingPasses(PuzzleReader).ToList();
            var notTaken = GetPossiblePasses()
                .Except(passes)
                .Single(nt => passes.Any(p => p.Id == (nt.Id + 1))
                                && passes.Any(p => p.Id == (nt.Id - 1)));
            return $"{notTaken.Id}";
        }
    }
}
