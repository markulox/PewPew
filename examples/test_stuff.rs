#[derive(Debug)]
struct Element{
    timestamp: i64,
    index: i64,
    content: String
}

impl Element {
    fn create(time:i64, idx: i64, content: String) -> Self {
        Element { timestamp:time, index: idx, content: content }
    }
}

struct TestMoveList {
    v_odd : Vec<Element>,
    v_eve : Vec<Element>,
    v_holder: Vec<Element>
}

impl TestMoveList {
    fn new() -> Self {
        TestMoveList {v_odd: Vec::new(), v_eve: Vec::new(), v_holder: Vec::new()}
    }

    fn gen_data(&mut self){
        for i in 0..102 {
            self.v_holder.push(Element::create(0,i, format!("{i}. Testing String")))
        }
    }

    fn split_data(&mut self) {
        for e_elm in self.v_holder.pop() {
            if e_elm.index % 2 == 0 {
                println!("Even!");
                self.v_eve.push(e_elm);
            } else {
                println!("Odd!");
                self.v_odd.push(e_elm);
            }
        }
        println!("Done moving data!");
    }
}

fn test_merge(mut a: Vec<Element>, mut b: Vec<Element>){
    let mut shoot_res_vec = a;
    let mut shoot_res_err_vec = b;

    let mut pos_shoot_res_vec: usize = shoot_res_vec.len() - 1;
    while let Some(err_shr) = shoot_res_err_vec.pop() {
        if pos_shoot_res_vec > shoot_res_vec.len() - 1 { // Same condition since we expect to jump to overflow
            shoot_res_vec.insert(0, err_shr);
            break;
        } else {
            // compare and insert
            while let Some(succ_shr) = shoot_res_vec.get(pos_shoot_res_vec) {
                if err_shr.timestamp > succ_shr.timestamp {
                    shoot_res_vec.insert(pos_shoot_res_vec + 1, err_shr);
                    break;
                } else {
                    pos_shoot_res_vec -= 1;
                }
            }
        }
    }
    for e in shoot_res_vec {
        println!("{:?}", e);
    }
    println!("Length of b after merged: {}", shoot_res_err_vec.len());
}

fn main() {
    // let mut a = TestMoveList::new();
    // a.gen_data();
    // a.split_data();

    let mut a = vec![
        Element::create(0, 1, format!("A")),
        Element::create(1, 1, format!("A")),
        Element::create(3, 1, format!("A")),
        Element::create(6, 1, format!("A")),
        Element::create(7, 1, format!("A")),
        Element::create(8, 1, format!("A")),
        Element::create(10, 1, format!("A")),
    ];

    let mut b = vec![
        Element::create(2, 1, format!("B")),
        Element::create(4, 1, format!("B")),
        Element::create(5, 1, format!("B")),
        Element::create(9, 1, format!("B")),
        Element::create(11, 1, format!("B")),
        Element::create(13, 1, format!("B")),
        Element::create(15, 1, format!("B")),
    ];

    test_merge(a, b);

}

