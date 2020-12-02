using System.Collections.Generic;
using System.Linq;

namespace AOC2020
{
    public record PasswordCheck(int Min, int Max, char Chr, string Password)
    {
        public bool CheckPassword()
        {
            var count = Password.Count(c => c == Chr);
            return count >= Min && count <= Max;
        }

        public bool CheckPassword2()
        {
            var x = Password[Min - 1] == Chr;
            var y = Password[Max - 1] == Chr;
            return x ^ y;
        }
    };

    public sealed class Day2 : Puzzle
    {
        public IEnumerable<PasswordCheck> Input { get; set; }
        public Day2()
        {
            Input = InputLines.Select(r =>
            {
                var parts = r.Split(" ");
                var minMax = parts[0];
                var chr = parts[1];
                var pass = parts[2];
                var minMaxInt = minMax.Split("-").Select(x => int.Parse(x)).ToArray();
                return new PasswordCheck(minMaxInt[0], minMaxInt[1], chr[0], pass);
            });
        }

        public override string Puzzle1() => Input.Count(pc => pc.CheckPassword()).ToString();

        public override string Puzzle2() => Input.Count(pc => pc.CheckPassword2()).ToString();
    }
}
