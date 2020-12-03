using AOC2020.Models;

namespace AOC2020
{
    public sealed class Day3 : Puzzle
    {
        public bool[,] Map { get; }

        public Day3()
        {
            Map = ParseMap(InputLines);
        }

        public static bool[,] ParseMap(string[] rows)
        {
            var width = rows[0].Length;
            var height = rows.Length;
            var map = new bool[height, width];
            for (int y = 0; y < height; y++)
            {
                var row = rows[y];
                for (int x = 0; x < width; x++)
                {
                    map[y, x] = row[x] == '#';
                }
            }
            return map;
        }

        public static int CountTrees(bool[,] map, Point start, Point slope)
        {
            var trees = 0;
            var width = map.GetLength(1);
            var height = map.GetLength(0);
            while (true)
            {
                start += slope;
                if(start.X >= width)
                {
                    start = new Point(start.X - width, start.Y);
                }

                if(start.Y >= height)
                {
                    return trees;
                }

                if(map[start.Y, start.X])
                {
                    trees++;
                }
            }
        }

        public override string Puzzle1()
        {
            Point start = new (0, 0);
            Point slope = new Point(3, 1);
            return CountTrees(Map, start, slope).ToString();
        }

        public override string Puzzle2()
        {
            Point start = new Point(0, 0);

            var slopes = new Point[]
            {
                new Point(1, 1),
                new Point(3, 1),
                new Point(5, 1),
                new Point(7, 1),
                new Point(1, 2),
            };

            long result = 1;
            foreach (var slope in slopes)
            {
                result *= CountTrees(Map, start, slope);
            }
            return result.ToString();
        }
    }
}
