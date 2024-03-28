method ContainsNoDuplicates(seq: seq<int>) returns (result: bool)
    ensures result <==> (forall i, j :: 0 <= i < j < |seq| ==> seq[i] != seq[j])
{
    result := true;
    for i := 0 to |seq|
        invariant 0 <= i <= |seq|
        invariant result <==> (forall k, l :: 0 <= k < l < i ==> seq[k] != seq[l])
    {
        for j := i + 1 to |seq|
            invariant i + 1 <= j <= |seq|
            invariant result <==> (forall k :: i + 1 <= k < j ==> seq[i] != seq[k])
        {
            if seq[i] == seq[j] {
                result := false;
                return;
            }
        }
    }
}