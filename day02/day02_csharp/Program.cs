
{ // Part 1


    // (1 for Rock, 2 for Paper, and 3 for Scissors)
    var scores = new Dictionary<string, int>(){
    { "A X", 1 + 3 },
    { "B X", 1 + 0 },
    { "C X", 1 + 6 },
    { "A Y", 2 + 6},
    { "B Y", 2 + 3},
    { "C Y", 2 + 0},
    { "A Z", 3 + 0},
    { "B Z", 3 + 6},
    { "C Z", 3 + 3},
};


    var total = 0;
    foreach (var line in File.ReadLines("../input02.txt"))
    {
        total += scores[line];
    }

    Console.WriteLine(total);
}

// Part 2 
{
    // (1 for Rock, 2 for Paper, and 3 for Scissors)
    var scores = new Dictionary<string, int>(){
    { "A X", 3 + 0},
    { "B X", 1 + 0},
    { "C X", 2 + 0},
    { "A Y", 1 + 3},
    { "B Y", 2 + 3},
    { "C Y", 3 + 3},
    { "A Z", 2 + 6},
    { "B Z", 3 + 6},
    { "C Z", 1 + 6},
};


    var total = 0;
    foreach (var line in File.ReadLines("../input02.txt"))
    {
        total += scores[line];
    }

    Console.WriteLine(total);
}