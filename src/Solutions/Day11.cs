using AOC2020.Helpers;
using System;
using System.Linq;
using System.Text;

namespace AOC2020
{
    public sealed class Day11 : Puzzle
    {
        internal GameOfChairs GoC { get; }

        public Day11()
        {
            GoC = new GameOfChairs(InputLines);
        }

        internal Day11(string[] input)
        {
            GoC = new GameOfChairs(input);
        }

        public class GameOfChairs
        {
            public enum Cell
            {
                Floor = 0,
                Empty = 1,
                Full = 2
            }

            private readonly Cell[] startGrid;
            private Cell[] Grid { get; set; }
            private int Width { get; init; }
            private int Height { get; init; }

            public GameOfChairs(string[] rows)
            {
                Height = rows.Length;
                Width = rows[0].Length;
                Grid = new Cell[Width * Height];
                int i = 0;
                foreach (var row in rows)
                {
                    foreach (var c in row)
                    {
                        switch (c)
                        {
                            case '.':
                                Grid[i] = Cell.Floor;
                                break;
                            case 'L':
                                Grid[i] = Cell.Empty;
                                break;
                            case '#':
                                Grid[i] = Cell.Full;
                                break;
                        }
                        i++;
                    }
                }
                startGrid = new Cell[Grid.Length];
                Array.Copy(Grid, startGrid, Grid.Length);
            }

            public void Reset()
            {
                Grid = startGrid;
            }

            public int GetIndex(Point p) => (p.Y * Width) + p.X;

            internal int Iteration(int minAlive, int maxDead, Func<GameOfChairs, Point, int> count)
            {
                var changes = 0;
                var nextGrid = new Cell[Grid.Length];
                Array.Copy(Grid, nextGrid, Grid.Length);
                for (int y = 0; y < Height; y++)
                {
                    for (int x = 0; x < Width; x++)
                    {
                        Point p = (x, y);
                        var i = GetIndex(p);
                        var c = GetCell(p);
                        if (c == Cell.Floor)
                        {
                            continue;
                        }

                        var nc = count(this, p);
                        if (c == Cell.Empty && nc <= minAlive)
                        {
                            changes++;
                            nextGrid[i] = Cell.Full;
                        }
                        else if (c == Cell.Full && nc >= maxDead)
                        {
                            changes++;
                            nextGrid[i] = Cell.Empty;
                        }
                    }
                }
                Grid = nextGrid;
                return changes;
            }

            public void IterateTillStable(int minAlive, int maxDead, Func<GameOfChairs, Point, int> count)
            {
                int changes;
                do
                {
                    changes = Iteration(minAlive, maxDead, count);
                } while (changes != 0);
            }

            public int CountOccupied() => Grid.Count(c => c == Cell.Full);

            public bool IsValid(Point p) => p.X >= 0 && p.X < Width && p.Y >= 0 && p.Y < Height;

            public Cell GetCell(Point p)
            {
                var i = GetIndex(p);
                if (i >= 0 && i < Grid.Length)
                {
                    return Grid[i];
                }
                throw new IndexOutOfRangeException();
            }

            private readonly Point[] directions = new Point[] { (-1, 0), (1, 0), (0, 1), (0, -1), (-1, 1), (1, 1), (-1, -1), (1, -1) };

            public int CountNeighbours(Point p)
                => directions.Select(d => p + d).Where(IsValid).Count(c => GetCell(c) == Cell.Full);

            public int CountLoSNeighbours(Point p)
                => directions.Count(direction => GetFirstNeighbour(p, direction) == Cell.Full);

            public Cell GetFirstNeighbour(Point p, Point direction)
            {
                var n = p + direction;
                while (IsValid(n))
                {
                    var cell = GetCell(n);
                    if (cell != Cell.Floor)
                    {
                        return cell;
                    }
                    n += direction;
                }
                return Cell.Floor;
            }

            public override string ToString()
            {
                var sb = new StringBuilder();
                for (int y = 0; y < Height; y++)
                {
                    for (int x = 0; x < Width; x++)
                    {
                        switch (GetCell((x, y)))
                        {
                            case Cell.Floor:
                                sb.Append('.');
                                break;
                            case Cell.Empty:
                                sb.Append('L');
                                break;
                            case Cell.Full:
                                sb.Append('#');
                                break;
                        }
                    }
                    sb.Append(Environment.NewLine);
                }
                return sb.ToString().Trim();
            }
        }

        public override string Puzzle1()
        {
            GoC.Reset();
            GoC.IterateTillStable(0, 4, (goc, p) => goc.CountNeighbours(p));
            return GoC.CountOccupied().ToString();
        }

        public override string Puzzle2()
        {
            GoC.Reset();
            GoC.IterateTillStable(0, 5, (goc, p) => goc.CountLoSNeighbours(p));
            return GoC.CountOccupied().ToString();
        }
    }
}
