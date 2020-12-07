using System.Linq;

namespace AOC2020
{
    public sealed class Day1 : Puzzle
    {
        public int[] Input => InputLines.ParseInt().ToArray();

        public override string Puzzle1()
        {
            var input = Input;
            return (from x in input
                    from y in input
                    where x + y == 2020
                    select x * y).First().ToString();
        }

        public override string Puzzle2()
        {
            var input = Input;
            return (from x in input
                    from y in input
                    from z in input
                    where x + y + z == 2020
                    select x * y * z).First().ToString();
        }
    }
}
