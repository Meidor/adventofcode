using System;
using System.Runtime.CompilerServices;
using AOC2019.Benchmarks;
using BenchmarkDotNet.Running;

[assembly: InternalsVisibleTo("test")]
namespace AOC2019
{
    class Program
    {
        static void Main(string[] args)
        {
            string dayInput;
            if (args.Length < 1)
            {
                PrintTitle();
                Console.Write("Which day would you like to run: ");
                dayInput = Console.ReadLine();
            }
            else
            {
                dayInput = args[0];
                Console.WriteLine();
            }
            if (int.TryParse(dayInput, out var day))
            {
                SolveDay(day);
            }
            else
            {
                Console.WriteLine($"{args[0]} is not a valid day number");
            }
        }

        static void PrintTitle()
        {
            Console.WriteLine();
            Console.WriteLine();
            Console.WriteLine("       d8888 .d88888b.  .d8888b.  .d8888b.  .d8888b.  d888  .d8888b.  ");
            Console.WriteLine("      d88888d88P\" \"Y88bd88P  Y88bd88P  Y88bd88P  Y88bd8888 d88P  Y88b ");
            Console.WriteLine("     d88P888888     888888    888       888888    888  888 888    888 ");
            Console.WriteLine("    d88P 888888     888888            .d88P888    888  888 Y88b. d888 ");
            Console.WriteLine("   d88P  888888     888888        .od888P\" 888    888  888  \"Y888P888 ");
            Console.WriteLine("  d88P   888888     888888    888d88P\"     888    888  888        888 ");
            Console.WriteLine(" d8888888888Y88b. .d88PY88b  d88P888\"      Y88b  d88P  888 Y88b  d88P ");
            Console.WriteLine("d88P     888 \"Y88888P\"  \"Y8888P\" 888888888  \"Y8888P\" 8888888\"Y8888P\"  ");
            Console.WriteLine();
            Console.WriteLine();
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
            Console.WriteLine("-------------------------------------");
            Console.WriteLine($"Day {dayNumber}");
            Console.WriteLine("-------------------------------------");
            if (puzzle1 != null)
            {
                Console.WriteLine($"Puzzle 1: {puzzle1.Invoke(day, null)}");
            }
            if (puzzle2 != null)
            {
                Console.WriteLine($"Puzzle 2: {puzzle2.Invoke(day, null)}");
            }
            Console.WriteLine("-------------------------------------");
        }
    }
}
