method FindLargestProducts(a: array<int>, b: array<int>, n: int) returns (result: array<int>)
    requires a != null && b != null
    requires a.Length == b.Length
    requires 0 <= n <= a.Length
    ensures result != null
    ensures result.Length == n
    ensures forall i :: 0 <= i < result.Length ==> 
        exists x, y :: 0 <= x < a.Length && 0 <= y < b.Length && result[i] == a[x] * b[y]
    ensures forall i :: 0 <= i < result.Length ==> 
        forall x, y :: 0 <= x < a.Length && 0 <= y < b.Length ==> 
            a[x] * b[y] <= result[i] ==> exists j :: 0 <= j < i && result[j] == a[x] * b[y]
{
    var products := new array<int>[a.Length * b.Length];
    var k := 0;
    for i := 0 to a.Length
        invariant 0 <= i <= a.Length
        invariant k == i * b.Length
        invariant forall x, y :: 0 <= x < i && 0 <= y < b.Length ==> 
            exists j :: 0 <= j < k && products[j] == a[x] * b[y]
    {
        for j := 0 to b.Length
            invariant 0 <= j <= b.Length
            invariant k == i * b.Length + j
            invariant forall y :: 0 <= y < j ==> exists l :: 0 <= l < k && products[l] == a[i] * b[y]
        {
            products[k] := a[i] * b[j];
            k := k + 1;
        }
    }
    result := new array<int>[n];
    for i := 0 to n
        invariant 0 <= i <= n
        invariant forall j :: 0 <= j < i ==> 
            exists x, y :: 0 <= x < a.Length && 0 <= y < b.Length && result[j] == a[x] * b[y]
        invariant forall j :: 0 <= j < i ==> 
            forall x, y :: 0 <= x < a.Length && 0 <= y < b.Length ==> 
                a[x] * b[y] <= result[j] ==> exists l :: 0 <= l < j && result[l] == a[x] * b[y]
    {
        var maxProduct := products[0];
        var maxIndex := 0;
        for j := 1 to products.Length
            invariant 1 <= j <= products.Length
            invariant exists x :: 0 <= x < j && maxProduct == products[x]
        {
            if products[j] > maxProduct {
                maxProduct := products[j];
                maxIndex := j;
            }
        }
        result[i] := maxProduct;
        products[maxIndex] := products[products.Length - 1];
        products := products[..products.Length - 1];
    }
}