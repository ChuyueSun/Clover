method PerfectSquaresBetween(start: int, end: int) returns (squares: seq<int>)
    requires start >= 0 && end >= start
    ensures forall i :: 0 <= i < |squares| ==> (exists j: int :: j * j == squares[i]) && start <= squares[i] <= end
    ensures forall i :: start <= i <= end && (exists j: int :: j * j == i) ==> i in squares
    {
        squares := [];
        var i := 0;
        while (i * i < start)
            invariant 0 <= i && i * i < start
        {
            i := i + 1;
        }
        while (i * i <= end)
            invariant start <= i * i <= end + 1
            invariant forall j :: 0 <= j < |squares| ==> (exists k: int :: k * k == squares[j]) && start <= squares[j] <= end
            invariant forall j :: start <= j <= end && (exists k: int :: k * k == j) ==> j in squares
        {
            squares := squares + [i * i];
            i := i + 1;
        }
    }