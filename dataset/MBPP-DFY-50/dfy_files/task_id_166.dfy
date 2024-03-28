predicate IsEven(n: int)
{
    n % 2 == 0
}

method CountXorEvenPairs(a: array<int>) returns (count: int)
    requires a != null
    ensures count >= 0
    ensures count == (seq{ i,j | i in 0..a.Length, j in i+1..a.Length && IsEven(a[i] ^ a[j]) }).Length
{
    count := 0;
    for i := 0 to a.Length
        invariant 0 <= i <= a.Length
        invariant count == (seq{ k,l | k in 0..i, l in k+1..i && IsEven(a[k] ^ a[l]) }).Length
    {
        for j := i+1 to a.Length
            invariant i < j <= a.Length
            invariant count == (seq{ k,l | k in 0..i, l in k+1..j && IsEven(a[k] ^ a[l]) }).Length
        {
            if IsEven(a[i] ^ a[j])
            {
                count := count + 1;
            }
        }
    }
}