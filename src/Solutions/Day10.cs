using System;
using System.Collections.Generic;
using System.Linq;

namespace AOC2020
{
    public sealed class Day10 : Puzzle
    {
        internal List<int> adapters;
        internal Dictionary<int, long> cache = new();

        public Day10()
        {
            adapters = InputLines.ParseInt().ToList();
            adapters.Sort();
            adapters.Insert(0, 0);
            adapters.Add(adapters[^1] + 3);
        }

        public long CountCombinations()
        {
            cache = new();
            return CountCombinations(0);
        }

        private long CountCombinations(int n)
        {
            if (n == adapters[^1])
            {
                return 1;
            }

            long result = 0;
            for (int i = 1; i <= 3; i++)
            {
                var c = n + i;
                if (adapters.Contains(c))
                {
                    result += cache.ContainsKey(c) ? cache[c] : CountCombinations(c);
                }
            }
            cache[n] = result;
            return result;
        }

        public long CountDifferences()
        {
            var previous = 0;
            var one = 0;
            var three = 1;
            for (int i = 1; i < adapters.Count - 1; i++)
            {
                var compare = adapters[i];
                if (compare - previous == 1)
                {
                    one++;
                }
                else if (compare - previous == 3)
                {
                    three++;
                }
                previous = compare;
            }
            return one * three;
        }

        public override string Puzzle1() => CountDifferences().ToString();
        public override string Puzzle2() => CountCombinations().ToString();
    }
}
