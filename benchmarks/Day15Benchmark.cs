using BenchmarkDotNet.Attributes;

namespace AOC2020.Benchmarks
{
    [MemoryDiagnoser]
    public class Day15Benchmark
    {
        private Day15 puzzle;

        [GlobalSetup]
        public void Setup()
        {
            puzzle = new();
        }

        [Benchmark]
        public void Puzzle2()
        {
            puzzle!.Puzzle2();
        }
    }
}
