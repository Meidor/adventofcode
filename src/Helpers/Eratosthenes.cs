using System;
using System.Collections.Generic;
using System.Collections;

namespace AOC2020.Helpers
{
    public class Eratosthenes : IEnumerable<int>
    {
        private static readonly List<int> primes = new List<int>();
        private int lastChecked;

        public Eratosthenes()
        {
            primes.Add(2);
            lastChecked = 2;
        }

        private static bool IsPrime(int checkValue)
        {
            bool isPrime = true;

            foreach (int prime in primes)
            {
                if ((checkValue % prime) == 0 && prime <= Math.Sqrt(checkValue))
                {
                    isPrime = false;
                    break;
                }
            }

            return isPrime;
        }

        public IEnumerator<int> GetEnumerator()
        {
            foreach (int prime in primes)
                yield return prime;

            while (lastChecked < int.MaxValue)
            {
                lastChecked++;

                if (IsPrime(lastChecked))
                {
                    primes.Add(lastChecked);
                    yield return lastChecked;
                }
            }
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return GetEnumerator();
        }
    }
}
