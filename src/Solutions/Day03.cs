using AOC2020.Helpers;

namespace AOC2020
{
    public sealed class Day03 : Puzzle
    {
        public const char TREE = '#';

        public static long CountTrees(string[] map, Point start, Point[] slopes)
        {
            long trees = 1;
            foreach (var slope in slopes)
            {
                trees *= CountTrees(map, start, slope);
            }
            return trees;
        }

        public static int CountTrees(string[] map, Point start, Point slope)
        {
            var trees = 0;
            var width = map[0].Length;
            while (true)
            {
                start += slope;
                if (start.Y >= map.Length)
                {
                    return trees;
                }
                if (start.X >= width)
                {
                    start = (start.X - width, start.Y);
                }
                if (map[start.Y][start.X] == TREE)
                {
                    trees++;
                }
            }
        }

        public override string Puzzle1() => $"{CountTrees(InputLines, (0, 0), (3, 1))}";
        public override string Puzzle2() => $"{CountTrees(InputLines, (0, 0), new Point[] { (1, 1), (3, 1), (5, 1), (7, 1), (1, 2) })}";
    }
}
