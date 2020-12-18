using AOC2020.Helpers;
using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

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
                    Cycle(is4d);
                }
            }

            private static IEnumerable<Vector4> ToSimulate(HashSet<Vector4> activeCubes, bool is4d)
            {
                foreach (var cube in activeCubes)
                {
                    yield return cube;
                    foreach (var point in is4d ? n4d : n3d)
                    {
                        yield return cube + point;
                    }
                }
            }

            private void Cycle(bool is4d)
            {
                List<Vector4> addList = new();
                List<Vector4> removeList = new();

                Parallel.ForEach(ToSimulate(ActiveCubes, is4d), (point) =>
                {
                    var active = ActiveCubes.Contains(point);
                    var neighbours = CountNeighbours(point, is4d);
                    if (active && !(neighbours == 2 || neighbours == 3))
                    {
                        lock (removeList)
                        {
                            removeList.Add(point);
                        }
                    }
                    else if (!active && neighbours == 3)
                    {
                        lock (addList)
                        {
                            addList.Add(point);
                        }
                    }
                });

                ActiveCubes.RemoveRange(removeList);
                ActiveCubes.AddRange(addList);
            }

            private static readonly int[] plus = new[] { -1, 0, 1 };

            private static readonly Vector4[] n3d = (from x in plus
                                                     from y in plus
                                                     from z in plus
                                                     where (x, y, z) != (0, 0, 0)
                                                     select new Vector4(x, y, z)).ToArray();

            private static readonly Vector4[] n4d = (from x in plus
                                                     from y in plus
                                                     from z in plus
                                                     from w in plus
                                                     where (x, y, z, w) != (0, 0, 0, 0)
                                                     select new Vector4(x, y, z, w)).ToArray();

            private int CountNeighbours(Vector4 pos, bool is4d)
            {
                var count = 0;
                foreach (var point in is4d ? n4d : n3d)
                {
                    if (ActiveCubes.Contains(pos + point))
                    {
                        count++;
                        if (count > 3)
                        {
                            return count;
                        }
                    }
                }
                return count;
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
