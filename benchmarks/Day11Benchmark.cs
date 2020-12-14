using BenchmarkDotNet.Attributes;

namespace AOC2020.Benchmarks
{
    public class Day11Benchmark
    {
        private Day11 puzzle;

        [GlobalSetup]
        public void Setup()
        {
            puzzle = new Day11();
        }

        [Benchmark]
        public void Puzzle1()
        {
            puzzle!.Puzzle1();
        }

        [Benchmark]
        public void Puzzle2()
        {
            puzzle!.Puzzle2();
        }
    }
}
