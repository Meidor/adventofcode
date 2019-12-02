using System;
using System.Runtime.CompilerServices;

[assembly: InternalsVisibleTo("test")]
namespace AOC2019
{
    class Program
    {
        static void Main(string[] args)
        {
            SolveDay2();
        }

        static void SolveDay1()
        {
            var day1 = new Day1();
            var puzzle1 = day1.Puzzle1();
            var puzzle2 = day1.Puzzle2();
            Console.WriteLine($"Day 1 Puzzle 1: {puzzle1}");
            Console.WriteLine($"Day 1 Puzzle 2: {puzzle2}");
        }

        static void SolveDay2()
        {
            var day2 = new Day2();
            var puzzle1 = day2.Puzzle1();
            var puzzle2 = day2.Puzzle2();
            Console.WriteLine($"Day 2 Puzzle 1: {puzzle1}");
            Console.WriteLine($"Day 2 Puzzle 2: {puzzle2}");
        }
    }
}
