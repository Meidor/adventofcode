using System;
using System.Runtime.CompilerServices;

[assembly: InternalsVisibleTo("test")]
namespace AOC2019
{
    class Program
    {
        static void Main(string[] args)
        {
            SolveDay1();
        }

        static void SolveDay1()
        {
            var day1 = new Day1();
            var puzzle1 = day1.Puzzle1();
            var puzzle2 = day1.Puzzle2();
            Console.WriteLine($"Day 1 Puzzle 1: {puzzle1}");
            Console.WriteLine($"Day 1 Puzzle 2: {puzzle2}");
        }
    }
}
