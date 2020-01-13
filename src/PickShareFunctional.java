public class PickShareFunctional {
  
  public static String findHighPriced(Stream prices){
    return prices
            .map( (symbol) -> Pair<String, BigInteger>(symbol, APIFinance.getPrice(symbol)))
            .max((a, b) -> Math.max(a.getValue(), b.getValue())
            .getKey();
      
    }
  }
}
