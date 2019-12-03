using BenchmarkDotNet.Attributes;

namespace AOC2019.Benchmarks
{
    [MemoryDiagnoser]
    public class Day3Benchmark
    {
        [Benchmark]
        public string Puzzle1()
        {
            var day3 = new Day3(@"C:\repos\aoc-2019\input\3.puzzle");
            return day3.Puzzle1();
        }

        [Benchmark]
        public string Puzzle2()
        {
            var day3 = new Day3(@"C:\repos\aoc-2019\input\3.puzzle");
            return day3.Puzzle2();
        }
    }
}