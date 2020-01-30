import java.io.BufferedReader;
import java.math.BigDecimal;
import java.io.InputStreamReader;
import java.net.URL;
import java.net.URLConnection;
import java.io.IOException;


public class APIFinance {
    private static final String BASE_URL = "https://www.alphavantage.co/query?";
    private final static String apiKey = "X5FCG839K0RIP3DQ";

    public static BigDecimal getPrice(final String symbol) {
        BigDecimal price = new BigDecimal(0);
        try {
            URL url = new URL(BASE_URL + "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKey);
            URLConnection connection = url.openConnection();
            InputStreamReader inputStream = new InputStreamReader(connection.getInputStream(), "UTF-8");
            BufferedReader bufferedReader = new BufferedReader(inputStream);
            String line;
            while ((line = bufferedReader.readLine()) != null) {
                if (line.contains("price")) {
                    price = new BigDecimal(line.split("\"")[3].trim());
                }
			 }
            bufferedReader.close(); } catch (IOException e) {
                System.out.println("failure sending request");
            }
        return price;
    }
}
