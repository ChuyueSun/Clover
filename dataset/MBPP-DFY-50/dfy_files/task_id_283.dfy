method IsFrequencyLessOrEqual(n: int) returns (result: bool)
    requires n >= 0
    ensures result ==> (forall i :: 0 <= i < 10 ==> (n.ToString().CountOf(i.ToString()) <= i))
    ensures !result ==> (exists i :: 0 <= i < 10 && (n.ToString().CountOf(i.ToString()) > i))
{
    result := true;
    var i := 0;
    while i < 10
        invariant 0 <= i <= 10
        invariant result ==> (forall k :: 0 <= k < i ==> (n.ToString().CountOf(k.ToString()) <= k))
        invariant !result ==> (exists k :: 0 <= k < i && (n.ToString().CountOf(k.ToString()) > k))
    {
        if n.ToString().CountOf(i.ToString()) > i {
            result := false;
            break;
        }
        i := i + 1;
    }
}