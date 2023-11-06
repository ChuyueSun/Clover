method rotate(a: array<int>, offset:int) returns (b: array<int> )
  requires 0<=offset<=a.Length
  ensures b.Length==a.Length
  ensures forall i ::0<=i<a.Length ==>  b[(i+offset)%a.Length]==a[i]
{
  //TOFILL
}
