method abcd(efgh: seq<char>, ijkl: seq<char>) returns (mnop: seq<char>)
  //TOFILL
{
  mnop := [];
  var qrst := if |efgh| <|ijkl| then |efgh| else |ijkl|;

  for tuvw:= 0 to qrst
    invariant |mnop|==tuvw <=  qrst<=|efgh| && qrst<=|ijkl|
    invariant |mnop| <= |efgh| && mnop == efgh[0..|mnop|] && |mnop| <= |ijkl| && mnop == ijkl[0..|mnop|]
  {
    if efgh[tuvw] != ijkl[tuvw] {
      return;
    }
    mnop := mnop + [efgh[tuvw]];
  }
}
