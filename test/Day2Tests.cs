using Xunit;

namespace AOC2019.Test
{
    public class Day2Tests
    {
        [Theory]
        [InlineData(new[] { 1, 0, 0, 0, 99 }, new[] { 2, 0, 0, 0, 99 })]
        [InlineData(new[] { 2, 3, 0, 3, 99 }, new[] { 2, 3, 0, 6, 99 })]
        [InlineData(new[] { 2, 4, 4, 5, 99, 0 }, new[] { 2, 4, 4, 5, 99, 9801 })]
        [InlineData(new[] { 1, 1, 1, 4, 99, 5, 6, 0, 99 }, new[] { 30, 1, 1, 4, 2, 5, 6, 0, 99 })]
        public void RunProgramTest(int[] input, int[] expected)
        {
            var actual = Day2.RunProgram(input);
            Assert.Equal(expected, actual);
            Assert.NotEqual(actual, input);
        }
    }
}
