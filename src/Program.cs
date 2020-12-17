using System;
using System.IO;
using System.Reflection;
using System.Runtime.CompilerServices;

[assembly: InternalsVisibleTo("AOC2020.Tests")]
namespace AOC2020
{
    internal class Program
    {
        private static int Main(string[] args)
        {
            string? dayInput;
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
                return SolveDay(day);
            }
            else
            {
                Console.Error.WriteLine($"{args[0]} is not a valid day number");
                return 2;
            }
        }

        private static void PrintTitle()
        {
            var dir = Path.GetDirectoryName(Assembly.GetEntryAssembly()?.Location);
            if (dir == null) return;
            using var fontStream = new StreamReader(Path.Combine(dir, "colossal.fig"));
            var font = new WenceyWang.FIGlet.FIGletFont(fontStream.BaseStream);
            var text = new WenceyWang.FIGlet.AsciiArt("AOC2020", font, WenceyWang.FIGlet.CharacterWidth.Full);
            Console.WriteLine();
            Console.WriteLine();
            Console.Write(text.ToString());
            Console.WriteLine();
            Console.WriteLine();
        }

        private static int SolveDay(int dayNumber)
        {
            var name = $"Day{dayNumber:D2}";
            var assembly = typeof(Program).Assembly;
            var type = assembly.GetType($"AOC2020.{name}");
            if (type == null)
            {
                Console.Error.WriteLine($"Puzzle {name} is not implemented.");
                return 1;
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
            return 0;
        }
    }
}
