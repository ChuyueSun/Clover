predicate IsUpperCase(c: char)
{
    65 <= c as int <= 90
}

method MaxRunUppercase(s: string) returns (maxRun: int)
    ensures maxRun >= 0
    ensures (exists i, j :: 0 <= i < j <= |s| && (forall k :: i <= k < j ==> IsUpperCase(s[k])) && j - i == maxRun) || (maxRun == 0 && forall i :: 0 <= i < |s| ==> !IsUpperCase(s[i]))
{
    var currentRun := 0;
    maxRun := 0;

    for i := 0 to |s|
        invariant 0 <= i <= |s|
        invariant 0 <= currentRun <= i
        invariant (exists ii, jj :: 0 <= ii < jj <= i && (forall k :: ii <= k < jj ==> IsUpperCase(s[k])) && jj - ii == maxRun) || (maxRun == 0 && forall ii :: 0 <= ii < i ==> !IsUpperCase(s[ii]))

    {
        if IsUpperCase(s[i]) {
            currentRun := currentRun + 1;
            if currentRun > maxRun {
                maxRun := currentRun;
            }
        }
        else {
            currentRun := 0;
        }
    }
}