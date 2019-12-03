using System;
using System.Collections.Generic;

namespace AOC2019
{
    public static class AOCExtensions
    {
        public static IEnumerable<int> ParseInt(this string[] input)
        {
            foreach (var i in input)
            {
                yield return int.Parse(i);
            }
        }

        private const double Epsilon = 1e-10;
        public static bool IsZero(this double d) => Math.Abs(d) < Epsilon;
    }
}