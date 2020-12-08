using System;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace AOC2020
{
    public sealed class Day8 : Puzzle
    {
        public static class AOComputer
        {
            public enum Opcode
            {
                acc = 0,
                jmp = 1,
                nop = 2
            }

            public record Instruction(Opcode Opcode, int Number);

            public static Instruction[] ParseProgram(string[] input)
            {
                Instruction[] program = new Instruction[input.Length];
                var re = new Regex("([a-z]+) ((?:\\+|-)[0-9]+)");
                for (int i = 0; i < input.Length; i++)
                {
                    var instruction = input[i];
                    var match = re.Match(instruction);
                    var opcode = (Opcode)Enum.Parse(typeof(Opcode), match.Groups[1].Value);
                    var number = int.Parse(match.Groups[2].Value);
                    program[i] = new Instruction(opcode, number);
                }
                return program;
            }

            public static bool TryRun(Instruction[] program, out int accumulatorResult)
            {
                int accumulator = 0;
                int ip = 0;
                HashSet<int> trace = new HashSet<int>();
                while (true)
                {
                    if (ip == program.Length)
                    {
                        accumulatorResult = accumulator;
                        return true;
                    }
                    if (ip > program.Length)
                    {
                        accumulatorResult = accumulator;
                        return false;
                    }

                    var instruction = program[ip];
                    if (trace.Contains(ip))
                    {
                        accumulatorResult = accumulator;
                        return false;
                    }

                    trace.Add(ip);
                    switch (instruction.Opcode)
                    {
                        case Opcode.acc:
                            accumulator += instruction.Number;
                            ip++;
                            break;
                        case Opcode.jmp:
                            ip += instruction.Number;
                            break;
                        case Opcode.nop:
                            ip++;
                            break;
                    }
                }
            }

            public static int BruteForce(string[] input)
            {
                var initialProgram = ParseProgram(input);

                for (int i = 0; i < initialProgram.Length; i++)
                {
                    var instruction = initialProgram[i];
                    Instruction[] program;
                    switch (instruction.Opcode)
                    {
                        case Opcode.jmp:
                            program = (Instruction[])initialProgram.Clone();
                            program[i] = instruction with { Opcode = Opcode.nop };
                            break;
                        case Opcode.nop:
                            program = (Instruction[])initialProgram.Clone();
                            program[i] = instruction with { Opcode = Opcode.jmp };
                            break;
                        default:
                            continue;
                    }
                    if (TryRun(program, out var result))
                    {
                        return result;
                    }
                }
                throw new InvalidOperationException("Should not be here...");
            }
        }

        public override string Puzzle1()
        {
            AOComputer.TryRun(AOComputer.ParseProgram(InputLines), out var result);
            return result.ToString();
        }

        public override string Puzzle2() => AOComputer.BruteForce(InputLines).ToString();
    }
}
