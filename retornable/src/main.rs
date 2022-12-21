fn distr_exponencial(r1 : f64, media : f64) -> f64
{
    (-media) * (1f64 - r1).ln() 
}

fn distr_uniforme(r3:f64, a:f64, b:f64) -> f64
{
    a+((b-a) * r3)
}

fn distr_normal(r2:f64,r3:f64,media:f64,desviacion_estandar:f64) -> f64
{
    ((-2_f64 * (1f64 - r2).ln() ).sqrt() * (std::f64::consts::PI * 2_f64 * r3)) * desviacion_estandar + media 
}

fn distr_triangular(r1:f64 , r2:f64 , min:f64 , max:f64 , moda:f64) -> f64
{
    let condicion = (moda - min ) / (max - min);
    if r1 <= condicion 
    {
         min + (moda - min) * r2.sqrt()
    }else
    {
        max - ((max - moda) * (1f64 - r2 ).sqrt())
    }
}

fn main() {

    println!("Hello, world!");
    println!("VAL: {}", distr_exponencial(0.6085, 6f64));
    println!("VAL2: {}", distr_uniforme(0.3779_f64, 8f64, 15f64  ));
    println!("VAL3: {}", distr_normal(0.8544,0.0169,4.0,0.5));
    println!("VAL4: {}", distr_triangular(0.2612,0.6067,45_f64,55_f64, 50_f64));
}
