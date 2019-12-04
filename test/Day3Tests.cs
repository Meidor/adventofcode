using System.Linq;
using Xunit;

namespace AOC2019.Test
{
    public class Day3Tests
    {
        [Theory]
        [InlineData(new[] { "R8,U5,L5,D3", "U7,R6,D4,L4" }, 6)]
        [InlineData(new[] { "R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83" }, 159)]
        [InlineData(new[] { "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7" }, 135)]
        public void FindClosestIntersectionTest(string[] input, int expected)
        {
            var day3 = SetupDay3(input);
            var actual = day3.CalculateMinDistance(input);
            Assert.Equal(expected, actual);
        }

        [Theory]
        [InlineData(new[] { "R8,U5,L5,D3", "U7,R6,D4,L4" }, 30)]
        [InlineData(new[] { "R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83" }, 610)]
        [InlineData(new[] { "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7" }, 410)]
        public void FindLeastStepsTest(string[] input, int expected)
        {
            var day3 = SetupDay3(input);
            var actual = day3.CalculateMinSteps(input);
            Assert.Equal(expected, actual);
        }

        private Day3 SetupDay3(string[] input)
        {
            var day3 = new Day3();
            day3.Wire1 = day3.GetWire(input[0]);
            day3.Wire2 = day3.GetWire(input[1]);
            day3.Crosspoints = Day3.CalculateCrosspoints(day3.Wire1, day3.Wire2).Where(x => x != (0, 0)).ToList();
            return day3;
        }
    }
}
