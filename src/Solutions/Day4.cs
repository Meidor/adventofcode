using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace AOC2020
{
    public sealed class Day4 : Puzzle
    {
        public record Passport
        {
            private readonly Regex hairRegex = new("^#[0-9a-f]{6}$");
            private readonly Regex eyeColorRegex = new("^(amb|blu|brn|gry|grn|hzl|oth)$");
            private readonly Regex passportRegex = new("^[0-9]{9}$");

            public string? BirthYear { get; init; }
            public string? IssueYear { get; init; }
            public string? ExpirationYear { get; init; }
            public string? Height { get; init; }
            public string? HairColor { get; init; }
            public string? EyeColor { get; init; }
            public string? PassportId { get; init; }
            public string? CountryId { get; init; }

            private bool HeightValid()
            {
                if (Height == null) return false;

                if (!int.TryParse(Height[..^2], out int height))
                {
                    return false;
                }

                var unit = Height[^2..];
                if (unit == "in")
                {
                    return height >= 59 && height <= 76;
                }
                else if (unit == "cm")
                {
                    return height >= 150 && height <= 193;
                }
                return false;
            }

            private bool HairColorValid() => HairColor != null && hairRegex.IsMatch(HairColor);

            private bool EyeColorValid() => EyeColor != null && eyeColorRegex.IsMatch(EyeColor);

            public bool PassportIdValid() => PassportId != null && passportRegex.IsMatch(PassportId);

            public bool IsValid
                => IsComplete
                && int.TryParse(BirthYear, out int byr) && byr >= 1920 && byr <= 2002
                && int.TryParse(IssueYear, out int iyr) && iyr >= 2010 && iyr <= 2020
                && int.TryParse(ExpirationYear, out int eyr) && eyr >= 2020 && eyr <= 2030
                && HeightValid()
                && HairColorValid()
                && EyeColorValid()
                && PassportIdValid();

            public bool IsComplete
                => BirthYear != null
                && IssueYear != null
                && ExpirationYear != null
                && Height != null
                && HairColor != null
                && EyeColor != null
                && PassportId != null;
        }

        public static IEnumerable<Passport> ParsePassports(StreamReader puzzleReader)
        {
            string? byr = null;
            string? iyr = null;
            string? eyr = null;
            string? hgt = null;
            string? hcl = null;
            string? ecl = null;
            string? pid = null;
            string? cid = null;
            while (!puzzleReader.EndOfStream)
            {
                var line = puzzleReader.ReadLine();
                if (line!.Length == 0)
                {
                    yield return new Passport
                    {
                        BirthYear = byr,
                        IssueYear = iyr,
                        ExpirationYear = eyr,
                        Height = hgt,
                        HairColor = hcl,
                        EyeColor = ecl,
                        PassportId = pid,
                        CountryId = cid
                    };
                    byr = iyr = eyr = hgt = hcl = ecl = pid = cid = null;
                    continue;
                }

                foreach (var kv in line.Split(" "))
                {
                    var split = kv.Split(":");
                    var key = split[0];
                    var value = split[1];
                    switch (key)
                    {
                        case "byr":
                            byr = value;
                            break;
                        case "iyr":
                            iyr = value;
                            break;
                        case "eyr":
                            eyr = value;
                            break;
                        case "hgt":
                            hgt = value;
                            break;
                        case "hcl":
                            hcl = value;
                            break;
                        case "ecl":
                            ecl = value;
                            break;
                        case "pid":
                            pid = value;
                            break;
                        case "cid":
                            cid = value;
                            break;
                    }
                }
            }
            yield return new Passport
            {
                BirthYear = byr,
                IssueYear = iyr,
                ExpirationYear = eyr,
                Height = hgt,
                HairColor = hcl,
                EyeColor = ecl,
                PassportId = pid,
                CountryId = cid
            };
            puzzleReader.Rewind();
        }

        public override string Puzzle1()
        {
            var passports = ParsePassports(PuzzleReader);
            return $"{passports.Count(p => p.IsComplete)}";
        }

        public override string Puzzle2()
        {
            var passports = ParsePassports(PuzzleReader);
            return $"{passports.Count(p => p.IsValid)}";
        }
    }
}
