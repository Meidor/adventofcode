using System;
using System.Runtime.CompilerServices;

[assembly: InternalsVisibleTo("test")]
namespace AOC2019
{
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length < 1)
            {
                Console.WriteLine("Please supply the day to solve as an argument.");
                return;
            }
            if (int.TryParse(args[0], out var day))
            {
                SolveDay(day);
            }
            else
            {
                Console.WriteLine($"{args[0]} is not a valid day number");
            }
        }

        static void SolveDay(int dayNumber)
        {
            var name = $"Day{dayNumber}";
            var assembly = typeof(Program).Assembly;
            var type = assembly.GetType($"AOC2019.{name}");
            if (type == null)
            {
                throw new NotImplementedException($"{name} is not implemented.");
            }
            var day = Activator.CreateInstance(type, null);
            var puzzle1 = type.GetMethod("Puzzle1");
            var puzzle2 = type.GetMethod("Puzzle2");
            Console.WriteLine($"Day {dayNumber} Puzzle 1: {puzzle1.Invoke(day, null)}");
            Console.WriteLine($"Day {dayNumber} Puzzle 2: {puzzle2.Invoke(day, null)}");
        }
    }
}
