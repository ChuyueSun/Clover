method CalDiv() returns (x:int, y:int)
  ensures x==191/7
  ensures y==191%7
{
  var dividend := 191;
  var divisor := 7;
  var quotient := 0;
  var remainder := 0;

  while (dividend >= divisor)
    invariant dividend >= 0
    invariant remainder >= 0 && remainder < divisor
    invariant dividend == quotient * divisor + remainder
  {
    dividend := dividend - divisor;
    quotient := quotient + 1;
  }

  remainder := dividend;

  x := quotient;
  y := remainder;
}
