using BenchmarkDotNet.Attributes;

namespace AOC2019.Benchmarks
{
    [MemoryDiagnoser]
    public class Day4Benchmark
    {
        [Benchmark]
        public string Puzzle1()
        {
            var day4 = new Day4();
            return day4.Puzzle1();
        }

        [Benchmark]
        public string Puzzle2()
        {
            var day4 = new Day4();
            return day4.Puzzle2();
        }
    }
}