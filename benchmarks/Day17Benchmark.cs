using BenchmarkDotNet.Attributes;

namespace AOC2020.Benchmarks
{
    [MemoryDiagnoser]
    public class Day17Benchmark
    {
        private Day17 puzzle;

        [GlobalSetup]
        public void Setup()
        {
            puzzle = new();
        }

        [Benchmark]
        public void Puzzle1()
        {
            puzzle.Puzzle1();
        }

        [Benchmark]
        public void Puzzle2()
        {
            puzzle.Puzzle2();
        }
    }
}
