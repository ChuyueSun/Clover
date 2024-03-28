method ContainsElement(seq: seq<int>, element: int) returns (result: bool)
    ensures result <==> element in seq
{
    result := false;
    for i := 0 to |seq|
        invariant 0 <= i <= |seq|
        invariant result <==> exists j :: 0 <= j < i && seq[j] == element
    {
        if seq[i] == element {
            result := true;
            break;
        }
    }
}