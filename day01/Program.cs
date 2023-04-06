List<int> l = new() { 0 };

foreach (var line in File.ReadLines("input01.txt"))
{
    if (string.IsNullOrEmpty(line))
    {
        l.Add(0);
    }
    else
    {
        l[l.Count - 1] += int.Parse(line);
    }
}

Console.WriteLine(l.Max());

// Part 2
l.Sort();
l.Reverse();
var top3 = l.Take(3);

foreach(var t in top3)
{
    Console.WriteLine(t);
}

Console.WriteLine(top3.Sum());
