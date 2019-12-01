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
    }
}