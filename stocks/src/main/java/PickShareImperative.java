import java.util.function.*;
import java.math.BigDecimal;

public class PickShareImperative {
    public static void pickShareImperative() {
        ShareInfo highPriced = new ShareInfo("", new BigDecimal("-1.0"));

        final Predicate isPriceLessThan500 = ShareUtil.isPriceLessThan(500);
        for(String symbol : Shares.symbols) {
            ShareInfo shareInfo = ShareUtil.getPrice(symbol);
            if(isPriceLessThan500.test(shareInfo))
                highPriced = ShareUtil.pickHigh(highPriced, shareInfo);
        }
        System.out.println("High priced under $500 is " + highPriced);
    }
}
