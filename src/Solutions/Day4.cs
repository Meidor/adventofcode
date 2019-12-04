using System;
using System.Linq;

namespace AOC2019
{
    public class Day4 : Puzzle
    {
        public override string Puzzle1()
        {
            var range = InputLines.First().Split("-").ParseInt().ToList();
            var possiblePasswords = Enumerable.Range(range[0], range[1] - range[0]);
            return possiblePasswords
                .Where(IsValidPassword)
                .Count()
                .ToString();
        }

        public override string Puzzle2()
        {
            var range = InputLines.First().Split("-").ParseInt().ToList();
            var possiblePasswords = Enumerable.Range(range[0], range[1] - range[0]);
            return possiblePasswords
                .Where(IsValidPassword2)
                .Count()
                .ToString();
        }

        internal static bool IsValidPassword(int attempt)
        {
            var pass = attempt.ToString();
            var previousChar = pass[0];
            var hasDouble = false;
            for (int i = 1; i < pass.Length; i++)
            {
                var currentChar = pass[i];
                if (currentChar < previousChar)
                {
                    return false;
                }
                if (currentChar == previousChar)
                {
                    hasDouble = true;
                }
                previousChar = currentChar;
            }
            return hasDouble;
        }

        internal static bool IsValidPassword2(int attempt)
        {
            var pass = attempt.ToString();
            var previousChar = pass[0];
            var hasDouble = false;
            var repeatLength = 0;
            for (int i = 1; i < pass.Length; i++)
            {
                var currentChar = pass[i];
                if (currentChar < previousChar)
                {
                    return false;
                }
                if (currentChar == previousChar)
                {
                    repeatLength++;
                }
                else if (repeatLength == 1)
                {
                    hasDouble = true;
                }
                else
                {
                    repeatLength = 0;
                }
                previousChar = currentChar;
            }
            return hasDouble || repeatLength == 1;
        }
    }
}