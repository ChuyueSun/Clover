method RemoveKLengthWords(s: string, k: int) returns (v: string)
    requires k > 0
    ensures forall w :: w in words(s) && |w| == k ==> w !in words(v)
    ensures forall w :: w in words(s) && |w| != k ==> w in words(v)
{
    var words := s.Split(' ');
    var result := "";
    var isFirst := true;

    for word in words
    {
        if |word| != k
        {
            if !isFirst
            {
                result := result + " ";
            }
            result := result + word;
            isFirst := false;
        }
    }

    v := result;
}