method MaxSubArraySum(a: array<int>) returns (maxSum: int)
    requires a.Length > 0
    ensures maxSum >= a[Max(a[..])]
    ensures exists i, j :: 0 <= i <= j < a.Length && maxSum == Sum(a[i..j+1])
{
    var currentSum := a[0];
    maxSum := a[0];

    for i := 1 to a.Length
        invariant 0 <= i <= a.Length
        invariant maxSum >= a[Max(a[..i])]
        invariant exists k, l :: 0 <= k <= l < i && maxSum == Sum(a[k..l+1])
    {
        currentSum := Max(currentSum + a[i], a[i]);
        maxSum := Max(maxSum, currentSum);
    }
}

function Max(a: seq<int>): int
    requires |a| > 0
{
    if |a| == 1 then a[0]
    else Max(a[..|a|-1]) >? a[|a|-1]
}

function Sum(a: seq<int>): int
{
    if |a| == 0 then 0
    else a[0] + Sum(a[1..])
}