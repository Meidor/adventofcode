using System;
using System.Linq;

namespace AOC2019
{
    public class Day2 : Puzzle
    {
        private const int SOLUTION = 19690720;
        private int[] program;
        private int[] Program => (int[])program.Clone();

        public Day2() : base()
        {
            program = InputLines.First().Split(",").ParseInt().ToArray();
        }
        public override string Puzzle1()
        {
            int[] program = Program;
            program[1] = 12;
            program[2] = 2;
            var output = RunProgram(program);
            return program[0].ToString();
        }
        public override string Puzzle2()
        {
            int noun = 0;
            int verb = 0;
            var program = Program;
            for (noun = 0; noun < 100; noun++)
            {
                for (verb = 0; verb < 100; verb++)
                {
                    try
                    {
                        program[1] = noun;
                        program[2] = verb;
                        var output = RunProgram(program);
                        if (output[0] == SOLUTION)
                        {
                            return (100 * noun + verb).ToString();
                        }
                    }
                    catch
                    {
                        continue;
                    }
                }
            }
            throw new Exception("Solution not found");
        }

        internal static int[] RunProgram(int[] program)
        {
            var output = (int[])program.Clone();
            var pointer = 0;
            var execute = true;
            while (execute)
            {
                var instruction = output[pointer];
                switch (instruction)
                {
                    case 1:
                        var positions = GetPositions(pointer + 1, output);
                        output[positions.output] = output[positions.input1] + output[positions.input2];
                        pointer += 4;
                        break;
                    case 2:
                        var positions2 = GetPositions(pointer + 1, output);
                        output[positions2.output] = output[positions2.input1] * output[positions2.input2];
                        pointer += 4;
                        break;
                    case 99:
                        execute = false;
                        break;
                    default:
                        throw new NotSupportedException();
                }
            }
            return output;
        }

        internal static (int input1, int input2, int output) GetPositions(int pointer, int[] program)
        {
            return (program[pointer], program[pointer + 1], program[pointer + 2]);
        }
    }
}