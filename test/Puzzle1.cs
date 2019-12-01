using Xunit;

namespace AOC2019.Test
{
    public class Day1Tests
    {
        [Theory]
        [InlineData(12, 2)]
        [InlineData(14, 2)]
        [InlineData(1969, 654)]
        [InlineData(100756, 33583)]
        public void Puzzle1(int input, int expected)
        {
            var solution = Day1.CalculateFuel(input);
            Assert.Equal(expected, solution);
        }

        [Theory]
        [InlineData(14, 2)]
        [InlineData(1969, 966)]
        [InlineData(100756, 50346)]
        public void Puzzle2(int input, int expected)
        {
            var solution = Day1.CalculateFuel(input, 0);
            Assert.Equal(expected, solution);
        }
    }
}
