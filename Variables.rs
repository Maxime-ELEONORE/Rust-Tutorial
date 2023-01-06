
fn main()
{

    let i = 0; //non mutable
    
    let mut j = 0;
    
    j = 1;

    let mut tab = Vec::new();

    tab.push(1);
    tab.push(2);
    tab.push(3);
    
    println!("i={}, j={}, tab={:?}", i, j, &tab);
}