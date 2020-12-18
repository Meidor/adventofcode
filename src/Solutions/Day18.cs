using System;
using System.Linq;
using System.Collections.Generic;
using System.Text;

namespace AOC2020
{
    public sealed class Day18 : Puzzle
    {
        public enum TokenType
        {
            Integer,
            Add,
            Multiply,
            OpenParentheses,
            CloseParantheses,
        }

        public record Token(TokenType Type);
        public record NumberToken(TokenType Type, int Value) : Token(Type);

        public static class Lexer
        {
            private static NumberToken GetNumberToken(string input, ref int pos)
            {
                var sb = new StringBuilder();
                do
                {
                    sb.Append(input[pos]);
                    pos++;
                } while (pos < input.Length && int.TryParse(input[pos].ToString(), out _));
                pos--;
                return new NumberToken(TokenType.Integer, int.Parse(sb.ToString()));
            }
            public static IEnumerable<Token> Tokenize(string input)
            {
                for (var pos = 0; pos < input.Length; pos++)
                {
                    Token? token = input[pos] switch
                    {
                        '(' => new Token(TokenType.OpenParentheses),
                        ')' => new Token(TokenType.CloseParantheses),
                        '+' => new Token(TokenType.Add),
                        '*' => new Token(TokenType.Multiply),
                        >= '0' and <= '9' => GetNumberToken(input, ref pos),
                        _ => null
                    };
                    if (token is not null)
                    {
                        yield return token;
                    }
                }
            }
        }

        public abstract class Calculator
        {
            protected readonly IEnumerator<Token> tokens;
            protected Calculator(string input)
            {
                tokens = Lexer.Tokenize(input).GetEnumerator();
            }

            protected long Factor()
            {
                var token = tokens.Current;
                if (token.Type == TokenType.Integer)
                {
                    Expect(TokenType.Integer);
                    return ((NumberToken)token).Value;
                }
                else if (token.Type == TokenType.OpenParentheses)
                {
                    Expect(TokenType.OpenParentheses);
                    var result = Expression();
                    Expect(TokenType.CloseParantheses);
                    return result;
                }
                throw new InvalidOperationException("Shouldn't be here...");
            }

            protected bool Accept(TokenType type)
            {
                var match = tokens.Current.Type == type;
                if (match)
                {
                    tokens.MoveNext();
                }
                return match;
            }

            protected bool Expect(TokenType type)
            {
                if (Accept(type))
                {
                    return true;
                }
                throw new InvalidOperationException($"Token type {tokens.Current.Type} doesn't match expected type {type}.");
            }

            protected abstract long Expression();

            public long Solve()
            {
                tokens.MoveNext();
                return Expression();
            }
        }

        public sealed class CalculatorOne : Calculator
        {
            public CalculatorOne(string input) : base(input)
            {
            }

            protected override long Expression()
            {
                long result = Factor();
                while (tokens.Current is not null && (tokens.Current.Type == TokenType.Add || tokens.Current.Type == TokenType.Multiply))
                {
                    var type = tokens.Current.Type;
                    Expect(type);
                    result = type switch
                    {
                        TokenType.Add => result + Factor(),
                        TokenType.Multiply => result * Factor(),
                        _ => result
                    };
                }
                return result;
            }
        }

        public sealed class CalculatorTwo : Calculator
        {
            public CalculatorTwo(string input) : base(input)
            {
            }

            private long Term()
            {
                var result = Factor();
                while (tokens.Current?.Type == TokenType.Add)
                {
                    Expect(TokenType.Add);
                    result += Factor();
                }
                return result;
            }

            protected override long Expression()
            {
                long result = Term();
                while (tokens?.Current.Type == TokenType.Multiply)
                {
                    Expect(TokenType.Multiply);
                    result *= Term();
                }
                return result;
            }
        }

        public override string Puzzle1() => InputLines.Select(input => new CalculatorOne(input).Solve()).Sum().ToString();

        public override string Puzzle2() => InputLines.Select(input => new CalculatorTwo(input).Solve()).Sum().ToString();
    }
}
