fn main() {
    ascii();
    println!("Collatz for n={} is {}", 12, collatz(12));
    println!("Armstrong n={} to {}",  153, armstrong(152));
    println!("Is {} perfect num {}",5, perfrct_num(5) );

}

fn ascii(){
    for i in 33..126 {
        // Print the ASCII character and its corresponding number
        println!("{}: {}", i, char::from(i));
        
    }
}
// Calculates the number of steps in the Collatz sequence for a given number
fn collatz(n:i64)->i64{
    let mut a=n;
    let mut i=0;
    while a!=1 {
        if a%2==0 {
            a=a/2;
            i+=1;
        }else {
            a=3*a+1;
            i+=1;
        }
    }
    i
}
// Checks whether a number is an Armstrong number
fn armstrong(n:i64)->bool{
    let mut a=n;
    let mut arr: Vec<_>=Vec::new();
    while a>0 {
        let tmp=a%10;
        arr.push(tmp);
        a/=10;
    }
    let mut h=0;
    let mut i=0;
    while h<n {
        h=0;
        for l in 0..arr.len() {
            h+=arr[l].pow(i);
        }
        if(h==n){
            return true
        }
        
        i+=1;
    }
    false
}
// Checks whether a number is a perfect number
fn perfrct_num(n:i64)->bool{
    let mut a=1;
    let mut res=0;

    while a<n{
        if n%a==0 {
            res+=a;
        }
        a+=1;
    }
    if res==n{
       return true
    }
    false

}
