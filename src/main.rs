use fast_math::log2;

fn pr_k() -> f32 {
    1.0/3.0
}

fn z2(){
    let P = vec!['a', 'b', 'c'];
    let pr_a = 1.0/2.0;
    let pr_b = 1.0/3.0;
    let pr_c = 1.0/6.0;

    let H_P = -pr_a * log2(pr_a) + -pr_b * log2(pr_b) + -pr_c * log2(pr_c);
    println!("H(P): {}", H_P);

    let pr_1 = (pr_a + pr_c) * pr_k();
    let pr_2 = (pr_a + pr_c) * pr_k();
    let pr_3 = (pr_a + pr_b + pr_c) * pr_k();
    let pr_4 = (pr_b + pr_c) * pr_k();

    let H_C = - pr_1 * log2(pr_1) + -pr_2 * log2(pr_2) + -pr_3 * log2(pr_3) + -pr_4 * log2(pr_4);
    println!("H(C) = {}", H_C);


    let H_K = - pr_k() * log2(pr_k()) * 3.0;
    println!("H_K = {}", H_K);
    //

    let H_K_C = H_K + H_P - H_C;
    println!("H(K | C) = {}", H_K_C);
    let H_a_1 = pr_a * pr_1


}

fn main() {
    z2();
    println!("Hello, world!");
}
