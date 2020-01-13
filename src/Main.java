public class Main {
    public static void main(String[] args) {
        // PickShareImperative.pickShareImperative();
      
        long start = System.nanoTime();
        PickShareImperative.pickShareImperative();
        double time = (System.nanoTime() - start)/ 1000000000.0;
        System.out.println("Imparative: " + time);
        
        start = System.nanoTime();
        String res = PickShareFunctional.findHighPriced(Shares.symbols.stream());
        time = (System.nanoTime() - start)/ 1000000000.0;
        System.out.println("Functional: " + time);

        start = System.nanoTime();
        res = PickShareFunctional.findHighPriced(Shares.symbols.parallelStream());
        time = (System.nanoTime() - start)/ 1000000000.0;
        System.out.println("Parallel + Functional: " + time);
        
        System.out.println(res);
    }
}
