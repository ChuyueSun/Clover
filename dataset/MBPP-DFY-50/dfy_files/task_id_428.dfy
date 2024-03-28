method ShellSort(a: array<int>)
    requires a != null
    modifies a
    ensures a.Length == old(a.Length)
    ensures IsSorted(a)
    ensures multiset(a[..]) == multiset(old(a[..]))
{
    var n := a.Length;
    var h := n / 2;
    while h > 0
        decreases h
        invariant 0 <= h <= n
        invariant h == 0 ==> IsSorted(a)
        invariant multiset(a[..]) == multiset(old(a[..]))
    {
        for i := h to n - 1
            decreases n - i
            invariant h <= i < n
            invariant IsHSort(a, h, i)
            invariant multiset(a[..]) == multiset(old(a[..]))
        {
            var j := i;
            while j >= h && a[j - h] > a[j]
                decreases j
                invariant h <= j < n
                invariant a[j - h] > a[j] ==> IsHSort(a, h, j - 1)
                invariant multiset(a[..]) == multiset(old(a[..]))
            {
                var temp := a[j];
                a[j] := a[j - h];
                a[j - h] := temp;
                j := j - h;
            }
        }
        h := h / 2;
    }
}

predicate IsSorted(a: array<int>)
{
    forall i, j :: 0 <= i < j < a.Length ==> a[i] <= a[j]
}

predicate IsHSort(a: array<int>, h: int, until: int)
{
    forall i, j :: 0 <= i < j <= until && j % h == 0 ==> a[i] <= a[j]
}