public class Main {
    public static void main(String[] args) {
        // PickShareImperative.pickShareImperative();
      
        String res = PickShareFunctional.findHighPriced(Shares.symbols.stream());
        System.out.println(res);
    }
}
