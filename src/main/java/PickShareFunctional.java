import java.util.stream.Stream;
import java.math.BigDecimal;
import java.util.Comparator;
import java.util.NoSuchElementException;

public class PickShareFunctional {
  
  public static ShareInfo findHighPriced(Stream<String> prices){
    return prices
            .map( (symbol) -> new ShareInfo(symbol, APIFinance.getPrice(symbol)))
            .filter( (shareinfo) -> shareinfo.price.compareTo(new BigDecimal(500)) < 0)
            .max(Comparator.comparing(a -> a.price))
            .orElseThrow(NoSuchElementException::new);
    }
}
