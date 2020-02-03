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

    final static String[] stocks = {"IBM","AAPL","AMZN","CSCO","SNE", "GOOG","MSFT","ORCL","FB","VRSN"};

    public void runTest(HashMap<String, BigDecimal> data, String expected){
        PowerMockito.mockStatic(APIFinance.class);
        data.forEach((k,v)->{
            Mockito.when(APIFinance.getPrice(k)).thenReturn(v);
        });
        assertEquals(
                expected,
                PickShareFunctional.findHighPriced(Shares.symbols.stream()).symbol
        );
                assertEquals(
                expected
                , PickShareFunctional.findHighPriced(Shares.symbols.parallelStream()).symbol
        );
    }

    @Test
    public void testFunctional() {

        HashMap<String, BigDecimal> data = new HashMap<>();
        data.put(stocks[0], new BigDecimal(500));
        data.put(stocks[1], new BigDecimal(1000));
        data.put(stocks[2], new BigDecimal(400));
        data.put(stocks[3], new BigDecimal(200));
        data.put(stocks[4], new BigDecimal(350));
        data.put(stocks[5], new BigDecimal(100));
        data.put(stocks[6], new BigDecimal(250));
        data.put(stocks[7], new BigDecimal(150));
        data.put(stocks[8], new BigDecimal(499));
        data.put(stocks[9], new BigDecimal(10));


        runTest(data, stocks[8]);
    }

}