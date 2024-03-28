method SetEvenBits(n: int) returns (result: int)
    requires n >= 0
    ensures result >= n
    ensures forall i :: 0 <= i < 32 && i % 2 == 0 ==> (result / 2^i) % 2 == 1
    ensures forall i :: 0 <= i < 32 && i % 2 == 1 ==> (result / 2^i) % 2 == (n / 2^i) % 2
{
    result := n;
    var mask := 0xAAAAAAAA; // binary: 10101010..., sets all even bits to 1
    result := result | mask;
}