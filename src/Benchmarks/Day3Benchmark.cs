using BenchmarkDotNet.Attributes;

namespace AOC2019.Benchmarks
{
    [MemoryDiagnoser]
    public class Day3Benchmark
    {
        public Day3 Day3 { get; private set; }

        [GlobalSetup]
        public void GlobalSetup()
        {
            Day3 = new Day3();
        }

        [Benchmark]
        public void LoadPuzzle()
        {
            Day3 = new Day3();
        }

        [Benchmark]
        public string Puzzle1()
        {
            return Day3.Puzzle1();
        }

        [Benchmark]
        public string Puzzle2()
        {
            return Day3.Puzzle2();
        }
    }
}