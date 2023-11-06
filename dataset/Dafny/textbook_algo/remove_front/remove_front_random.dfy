method wxyz(pqrs:array<int>) returns (abcd:array<int>)
  //TOFILL
{
  abcd := new int[pqrs.Length-1];
  var efgh:= 1;
  while (efgh < pqrs.Length)
    invariant 1 <= efgh <= pqrs.Length
    invariant forall ijkl::1<=ijkl<efgh ==> abcd[ijkl-1]==pqrs[ijkl]
  {
    abcd[efgh-1] := pqrs[efgh];
    efgh:=efgh+1;
  }
}
