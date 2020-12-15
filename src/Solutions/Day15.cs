using AOC2020.Helpers;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;

namespace AOC2020
{
    public sealed class Day15 : Puzzle
    {
        public class MemoryGame
        {
            public readonly Dictionary<int, int[]> memoryGame = new();
            public readonly int[] startingNumbers;
            public int turn = 1;
            public int LastSpoken { get; private set; } = -1;

            public MemoryGame(int[] startingNumbers)
            {
                this.startingNumbers = startingNumbers;
                Reset();
            }

            public void Init()
            {
                for(var i = 0; i < startingNumbers.Length; i++)
                {
                    Speak(startingNumbers[i]);
                    turn++;
                }
            }

            public void Reset()
            {
                turn = 1;
                memoryGame.Clear();
                Init();
            }

            public void Speak(int number)
            {
                if (memoryGame.ContainsKey(number))
                {
                    memoryGame[number][1] = memoryGame[number][0];
                    memoryGame[number][0] = turn;
                }
                else
                {
                    memoryGame[number] = new int[2] { turn, -1 };
                }
                LastSpoken = number;
            }

            public void Turn()
            {
                if(memoryGame[LastSpoken][1] == -1)
                {
                    Speak(0);
                }
                else
                {
                    var mem = memoryGame[LastSpoken];
                    var x = mem[0];
                    var y = mem[1];
                    Speak(x - y);
                }
                turn++;
            }

            public void TakeTurnsTill(int round)
            {
                while(turn <= round)
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
            var game = new MemoryGame(numbers);
            game.TakeTurnsTill(2020);
            return game.LastSpoken.ToString();
        }

        public override string Puzzle2()
        {
            var game = new MemoryGame(numbers);
            game.TakeTurnsTill(30000000);
            return game.LastSpoken.ToString();
        }
    }
}
