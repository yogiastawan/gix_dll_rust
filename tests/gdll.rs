use gix_dll_rust::GixDLList;

#[test]
fn gdll_test() {
    let gdll = GixDLList::<i32>::new().expect("Cannot create gdll");
    let node_a = gdll.append(&10).expect("Cannot append");
    let node_b = gdll.prepend(&20).expect("Cannot prepend");
    let node_c = gdll.append(&30).expect("Cannot append");

    println!("{}", &gdll);
    // [20, 10, 30]
    assert_eq!(gdll.size(), 3);
    assert!(node_c.set_data(&25).is_ok());

    println!("{}", &gdll);
    // [20, 10, 25]
    let node_d = gdll
        .insert_after(&node_b, &50)
        .expect("Cannot insert_after");
    let node_e = gdll
        .insert_before(&node_a, &100)
        .expect("Cannot insert_before");
    assert_eq!(gdll.size(), 5);
    println!("{}", &gdll);
    // [20, 50, 100, 10, 25]

    assert!(gdll.remove(node_e).is_ok());
    assert_eq!(gdll.size(), 4);
    println!("{}", &gdll);
    // [20, 50, 10, 25]

    let a = gdll.get_data_at(2).expect("Cannot get value of index 2");
    assert_eq!(*a, 10);

    assert!(gdll.set_data_at(1, &1000).is_ok());
    assert_eq!(*gdll.get_data_at(1).unwrap(), 1000);
    println!("{}", &gdll);
    // [20, 1000, 10, 25]

    assert!(gdll.remove_at(1).is_ok()); //**REMOVE NODE_D**
    assert_eq!(gdll.size(), 3);
    println!("{}", gdll);
    // [20, 10, 25]

    /* below is resulting error because node_d ptr was free in C (**REMOVE NODE_D**). But node_d still contains the address that was
     * free (not set to NULL)*/
    assert!(node_d.set_data(&50).is_err());
    assert!(node_d.get_data().is_none());
    assert!(gdll.remove(node_d).is_err());
}
