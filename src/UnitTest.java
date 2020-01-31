import static org.junit.Assert.assertEquals;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.mockito.Mockito;
import org.powermock.api.mockito.PowerMockito;
import org.powermock.core.classloader.annotations.PrepareForTest;
import org.powermock.modules.junit4.PowerMockRunner;

import java.math.BigDecimal;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;


@RunWith(PowerMockRunner.class)
@PrepareForTest(APIFinance.class)
public class UnitTest {

    @Test
    public void testFunctional() {
        PowerMockito.mockStatic(APIFinance.class);


        String[] stocks = {"IBM","AAPL","AMZN","CSCO","SNE", "GOOG","MSFT","ORCL","FB","VRSN"};
        HashMap<String, BigDecimal> m = new HashMap<>();

        for (int i=0; i <10; i+=1) {
            m.put(stocks[i], new BigDecimal(Math.random()));
            Mockito.when(APIFinance.getPrice(stocks[i])).thenReturn(m.get(stocks[i]));
        }

        assertEquals(
                Collections.max(m.entrySet(), Map.Entry.comparingByValue()).getKey()
                , PickShareFunctional.findHighPriced(Shares.symbols.stream())
        );
        assertEquals(
                Collections.max(m.entrySet(), Map.Entry.comparingByValue()).getKey()
                , PickShareFunctional.findHighPriced(Shares.symbols.parallelStream())
        );

    }

}