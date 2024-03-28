method FindUniqueElement(arr: array<int>) returns (found: bool, uniqueElement: int)
    requires arr != null
    requires forall i, j :: 0 <= i < j < arr.Length ==> arr[i] <= arr[j]
    ensures !found ==> forall x :: x in set arr[..] ==> |filter(y => y == x, arr[..])| != 1
    ensures found ==> |filter(y => y == uniqueElement, arr[..])| == 1
{
    found := false;
    uniqueElement := 0;
    var i := 0;
    while (i < arr.Length)
        invariant 0 <= i <= arr.Length
        invariant !found ==> forall x :: x in set arr[..i] ==> |filter(y => y == x, arr[..i])| != 1
        invariant found ==> |filter(y => y == uniqueElement, arr[..i])| == 1
    {
        var count := 1;
        while (i + count < arr.Length && arr[i + count] == arr[i])
            invariant i <= i + count <= arr.Length
            invariant count >= 1
            invariant forall k :: 0 <= k < count ==> arr[i + k] == arr[i]
        {
            count := count + 1;
        }
        if (count == 1)
        {
            found := true;
            uniqueElement := arr[i];
            return;
        }
        i := i + count;
    }
}