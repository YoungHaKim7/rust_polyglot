using System;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;

class Program
{
    const int NumberOfSteps = 100_000_000;

    static void Main()
    {
        Console.WriteLine("Function             | Elaspsed Time     | Estimated Pi");
        Console.WriteLine("--------------------------------------------------------");

        Time(SerialLinqPi, nameof(SerialLinqPi));
        Time(ParallelLinqPi, nameof(ParallelLinqPi));
        Time(SerialPi, nameof(SerialPi));
        Time(ParallelPi, nameof(ParallelPi));
        Time(ParallelPartitionerPi, nameof(ParallelPartitionerPi));

        Console.WriteLine("Press any key to continue....");
        Console.ReadLine();
    }

    static void Time(
            Func<double> estimatePi,
            string function)
    {
        var sw = Stopwatch.StartNew();
        var pi = estimatePi();
        Console.WriteLine($"{function.PadRight(22)} | {sw.Elapsed} | {pi}");
    }

    static double SerialLinqPi()
    {
        double step = 1.0 / (double)NumberOfSteps;
        return (from i in Enumerable.Range(0, NumberOfSteps)
                let x = (i + 0.5) * step
                select 4.0 / (1.0 + x * x)).Sum() * step;
    }

    static double ParallelLinqPi()
    {
        double step = 1.0 / (double)NumberOfSteps;
        return (from i in Enumerable.Range(0, NumberOfSteps)
                let x = (i + 0.5) * step
                select 4.0 / (1.0 + x * x)).Sum() * step;
    }

    static double SerialPi()
    {
        double sum = 0.0;
        double step = 1.0 / (double)NumberOfSteps;
        for (int i = 0; i < NumberOfSteps; i++)
        {
            double x = (i + 0.5) * step;
            sum += 4.0 / (1.0 + x * x);
        }
        return step * sum;
    }

    static double ParallelPi()
    {
        double sum = 0.0;
        double step = 1.0 / (double)NumberOfSteps;
        object monitor = new object();
        Parallel.For(0, NumberOfSteps, () => 0.0, (i, state, local) =>
        {
            double x = (i + 0.5) * step;
            return local + 4.0 / (1.0 + x * x);
        }, local => { lock (monitor) sum += local; });
        return step * sum;
    }

    static double ParallelPartitionerPi()
    {
        double sum = 0.0;
        double step = 1.0 / (double)NumberOfSteps;
        object monitor = new object();
        Parallel.ForEach(Partitioner.Create(0, NumberOfSteps), () => 0.0, (range, state, local) =>
        {
            for (int i = range.Item1; i < range.Item2; i++)
            {
                double x = (i + 0.5) * step;
                local += 4.0 / (1.0 + x * x);
            }
            return local;
        }, local => { lock (monitor) sum += local; });
        return step * sum;
    }
}

