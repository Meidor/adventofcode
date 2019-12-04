using Xunit;

namespace AOC2019.Test
{
    public class Day4Tests
    {
        [Theory]
        [InlineData(111111, true)]
        [InlineData(111121, false)]
        [InlineData(223450, false)]
        [InlineData(123789, false)]

        public void Puzzle1(int attempt, bool expected)
        {
            var solution = Day4.IsValidPassword(attempt);
            Assert.Equal(expected, solution);
        }

        [Theory]
        // [InlineData(112233, true)]
        // [InlineData(123444, false)]
        [InlineData(111122, true)]
        public void Puzzle2(int attempt, bool expected)
        {
            var solution = Day4.IsValidPassword2(attempt);
            Assert.Equal(expected, solution);
        }
    }
}
