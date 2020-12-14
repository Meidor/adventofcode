using AOC2020.Helpers;
using System;
using System.Linq;
using System.Text;

namespace AOC2020
{
    public sealed class Day12 : Puzzle
    {
        public class Ship
        {
            private readonly Point upDown = (0, 1);
            private readonly Point leftRight = (1, 0);

            public enum Direction
            {
                North = 0,
                East = 90,
                South = 180,
                West = 270
            }

            public Point PositionShip { get; set; } = (0, 0);
            public Point PositionWaypoint { get; set; } = (10, 1);
            public int Facing { get; set; } = 90;
            public int ManhattanDistance => Math.Abs(PositionShip.X) + Math.Abs(PositionShip.Y);

            public void FollowInstructions(string[] instructions)
            {
                foreach (var instruction in instructions)
                {
                    var command = instruction[0];
                    var units = int.Parse(instruction[1..]);
                    switch (command)
                    {
                        case 'F':
                            ForwardShip(units);
                            break;
                        case 'L':
                            LeftShip(units);
                            break;
                        case 'R':
                            RightShip(units);
                            break;
                        case 'N':
                            NorthShip(units);
                            break;
                        case 'E':
                            EastShip(units);
                            break;
                        case 'S':
                            SouthShip(units);
                            break;
                        case 'W':
                            WestShip(units);
                            break;
                    }
                }
            }

            public void ForwardShip(int units)
            {
                switch ((Direction)Facing)
                {
                    case Direction.North:
                        NorthShip(units);
                        break;
                    case Direction.East:
                        EastShip(units);
                        break;
                    case Direction.South:
                        SouthShip(units);
                        break;
                    case Direction.West:
                        WestShip(units);
                        break;
                }
            }

            public void NorthShip(int units) => PositionShip += upDown * units;
            public void EastShip(int units) => PositionShip += leftRight * units;
            public void SouthShip(int units) => PositionShip -= upDown * units;
            public void WestShip(int units) => PositionShip -= leftRight * units;
            public void RightShip(int degrees) => Facing = (Facing + degrees) % 360;
            public void LeftShip(int degrees)
            {
                Facing -= degrees;
                if (Facing < 0)
                {
                    Facing += 360;
                }
            }

            public void FollowWaypointInstructions(string[] instructions)
            {
                foreach (var instruction in instructions)
                {
                    var command = instruction[0];
                    var units = int.Parse(instruction[1..]);
                    switch (command)
                    {
                        case 'F':
                            ForwardWaypoint(units);
                            break;
                        case 'L':
                            LeftWaypoint(units);
                            break;
                        case 'R':
                            RightWaypoint(units);
                            break;
                        case 'N':
                            NorthWaypoint(units);
                            break;
                        case 'E':
                            EastWaypoint(units);
                            break;
                        case 'S':
                            SouthWaypoint(units);
                            break;
                        case 'W':
                            WestWaypoint(units);
                            break;
                    }
                }
            }

            public void ForwardWaypoint(int units) => PositionShip += PositionWaypoint * units;

            public void RightWaypoint(int degrees)
            {
                switch (degrees)
                {
                    case 90:
                        PositionWaypoint = (PositionWaypoint.Y, -PositionWaypoint.X);
                        break;
                    case 180:
                        PositionWaypoint = (-PositionWaypoint.X, -PositionWaypoint.Y);
                        break;
                    case 270:
                        PositionWaypoint = (-PositionWaypoint.Y, PositionWaypoint.X);
                        break;
                }
            }

            public void LeftWaypoint(int degrees) => RightWaypoint(360 - degrees);
            public void NorthWaypoint(int units) => PositionWaypoint += upDown * units;
            public void EastWaypoint(int units) => PositionWaypoint += leftRight * units;
            public void SouthWaypoint(int units) => PositionWaypoint -= upDown * units;
            public void WestWaypoint(int units) => PositionWaypoint -= leftRight * units;
        }

        public override string Puzzle1()
        {
            var ship = new Ship();
            ship.FollowInstructions(InputLines);
            return ship.ManhattanDistance.ToString();
        }

        public override string Puzzle2()
        {
            var ship = new Ship();
            ship.FollowWaypointInstructions(InputLines);
            return ship.ManhattanDistance.ToString();
        }
    }
}
