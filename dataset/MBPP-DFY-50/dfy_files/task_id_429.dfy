method ElementWiseAndSeq(a: seq<int>, b: seq<int>) returns (result: seq<int>)
    requires |a| == |b|
    ensures |result| == |a|
    ensures forall i :: 0 <= i < |result| ==> result[i] == a[i] & b[i]
{
    var res := [];
    for i := 0 to |a|
        invariant 0 <= i <= |a|
        invariant |res| == i
        invariant forall k :: 0 <= k < i ==> res[k] == a[k] & b[k]
    {
        res := res + [a[i] & b[i]];
    }
    return res;
}