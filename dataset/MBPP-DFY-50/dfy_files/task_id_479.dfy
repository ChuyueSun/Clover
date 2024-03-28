method FirstDigit(n: int) returns (d: int)
    requires n > 0
    ensures 1 <= d <= 9
    ensures d == n / Power10(NumberOfDigits(n) - 1)
{
    var m := n;
    while m >= 10
        invariant m >= d
        invariant m >= 10
    {
        m := m / 10;
    }
    d := m;
}

function Power10(n: nat): (r: nat)
    ensures r >= 1
    ensures n > 0 ==> r % 10 == 0
{
	if (n == 0) then 1 else 10 * Power10(n-1)
}

function NumberOfDigits(n: nat) : (r: nat)
    ensures r >= 1
    ensures r == 1 <==> 0 <= n <= 9
{
    if 0 <= n <= 9 then 1 else 1+NumberOfDigits(n/10)
}