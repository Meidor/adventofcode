using System.Linq;
using System;

namespace AOC2019
{
    public class Day1 : Puzzle
    {
        public int Puzzle1()
        {
            return InputLines
                        .ParseInt()
                        .Select(i => CalculateFuel(i))
                        .Sum();
        }

        public int Puzzle2()
        {
            var startInput = InputLines.ParseInt();
            var solution = 0;
            foreach (var input in startInput)
            {
                var fuel = CalculateFuel(input, 0);
                solution += fuel;
            }
            return solution;
        }

        internal static int CalculateFuel(int mass) => (int)Math.Floor(mass / 3f) - 2;

        internal static int CalculateFuel(int mass, int total)
        {
            var newMass = CalculateFuel(mass);
            if (newMass <= 0)
            {
                return total;
            }
            return CalculateFuel(newMass, total + newMass);
        }
    }
}
