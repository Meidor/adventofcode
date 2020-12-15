using System.Linq;

namespace AOC2020
{
    public sealed class Day15 : Puzzle
    {
        public class MemoryGame
        {
            public (int last, int secondLast)[] memory;
            public readonly int[] startingNumbers;
            private readonly int rounds;
            public int turn = 1;
            public int LastSpoken { get; private set; } = -1;

            public MemoryGame(int[] startingNumbers, int rounds)
            {
                this.startingNumbers = startingNumbers;
                memory = new (int x, int y)[rounds];
                this.rounds = rounds;
                Reset();
            }

            public void Init()
            {
                for (var i = 0; i < startingNumbers.Length; i++)
                {
                    Speak(startingNumbers[i]);
                    turn++;
                }
            }

            public void Reset()
            {
                turn = 1;
                memory = new (int last, int secondLast)[rounds];
                Init();
            }

            public void Speak(int number)
            {
                if (memory[number] != (0, 0))
                {
                    memory[number].secondLast = memory[number].last;
                    memory[number].last = turn;
                }
                else
                {
                    memory[number] = (turn, -1);
                }
                LastSpoken = number;
            }

            public void Turn()
            {
                if (memory[LastSpoken].secondLast == -1)
                {
                    Speak(0);
                }
                else
                {
                    var (last, secondLast) = memory[LastSpoken];
                    Speak(last - secondLast);
                }
                turn++;
            }

            public void TakeTurns()
            {
                while (turn <= rounds)
                {
                    Turn();
                }
            }
        }

        private readonly int[] numbers;

        public Day15()
        {
            numbers = InputLines[0].Split(",").ParseInt().ToArray();
        }

        public Day15(int[] numbers)
        {
            this.numbers = numbers;
        }

        public override string Puzzle1()
        {
            var game = new MemoryGame(numbers, 2020);
            game.TakeTurns();
            return game.LastSpoken.ToString();
        }

        public override string Puzzle2()
        {
            var game = new MemoryGame(numbers, 30000000);
            game.TakeTurns();
            return game.LastSpoken.ToString();
        }
    }
}
