using AOC2020.Helpers;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace AOC2020
{
    public sealed class Day17 : Puzzle
    {
        private readonly CubesOfLife col;

        public sealed class CubesOfLife
        {
            public HashSet<Vector4> InitialState { get; init; }
            public HashSet<Vector4> ActiveCubes { get; private set; } = new();

            public CubesOfLife(string[] input)
            {
                Vector4 pos = new(0, 0, 0, 0);
                for (int y = 0; y < input.Length; y++)
                {
                    for (int x = 0; x < input[0].Length; x++)
                    {
                        if (input[y][x] == '#')
                        {
                            ActiveCubes.Add(pos + (x, y));
                        }
                    }
                }
                InitialState = new(ActiveCubes);
            }

            public void Reset()
            {
                ActiveCubes = new(InitialState);
            }

            public void Run(int cycles, bool is4d = false)
            {
                for (int i = 0; i < cycles; i++)
                {
                    if (is4d)
                    {
                        Cycle4D();
                    }
                    else
                    {
                        Cycle();
                    }
                }
            }

            public string GetSlice(double z, double? w = null)
            {
                var (start, end) = GetSearchArea(w is not null);
                var minX = start.X;
                var maxX = end.X;
                var minY = start.Y;
                var maxY = end.Y;

                StringBuilder sb = new();
                for (var y = minY; y <= maxY; y++)
                {
                    for (var x = minX; x <= maxX; x++)
                    {
                        if (IsActive((x, y, z, w ?? 0)))
                        {
                            sb.Append('#');
                        }
                        else
                        {
                            sb.Append('.');
                        }
                    }
                    sb.Append(Environment.NewLine);
                }
                return sb.ToString();
            }

            private (Vector4 start, Vector4 end) GetSearchArea(bool is4d)
            {
                var xQuery = ActiveCubes.Select(c => c.X);
                var yQuery = ActiveCubes.Select(c => c.Y);
                var zQuery = ActiveCubes.Select(c => c.Z);
                var wQuery = ActiveCubes.Select(c => c.W);

                Vector4 add = (1, 1, 1, is4d ? 1 : 0);
                var start = (xQuery.Min(), yQuery.Min(), zQuery.Min(), wQuery.Min()) - add;
                var end = (xQuery.Max(), yQuery.Max(), zQuery.Max(), wQuery.Max()) + add;
                return (start, end);
            }

            private void Cycle4D()
            {
                HashSet<Vector4> addList = new();
                HashSet<Vector4> removeList = new();
                var (start, end) = GetSearchArea(true);
                bool active;
                int neighbours;
                for (var w = start.W; w <= end.W; w++)
                {
                    for (var z = start.Z; z <= end.Z; z++)
                    {
                        for (var y = start.Y; y <= end.Y; y++)
                        {
                            for (var x = start.X; x <= end.X; x++)
                            {
                                Vector4 pos = (x, y, z, w);
                                active = IsActive(pos);
                                neighbours = CountNeighbours(pos, true);
                                if (active && !(neighbours == 2 || neighbours == 3))
                                {
                                    removeList.Add(pos);
                                }
                                else if (!active && neighbours == 3)
                                {
                                    addList.Add(pos);
                                }
                            }
                        }
                    }
                }
                ActiveCubes.RemoveRange(removeList);
                ActiveCubes.AddRange(addList);
            }

            private void Cycle()
            {
                HashSet<Vector4> addList = new();
                HashSet<Vector4> removeList = new();
                var (start, end) = GetSearchArea(false);
                bool active;
                int neighbours;
                for (var z = start.Z; z <= end.Z; z++)
                {
                    for (var y = start.Y; y <= end.Y; y++)
                    {
                        for (var x = start.X; x <= end.X; x++)
                        {
                            Vector4 pos = (x, y, z);
                            active = IsActive(pos);
                            neighbours = CountNeighbours(pos, false);
                            if (active && !(neighbours == 2 || neighbours == 3))
                            {
                                removeList.Add(pos);
                            }
                            else if (!active && neighbours == 3)
                            {
                                addList.Add(pos);
                            }
                        }
                    }
                }
                ActiveCubes.RemoveRange(removeList);
                ActiveCubes.AddRange(addList);
            }

            private bool IsActive(Vector4 pos)
            {
                return ActiveCubes.Contains(pos);
            }

            private static readonly int[] plus = new[] { -1, 0, 1 };
            private readonly Vector4[] n3d = (from x in plus
                                              from y in plus
                                              from z in plus
                                              where (x, y, z) != (0, 0, 0)
                                              select new Vector4(x, y, z)).ToArray();

            private readonly Vector4[] n4d = (from x in plus
                                              from y in plus
                                              from z in plus
                                              from w in plus
                                              where (x, y, z, w) != (0, 0, 0, 0)
                                              select new Vector4(x, y, z, w)).ToArray();

            private int CountNeighbours(Vector4 pos, bool is4d)
            {
                var plus = new[] { -1, 0, 1 };
                if (is4d)
                {
                    return n4d.Count(n => IsActive(pos + n));
                }
                return n3d.Count(n => IsActive(pos + n));
            }
        }

        public Day17()
        {
            col = new CubesOfLife(InputLines);
        }

        public Day17(string[] input)
        {
            col = new CubesOfLife(input);
        }

        public override string Puzzle1()
        {
            col.Reset();
            col.Run(6);
            return col.ActiveCubes.Count.ToString();
        }

        public override string Puzzle2()
        {
            col.Reset();
            col.Run(6, true);
            return col.ActiveCubes.Count.ToString();
        }
    }
}
