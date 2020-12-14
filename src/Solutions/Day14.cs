using AOC2020.Models;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;

namespace AOC2020
{
    public sealed class Day14 : Puzzle
    {
        public sealed class BitmaskSystem
        {
            private readonly Regex memRegex = new Regex("mem\\[([0-9]+)\\] = ([0-9]+)");
            private readonly Regex maskRegex = new Regex("mask = ([01X]+)");
            private readonly Dictionary<ulong, ulong> memory = new();

            public char[] Mask { get; set; } = Array.Empty<char>();

            public void ProcessInput(string[] input, bool version2 = false)
            {
                foreach (var line in input)
                {
                    var maskMatch = maskRegex.Match(line);
                    if (maskMatch.Success)
                    {
                        Mask = maskMatch.Groups[1].Value.ToCharArray();
                    }
                    var memMatch = memRegex.Match(line);
                    if (memMatch.Success)
                    {
                        var address = ulong.Parse(memMatch.Groups[1].Value);
                        var value = ulong.Parse(memMatch.Groups[2].Value);
                        if (version2)
                        {
                            SetAddressV2(address, value);
                        }
                        else
                        {
                            SetAddress(address, value);
                        }
                    }
                }
            }

            private void SetAddress(ulong address, ulong value)
            {
                memory[address] = ApplyBitmaskValue(value);
            }
            private void SetAddressV2(ulong address, ulong value)
            {
                foreach (var a in ApplyBitmaskAddress(address))
                {
                    memory[a] = value;
                }
            }

            private List<ulong> ApplyBitmaskAddress(ulong address)
            {
                char[] bits = Convert.ToString((long)address, 2).PadLeft(36, '0').ToCharArray();
                var result = new List<char[]> { bits };
                var clones = new List<char[]>();
                for (int i = 0; i < Mask.Length; i++)
                {
                    if (Mask[i] == '0')
                    {
                        continue;
                    }

                    if (Mask[i] == '1')
                    {
                        foreach (var r in result)
                        {
                            r[i] = '1';
                        }
                    }
                    else if (Mask[i] == 'X')
                    {
                        clones.Clear();
                        foreach (var r in result)
                        {
                            var clone = new char[36];
                            Array.Copy(r, clone, 36);
                            r[i] = '0';
                            clone[i] = '1';
                            clones.Add(clone);
                        }
                        result.AddRange(clones);
                    }
                }
                return result.ConvertAll(r => Convert.ToUInt64(new string(r), 2));
            }

            private ulong ApplyBitmaskValue(ulong value)
            {
                char[] bits = Convert.ToString((long)value, 2).PadLeft(36, '0').ToCharArray();
                for (int i = 0; i < Mask.Length; i++)
                {
                    if (Mask[i] == 'X')
                    {
                        continue;
                    }
                    bits[i] = Mask[i];
                }
                return Convert.ToUInt64(new string(bits), 2);
            }

            public ulong SumMemory() => memory.Values.Aggregate((x, y) => x + y);
        }

        public override string Puzzle1()
        {
            var system = new BitmaskSystem();
            system.ProcessInput(InputLines);
            return system.SumMemory().ToString();
        }

        public override string Puzzle2()
        {
            var system = new BitmaskSystem();
            system.ProcessInput(InputLines, true);
            return system.SumMemory().ToString();
        }
    }
}
