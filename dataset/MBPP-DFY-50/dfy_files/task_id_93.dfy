method Power(a: int, b: nat) returns (result: int)
    ensures result == a ^^ b
{
    result := 1;
    var i := 0;
    while i < b
        invariant 0 <= i <= b
        invariant result == a ^^ i
    {
        result := result * a;
        i := i + 1;
    }
}