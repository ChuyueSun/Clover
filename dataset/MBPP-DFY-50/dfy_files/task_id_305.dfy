method GetTwoPWords(words: seq<string>) returns (result: seq<string>)
    ensures |result| <= 2
    ensures forall w :: w in result ==> w != "" && (w[0] == 'p' || w[0] == 'P')
    ensures forall w :: w in result ==> w in words
    ensures forall w :: w in words && w != "" && (w[0] == 'p' || w[0] == 'P') ==> w in result
{
    result := [];
    for w in words
        invariant |result| <= 2
        invariant forall x :: x in result ==> x != "" && (x[0] == 'p' || x[0] == 'P')
        invariant forall x :: x in result ==> x in words
    {
        if |result| == 2 {
            break;
        }
        if w != "" && (w[0] == 'p' || w[0] == 'P') {
            result := result + [w];
        }
    }
}