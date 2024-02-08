method SquareRoot(N:nat) returns (r:nat)
  ensures r*r <= N < (r+1)*(r+1)
{
  var guess := N/2;
  var improvedGuess := N/2;

  while (improvedGuess*improvedGuess > N) {
    guess := improvedGuess;
    improvedGuess := (guess + N/guess) / 2;
  }

  return guess;
}
