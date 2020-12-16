using AOC2020.Benchmarks;
using BenchmarkDotNet.Running;
using System;
namespace AOC2020.Benchmarks
{
    internal class Program
    {
        private static int Main(string[] args)
        {
            string dayInput;
            if (args.Length < 1)
            {
                Console.Write("Benchmark of which day would you like to run: ");
                dayInput = Console.ReadLine();
            }
            else
            {
                dayInput = args[0];
                Console.WriteLine();
            }

            if (int.TryParse(dayInput, out var day))
            {
                var name = $"Day{day:D2}Benchmark";
                var assembly = typeof(Program).Assembly;
                var type = assembly.GetType($"AOC2020.Benchmarks.{name}");
                if (type == null)
                {
                    Console.Error.WriteLine($"Benchmark {name} is not implemented.");
                    return 1;
                }
                BenchmarkRunner.Run(type);
                return 0;
            }
            else
            {
                Console.Error.WriteLine($"{args[0]} is not a valid day number");
                return 2;
            }
        }
    }
}
